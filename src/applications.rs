use sqlx::PgConnection;

#[derive(Debug, Clone)]
pub struct Applicant {
  pub id: uuid::Uuid ,
  pub name: String,
  pub assignment_id: i32,
  pub git_url: String
}

pub async fn application(conn: &mut PgConnection, id: uuid::Uuid) -> Result<Applicant, sqlx::Error> {
  Ok(sqlx::query_as!(
    Applicant,
    r#"
        SELECT * FROM applications WHERE id = $1
    "#,
    id
  )
  .fetch_one(conn)
  .await?)
}

pub async fn applications(conn: &mut PgConnection) -> Result<Vec<Applicant>, sqlx::Error> {
  Ok(sqlx::query_as!(
    Applicant,
    r#"
        SELECT * FROM applications
    "#
  )
  .fetch_all(conn)
  .await?)
}
