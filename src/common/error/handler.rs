use super::{biz_code::BizCode, biz_error::BizError};
use crate::model::result::base::BizResult;
use axum::{response::IntoResponse, Json};

pub async fn biz_error_handler(err: BizError) -> impl IntoResponse {
    match err {
        BizError::Ok => Json(BizResult::ok(())),
        BizError::Error => Json(BizResult::<()>::from(BizCode::ERROR)),
        BizError::TokenMissing => Json(BizResult::<()>::from(BizCode::TOKEN_MISSING)),
        BizError::TokenIvalid => Json(BizResult::<()>::from(BizCode::TOKEN_INVALID)),
        BizError::ParamMissing => Json(BizResult::<()>::from(BizCode::PARAM_MISSING)),
        BizError::MemberNotExist => Json(BizResult::<()>::from(BizCode::MEMBER_NOT_EXIST)),
        BizError::EmailPasswordIncorrect => {
            Json(BizResult::<()>::from(BizCode::EMAIL_PASSWORD_INCORRECT))
        }
    }
    .into_response()
}
