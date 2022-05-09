use sqlx::PgConnection;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: i32,
    pub name: String,
    pub role_name: String,
    pub description: String,
    pub git_url: String,
}

pub async fn assignment(conn: &mut PgConnection, id: i32) -> Result<Assignment, sqlx::Error> {
    Ok(sqlx::query_as!(
        Assignment,
        r#"
        SELECT * FROM assignments WHERE id = $1
    "#,
        id
    )
    .fetch_one(conn)
    .await?)
}

pub async fn assignments(conn: &mut PgConnection) -> Result<Vec<Assignment>, sqlx::Error> {
    Ok(sqlx::query_as!(
        Assignment,
        r#"
        SELECT * FROM assignments
    "#
    )
    .fetch_all(conn)
    .await?)
}
