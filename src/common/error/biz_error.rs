use super::biz_code::BizCode;
use crate::model::result::base::BizResult;
use axum::{
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, Clone, Copy)]
pub enum BizError {
    Ok,
    Error,
    TokenMissing,
    TokenIvalid,
    ParamMissing,
    MemberNotExist,
    EmailPasswordIncorrect,
}

impl IntoResponse for BizError {
    fn into_response(self) -> Response {
        let reply = match self {
            Self::Ok => Json(BizResult::<()>::from(BizCode::OK)),
            Self::Error => Json(BizResult::<()>::from(BizCode::ERROR)),
            Self::TokenMissing => Json(BizResult::<()>::from(BizCode::TOKEN_MISSING)),
            Self::TokenIvalid => Json(BizResult::<()>::from(BizCode::TOKEN_INVALID)),
            Self::ParamMissing => Json(BizResult::<()>::from(BizCode::PARAM_MISSING)),
            Self::MemberNotExist => Json(BizResult::<()>::from(BizCode::MEMBER_NOT_EXIST)),
            Self::EmailPasswordIncorrect => {
                Json(BizResult::<()>::from(BizCode::EMAIL_PASSWORD_INCORRECT))
            }
        };
        reply.into_response()
    }
}
