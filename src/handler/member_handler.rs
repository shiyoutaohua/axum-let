use crate::common::error::biz_error::BizError;
use crate::model::dto::member::MemberSession;
use crate::model::param::member::{
    MemberDeleteParam, MemberDetailParam, MemberExistParam, MemberLoginParam,
};
use crate::model::result::base::BizResult;
use crate::model::result::member::MemberLoginSuccessResult;
use crate::repository::member_repository;
use axum::response::IntoResponse;
use axum::{extract::Query, Extension, Json};
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::Duration;
use redis::AsyncCommands;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn count(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let count = member_repository::count(db_pool).await;
    Json(BizResult::ok(count))
}

pub async fn all(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    let vec = member_repository::find_all(db_pool).await;
    Json(BizResult::ok(vec))
}

pub async fn exist_by_id(
    Extension(db_pool): Extension<MySqlPool>,
    Query(param): Query<MemberExistParam>,
) -> impl IntoResponse {
    let exist = match param.member_id {
        Some(member_id) => member_repository::exist_by_id(db_pool, member_id).await,
        None => false,
    };
    Json(BizResult::ok(exist))
}

pub async fn detail_by_id(
    Extension(db_pool): Extension<MySqlPool>,
    Query(param): Query<MemberDetailParam>,
) -> impl IntoResponse {
    let member_to = match param.member_id {
        Some(member_id) => member_repository::find_by_id(db_pool, member_id).await,
        None => None,
    };
    Json(BizResult::ok(member_to))
}

pub async fn delete_by_id(
    Extension(db_pool): Extension<MySqlPool>,
    Query(param): Query<MemberDeleteParam>,
) -> impl IntoResponse {
    let ok = match param.member_id {
        Some(member_id) => member_repository::delete_by_id(db_pool, member_id).await,
        None => false,
    };
    Json(BizResult::ok(ok))
}

pub async fn truncate(Extension(db_pool): Extension<MySqlPool>) -> impl IntoResponse {
    member_repository::truncate(db_pool).await;
    Json(BizResult::ok(()))
}

pub async fn login(
    Extension(db_pool): Extension<MySqlPool>,
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
    Json(param): Json<MemberLoginParam>,
) -> Result<impl IntoResponse, BizError> {
    let member_to = member_repository::find_by_email(db_pool, &param.email).await;
    if let Some(member_to) = member_to {
        if Some(param.login_password) == member_to.login_password {
            let rand_factor = Uuid::new_v4().as_simple().to_string();
            let token = format!("token_{}_{}", member_to.member_id.unwrap(), rand_factor);
            let member_session = MemberSession {
                member_id: member_to.member_id,
                member_name: member_to.member_name,
            };
            let member_session = serde_json::to_string(&member_session).unwrap_or("".into());
            let mut conn = redis_pool.get().await.unwrap();
            let _: () = conn
                .set_ex(
                    token.clone(),
                    member_session,
                    Duration::hours(1).num_seconds() as usize,
                )
                .await
                .unwrap();
            return Ok(Json(BizResult::ok(MemberLoginSuccessResult {
                token: token.to_string(),
            })));
        }
    }
    Err(BizError::EmailPasswordIncorrect)
}
