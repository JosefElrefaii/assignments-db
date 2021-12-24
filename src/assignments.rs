use sqlx::PgConnection;

#[derive(Debug, Clone)]
pub struct Assignment {
  id: i32,
  name: String
}

pub async fn assignments(conn: &mut PgConnection) -> Result<Vec<Assignment>, sqlx::Error> {
  Ok(sqlx::query_as!(
    Assignment,
    r#"
        SELECT * FROM regions
    "#
  )
  .fetch_all(conn)
  .await?)
}
