use csv::Reader;
use nanoid::nanoid;
use serde::Deserialize;
use sqlx::PgPool;
use rust::db::init::init_db;
use anyhow::Result;
use sqlx::types::chrono::{DateTime as SQLxDateTime, Utc as SQLxUtc};

#[derive(Debug, Deserialize)]
struct OldTask {
   id: String,
   title: String, 
   content: String,
   priority: i16,
   completed: bool,
   created_at: SQLxDateTime<SQLxUtc>,
   updated_at: SQLxDateTime<SQLxUtc>,
   due_date: Option<SQLxDateTime<SQLxUtc>>,
   file: Option<String>,
   assignee: String
}

async fn migrate_tasks(
   pool: &PgPool,
   csv_path: &str,
   default_created_by: &str,
   default_assignee: &str
) -> Result<()> {
   let mut rdr = Reader::from_path(csv_path)?;
   
   for result in rdr.deserialize() {
       let old_task: OldTask = result?;
       let new_id = nanoid!();
       
       sqlx::query(r#"
           INSERT INTO tasks (
               id, title, markdown, priority, completed,
               created_at, updated_at, due_date, file_path,
               created_by, assignee
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
       "#)
       .bind(&new_id)
       .bind(&old_task.title)
       .bind(&old_task.content)
       .bind(&old_task.priority)
       .bind(&old_task.completed)
       .bind(&old_task.created_at)
       .bind(&old_task.updated_at)
       .bind(&old_task.due_date)
       .bind("")
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
    
    migrate_tasks(
        &pool,
        "task_old.csv",
        "9aJrNaoF9ftHcB0DtW5FX",
        "CnhaRk11dO3pAM5iZ4B1P"
    ).await?;

    println!("Migration completed!");
    Ok(())
}