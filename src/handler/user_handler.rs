use std::time::Duration;

use crate::common::error::biz_error::BizError;
use crate::common::extractor::caller::Caller;
use crate::common::extractor::visitor::Visitor;
use crate::common::util::id::UuidGenerator;
use crate::model::dto::user::UserSession;
use crate::model::param::user::UserLoginParam;
use crate::model::result::user::{UserDetailResult, UserLoginResult};
use crate::model::{param::user::UserListParam, result::base::BizResult};
use crate::repository::user_repository;
use axum::{response::IntoResponse, Extension, Json};
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn all(
    _caller: Caller,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let vec: Vec<UserDetailResult> = user_repository::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .iter()
        .map(|el| UserDetailResult::from(el))
        .collect();
    BizResult::ok(vec)
}

pub async fn list_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Json(param): Json<UserListParam>,
) -> impl IntoResponse {
    let vec: Vec<UserDetailResult> = user_repository::Entity::find()
        .filter(user_repository::Column::UserId.is_in(param.user_ids.unwrap()))
        .all(&db)
        .await
        .unwrap()
        .iter()
        .map(|el| UserDetailResult::from(el))
        .collect();
    BizResult::ok(vec)
}

pub async fn detail_by_id(
    visitor: Visitor,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, BizError> {
    let model_opt = user_repository::Entity::find_by_id(visitor.user_id)
        .one(&db)
        .await
        .unwrap();
    if let Some(user) = model_opt {
        Ok(BizResult::ok(Some(UserDetailResult::from(user))))
    } else {
        Err(BizError::USER_NOT_EXIST)
    }
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
    Json(param): Json<UserLoginParam>,
) -> Result<impl IntoResponse, BizError> {
    let model_opt = user_repository::find_by_email(&db, &param.email).await;
    if let Some(user) = model_opt {
        let uuid = UuidGenerator::next_v7();
        let token = format!("token_{}_{}", user.user_id, uuid);
        let user_session = UserSession {
            user_id: user.user_id,
            nickname: Some(user.nickname),
        };
        let user_session = serde_json::to_string(&user_session).unwrap_or("".into());
        let mut redis_conn = redis_pool.get().await.unwrap();
        let _: () = redis_conn
            .set_ex(
                token.clone(),
                user_session,
                Duration::from_secs(1 * 60 * 60)
                    .as_secs()
                    .try_into()
                    .unwrap(),
            )
            .await
            .unwrap();
        Ok(BizResult::ok(UserLoginResult {
            token: token.to_string(),
        }))
    } else {
        Err(BizError::EMAIL_PASSWORD_INCORRECT)
    }
}
