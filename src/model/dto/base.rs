use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDto {
    pub page_size: usize,
    pub page_num: usize,
    pub total: usize,
}
