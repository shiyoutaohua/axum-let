use crate::model::result::base::BizResult;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BizError {
    pub code: i32,
    pub msg: &'static str,
}

impl BizError {
    pub const OK: BizError = BizError { code: 0, msg: "ok" };

    pub const ERROR: BizError = BizError {
        code: 1,
        msg: "unknown error",
    };

    pub const UNAUTHORIZED: BizError = BizError {
        code: 401,
        msg: "unauthorized access",
    };

    pub const TOKEN_MISSING: BizError = BizError {
        code: 10000,
        msg: "token missing",
    };

    pub const TOKEN_INVALID: BizError = BizError {
        code: 10001,
        msg: "token invalid",
    };

    pub const PARAM_MISSING: BizError = BizError {
        code: 10002,
        msg: "param missing",
    };

    pub const USER_NOT_EXIST: BizError = BizError {
        code: 10003,
        msg: "user not exist",
    };

    pub const EMAIL_PASSWORD_INCORRECT: BizError = BizError {
        code: 10004,
        msg: "email or password incorrect",
    };

    pub const APIKEY_MISSING: BizError = BizError {
        code: 1_000_000_000,
        msg: "apikey missing",
    };

    pub const APIKEY_INVALID: BizError = BizError {
        code: 1_000_000_001,
        msg: "apikey invalid",
    };
}

impl IntoResponse for BizError {
    fn into_response(self) -> axum::response::Response {
        BizResult::<()>::from(self).into_response()
    }
}
