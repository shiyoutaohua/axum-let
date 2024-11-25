use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
    pub deleted_at: Option<TimeDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Option<Model> {
    Entity::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await
        .unwrap_or_default()
}
