use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::Path;
use axum::{extract::State, Extension, Json};

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use hyper::StatusCode;

use crate::chat::events::Event;
use crate::chat::server::Command;
use crate::models::channel::{Channel, ChannelUser};
use crate::schema::{channels_users, users};
use crate::views::channel::ChannelDetails;
use crate::{
    models::{channel::NewChannel, user::User},
    state::AppState,
};

use super::AppError;

pub async fn get_channel(
    State(state): State<Arc<AppState>>,
    Path(channel_id): Path<i32>,
) -> Result<Json<ChannelDetails>, AppError> {
    use crate::schema::channels::dsl::*;

    let mut conn = state.db_pool.get().await?;

    let channel = match channels
        .filter(id.eq(channel_id))
        .select(ChannelDetails::as_select())
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

    Ok(Json(channel))
}

pub async fn create_channel(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(mut channel): Json<NewChannel>,
) -> Result<Json<ChannelDetails>, AppError> {
    use crate::schema::channels::dsl::*;
    use crate::schema::channels_users::dsl::*;

    channel.owner_id = Some(user.id);

    let mut conn = state.db_pool.get().await?;

    let channel = conn
        .transaction::<ChannelDetails, diesel::result::Error, _>(|conn| {
            async move {
                let channel = diesel::insert_into(channels)
                    .values(channel)
                    .returning(ChannelDetails::as_returning())
                    .get_result(conn)
                    .await?;

                diesel::insert_into(channels_users)
                    .values((channel_id.eq(channel.id), user_id.eq(user.id)))
                    .execute(conn)
                    .await?;

                Ok(channel)
            }
            .scope_boxed()
        })
        .await?;

    let tx = state.chat_server.get_manager_tx();

    tx.send(Command::Send {
        destination: user.clone(),
        message: serde_json::to_string(&Event::channel_created(&channel))?,
    })
    .await
    .map_err(|err| AppError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error: anyhow!(err.to_string()),
    })?;

    Ok(Json(channel))
}

pub async fn delete_channel(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<i32>,
) -> Result<(), AppError> {
    use crate::schema::channels::dsl::*;

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

    if channel.owner_id != user.id {
        return Err(AppError {
            status_code: StatusCode::FORBIDDEN,
            error: anyhow!("you don't own this channel"),
        });
    }

    let members = ChannelUser::belonging_to(&channel)
        .inner_join(users::table)
        .select(User::as_select())
        .load(&mut conn)
        .await?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            diesel::delete(channels_users::table.filter(channels_users::channel_id.eq(channel_id)))
                .execute(conn)
                .await?;

            diesel::delete(channels.filter(id.eq(channel_id)))
                .execute(conn)
                .await?;

            Ok(())
        }
        .scope_boxed()
    })
    .await?;

    let tx = state.chat_server.get_manager_tx();
    let channel_details: Arc<ChannelDetails> = Arc::new(channel.clone().into());

    for member in members {
        tx.send(Command::Send {
            destination: member,
            message: serde_json::to_string(&Event::channel_deleted(&channel_details.clone()))?,
        })
        .await
        .map_err(|err| AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: anyhow!(err.to_string()),
        })?;
    }

    Ok(())
}
