use crate::common::error::biz_code::BizCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BizResult<T> {
    pub code: i32,
    pub msg: String,
    pub tip: String,
    pub data: Option<T>,
}

impl<T> BizResult<T> {
    pub fn ok(data: T) -> Self {
        BizResult {
            code: 0,
            msg: String::from("ok"),
            tip: String::from("ok"),
            data: Some(data),
        }
    }
}

impl From<BizCode> for BizResult<()> {
    fn from(value: BizCode) -> Self {
        BizResult {
            code: value.code,
            msg: value.msg.to_string(),
            tip: value.tip.to_string(),
            data: None,
        }
    }
}
