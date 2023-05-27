#[derive(Debug, Clone, Copy)]
pub struct BizCode {
    pub code: i32,
    pub msg: &'static str,
    pub tip: &'static str,
}

impl BizCode {
    pub const OK: BizCode = BizCode {
        code: 0,
        msg: "ok",
        tip: "ok",
    };

    pub const ERROR: BizCode = BizCode {
        code: 1,
        msg: "unknown error",
        tip: "unknown error",
    };

    pub const UNAUTHORIZED: BizCode = BizCode {
        code: 401,
        msg: "unauthorized access",
        tip: "unauthorized access",
    };

    pub const TOKEN_MISSING: BizCode = BizCode {
        code: 10000,
        msg: "token missing",
        tip: "token missing",
    };

    pub const TOKEN_INVALID: BizCode = BizCode {
        code: 10001,
        msg: "token invalid",
        tip: "token invalid",
    };

    pub const PARAM_MISSING: BizCode = BizCode {
        code: 10001,
        msg: "token invalid",
        tip: "token invalid",
    };

    pub const MEMBER_NOT_EXIST: BizCode = BizCode {
        code: 10001,
        msg: "member not exist",
        tip: "member not exist",
    };

    pub const EMAIL_PASSWORD_INCORRECT: BizCode = BizCode {
        code: 10001,
        msg: "email or password incorrect",
        tip: "email or password incorrect",
    };
}
