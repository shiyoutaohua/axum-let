use crate::model::to::member::MemberTo;
use sqlx::MySqlPool;

pub async fn count(cp: MySqlPool) -> usize {
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM member")
        .fetch_one(&cp)
        .await
        .unwrap();
    row.0 as usize
}

pub async fn exist_by_id(cp: MySqlPool, member_id: u64) -> bool {
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM member WHERE member_id = ?")
        .bind(member_id)
        .fetch_one(&cp)
        .await
        .unwrap();
    row.0 > 0
}

pub async fn find_by_id(cp: MySqlPool, member_id: u64) -> Option<MemberTo> {
    sqlx::query_as::<_, MemberTo>("SELECT * FROM member WHERE member_id = ?")
        .bind(member_id)
        .fetch_optional(&cp)
        .await
        .unwrap()
}

pub async fn find_by_email(cp: MySqlPool, email: &str) -> Option<MemberTo> {
    sqlx::query_as::<_, MemberTo>("SELECT * FROM member WHERE email = ?")
        .bind(email)
        .fetch_optional(&cp)
        .await
        .unwrap()
}

pub async fn find_all(cp: MySqlPool) -> Vec<MemberTo> {
    sqlx::query_as::<_, MemberTo>("SELECT * FROM member")
        .fetch_all(&cp)
        .await
        .unwrap()
}

pub async fn delete_by_id(cp: MySqlPool, member_id: u64) -> bool {
    let count = sqlx::query("DELETE FROM member WHERE member_id = ?")
        .bind(member_id)
        .execute(&cp)
        .await
        .unwrap()
        .rows_affected();
    count == 1
}

pub async fn truncate(cp: MySqlPool) {
    let _ = sqlx::query("TRUNCATE TABLE member")
        .execute(&cp)
        .await
        .unwrap();
}
