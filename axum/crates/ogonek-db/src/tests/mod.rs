use sqlx::PgPool;
#[allow(dead_code)]
// Helper function to create a test user
pub async fn create_test_user(db: &PgPool, username: &str, email: &str) -> String {
    let user_id = nanoid::nanoid!();
    sqlx::query!(
        r#"
        INSERT INTO "user" (id, username, email, name, pass, role, verified)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user_id,
        username,
        email,
        "Test Name",
        "hashed_password",
        "student",
        false
    )
    .execute(db)
    .await
    .expect("Failed to create test user");

    user_id
}
#[allow(dead_code)]
// Helper function to clean up test user
pub async fn cleanup_user(db: &PgPool, user_id: &str) {
    let _ = sqlx::query!(r#"DELETE FROM "user" WHERE id = $1"#, user_id)
        .execute(db)
        .await;
}
