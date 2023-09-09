use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Extension, Json,
};

use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use hyper::StatusCode;

use crate::models::channel::{Channel, ChannelUser};
use crate::schema::{channels_users, users};
use crate::views::channel::ChannelDetails;
use crate::{chat::events::ServerEvent, views::user::UserDetails};
use crate::{chat::server::Command, views::channel::ChannelDetailsWithUser};
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

    state
        .chat_server
        .send_command(Command::Send {
            destination: user.clone(),
            message: ServerEvent::channel_created(&channel, vec![user.clone().into()]),
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

    let event = ServerEvent::channel_deleted(&channel.into());

    for member in members {
        state
            .chat_server
            .send_command(Command::Send {
                destination: member,
                message: event.clone(),
            })
            .await
            .map_err(|err| AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: anyhow!(err.to_string()),
            })?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct ChannelInvite {
    username: String,
}

pub async fn invite_user(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<i32>,
    Json(invite): Json<ChannelInvite>,
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

    let target_user = match users::table
        .filter(users::username.eq(invite.username))
        .select(User::as_select())
        .get_result(&mut conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::NOT_FOUND,
                error: anyhow!("User with this username was not found"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    diesel::insert_into(channels_users::table)
        .values((
            channels_users::channel_id.eq(channel_id),
            channels_users::user_id.eq(target_user.id),
        ))
        .execute(&mut conn)
        .await?;

    let members = ChannelUser::belonging_to(&channel)
        .inner_join(users::table)
        .select(UserDetails::as_select())
        .load(&mut conn)
        .await?;

    for member in members.clone() {
        if member.id == target_user.id {
            continue;
        }

        state
            .chat_server
            .send_command(Command::Send {
                destination: User::from(member),
                message: ServerEvent::UserJoined {
                    channel: ChannelDetailsWithUser::new(channel.clone().into(), members.clone()),
                    user: target_user.clone().into(),
                },
            })
            .await?;
    }

    state
        .chat_server
        .send_command(Command::Send {
            destination: target_user.clone(),
            message: ServerEvent::channel_created(&channel.clone().into(), members),
        })
        .await?;

    Ok(())
}
