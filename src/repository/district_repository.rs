use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "district")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub district_id: i64,
    pub code: String,
    pub name: String,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
    pub deleted_at: Option<TimeDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
