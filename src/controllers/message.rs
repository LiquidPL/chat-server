use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use hyper::StatusCode;

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

use crate::{
    chat::events::Event,
    chat::server::Command,
    models::{
        channel::{Channel, ChannelUser},
        message::NewMessage,
        user::User,
    },
    schema::{channels, messages, users},
    state::AppState,
    views::message::MessageDetails,
};

use super::AppError;

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<i32>,
    Json(mut message): Json<NewMessage>,
) -> Result<Json<MessageDetails>, AppError> {
    use crate::schema::channels::dsl::*;

    message.sender_id = Some(user.id);
    message.channel_id = Some(channel_id);

    let mut conn = state.db_pool.get().await?;

    let channel = match channels
        .filter(id.eq(channel_id))
        .select(Channel::as_select())
        .get_result(&mut conn)
        .await
    {
        Ok(channel) => Ok(channel),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::NOT_FOUND,
                error: anyhow!("not found"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    match ChannelUser::belonging_to(&channel)
        .inner_join(users::table)
        .filter(users::id.eq(user.id))
        .select(User::as_select())
        .get_result(&mut conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::FORBIDDEN,
                error: anyhow!("you don't have access to this channel"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    let message = diesel::insert_into(messages::table)
        .values(message)
        .returning(MessageDetails::as_returning())
        .get_result(&mut conn)
        .await?;

    let members = ChannelUser::belonging_to(&channel)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await?;

    let tx = state.chat_server.get_manager_tx();
    let message_created = Arc::new(Event::message_created(&message));

    for member in members {
        tx.send(Command::Send {
            destination: member,
            message: serde_json::to_string(&message_created)?,
        })
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: anyhow!(err.to_string()),
        })?;
    }

    Ok(Json(message))
}

pub async fn delete_message(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path((channel_id, message_id)): Path<(i32, i32)>,
) -> Result<(), AppError> {
    let mut conn = state.db_pool.get().await?;

    let channel = match channels::table
        .filter(channels::id.eq(channel_id))
        .select(Channel::as_select())
        .get_result(&mut conn)
        .await
    {
        Ok(channel) => Ok(channel),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::FORBIDDEN,
                error: anyhow!("you don't have access to this channel"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    let message = match messages::table
        .filter(messages::id.eq(message_id))
        .select(MessageDetails::as_select())
        .get_result(&mut conn)
        .await
    {
        Ok(message) => Ok(message),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::NOT_FOUND,
                error: anyhow!("not found"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    if message.sender_id != user.id {
        return Err(AppError {
            status_code: StatusCode::FORBIDDEN,
            error: anyhow!("you don't own this message"),
        });
    }

    diesel::delete(messages::table.filter(messages::id.eq(message_id)))
        .execute(&mut conn)
        .await?;

    let members = ChannelUser::belonging_to(&channel)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await?;

    let tx = state.chat_server.get_manager_tx();
    let message_details: Arc<MessageDetails> = Arc::new(message);

    for member in members {
        tx.send(Command::Send {
            destination: member,
            message: serde_json::to_string(&Event::message_deleted(&message_details.clone()))?,
        })
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: anyhow!(err.to_string()),
        })?;
    }

    Ok(())
}
