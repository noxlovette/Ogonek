use csv::Reader;
use nanoid::nanoid;
use serde::Deserialize;
use sqlx::PgPool;
use rust::db::init::init_db;
use anyhow::Result;
use sqlx::types::chrono::{DateTime as SQLxDateTime, Utc as SQLxUtc};

#[derive(Debug, Deserialize)]
struct OldLesson {
    id: String,  // old UUID (we won't use this)
    title: String,
    content: String,
    created_at: SQLxDateTime<SQLxUtc>,
    updated_at: SQLxDateTime<SQLxUtc>,
    category: String, // we'll skip this
    topic: String,
    assignee: String, // old UUID (you'll provide new ID manually)
}

async fn migrate_lessons(
    pool: &PgPool, 
    csv_path: &str,
    default_created_by: &str,  // new nanoid of the default creator
    default_assignee: &str     // new nanoid of the default assignee
) -> Result<()> {
    let mut rdr = Reader::from_path(csv_path)?;
    
    for result in rdr.deserialize() {
        let old_lesson: OldLesson = result?;
        let new_id = nanoid!();
        
        sqlx::query(r#"
            INSERT INTO lessons (
                id, title, markdown, topic, 
                created_at, updated_at, 
                created_by, assignee
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#)
        .bind(&new_id)
        .bind(&old_lesson.title)
        .bind(&old_lesson.content)
        .bind(&old_lesson.topic)
        .bind(&old_lesson.created_at)
        .bind(&old_lesson.updated_at)
        .bind(default_created_by)
        .bind(default_assignee)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = init_db().await?;
    
    migrate_lessons(
        &pool,
        "lesson_old.csv",
        "9aJrNaoF9ftHcB0DtW5FX",
        "CnhaRk11dO3pAM5iZ4B1P"
    ).await?;

    println!("Migration completed!");
    Ok(())
}