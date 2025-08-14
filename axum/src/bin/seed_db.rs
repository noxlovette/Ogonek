// src/bin/seed_db.rs
use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use ogonek::{auth::password::hash_password, db::init::init_db, types::ProfileUpdate};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db = init_db().await?;

    println!("🌱 Starting database seeding...");

    // Clear existing data (optional - remove if you want to keep existing data)
    clear_dev_data(&db).await?;

    // Create users
    let users = create_users(&db).await?;

    // Create profiles for users
    create_profiles(&db, &users).await?;

    // Create teacher-student relationships
    create_teacher_student_relationships(&db, &users).await?;

    // Create lessons
    create_lessons(&db, &users).await?;

    // Create tasks
    create_tasks(&db, &users).await?;

    // Create decks
    create_decks(&db, &users).await?;

    println!("✅ Database seeding completed!");

    Ok(())
}

async fn clear_dev_data(db: &PgPool) -> Result<()> {
    println!("🧹 Clearing existing dev data...");

    // Order matters due to foreign key constraints
    sqlx::query!("DELETE FROM decks WHERE created_by LIKE 'dev_%'")
        .execute(db)
        .await?;
    sqlx::query!("DELETE FROM tasks WHERE created_by LIKE 'dev_%'")
        .execute(db)
        .await?;
    sqlx::query!("DELETE FROM lessons WHERE created_by LIKE 'dev_%'")
        .execute(db)
        .await?;
    sqlx::query!(
        "DELETE FROM teacher_student WHERE teacher_id LIKE 'dev_%' OR student_id LIKE 'dev_%'"
    )
    .execute(db)
    .await?;
    sqlx::query!("DELETE FROM profile WHERE user_id LIKE 'dev_%'")
        .execute(db)
        .await?;
    sqlx::query!("DELETE FROM \"user\" WHERE id LIKE 'dev_%'")
        .execute(db)
        .await?;

    Ok(())
}

async fn create_users(db: &PgPool) -> Result<Vec<String>> {
    println!("👥 Creating dev users...");

    let dev_password = "!7!N6x$#62j0fE3zdGnS";
    let hashed_password = hash_password(dev_password)?;

    let users = vec![
        (
            "dev_teacher1",
            "Alice Teacher",
            "alice@dev.ogonek.app",
            "teacher",
        ),
        (
            "dev_teacher2",
            "Bob Instructor",
            "bob@dev.ogonek.app",
            "teacher",
        ),
        (
            "dev_student1",
            "Charlie Student",
            "charlie@dev.ogonek.app",
            "student",
        ),
        (
            "dev_student2",
            "Diana Learner",
            "diana@dev.ogonek.app",
            "student",
        ),
        ("dev_student3", "Eve Pupil", "eve@dev.ogonek.app", "student"),
    ];

    let mut user_ids = Vec::new();

    for (username, name, email, role) in users {
        let user_id = format!("{}_{}", username, nanoid::nanoid!(8));

        sqlx::query!(
            r#"
            INSERT INTO "user" (id, username, name, email, pass, role, verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user_id,
            username,
            name,
            email,
            hashed_password,
            role,
            false
        )
        .execute(db)
        .await?;

        user_ids.push(user_id);
    }

    println!("  🔑 All users created with password: {dev_password}");

    Ok(user_ids)
}

async fn create_profiles(db: &PgPool, user_ids: &[String]) -> Result<()> {
    println!("📋 Updating profiles...");

    let profiles = vec![
        ProfileUpdate {
            video_call_url: Some("https://zoom.us/j/alice123".to_string()),
            avatar_url: Some("https://avatars.dev/alice.png".to_string()),
            telegram_id: Some("900828558".to_string()),
        },
        ProfileUpdate {
            video_call_url: Some("https://meet.google.com/bob-room".to_string()),
            avatar_url: Some("https://avatars.dev/bob.png".to_string()),
            telegram_id: Some("900828558".to_string()),
        },
        ProfileUpdate {
            video_call_url: None,
            avatar_url: Some("https://avatars.dev/charlie.png".to_string()),
            telegram_id: Some("900828558".to_string()),
        },
        ProfileUpdate {
            video_call_url: None,
            avatar_url: Some("https://avatars.dev/diana.png".to_string()),
            telegram_id: Some("900828558".to_string()),
        },
        ProfileUpdate {
            video_call_url: None,
            avatar_url: Some("https://avatars.dev/eve.png".to_string()),
            telegram_id: Some("900828558".to_string()),
        },
    ];

    for (i, user_id) in user_ids.iter().enumerate() {
        let profile = &profiles[i];

        sqlx::query!(
            r#"
            UPDATE profile 
            SET video_call_url = $2, avatar_url = $3, telegram_id = $4
            WHERE user_id = $1
            "#,
            user_id,
            profile.video_call_url,
            profile.avatar_url,
            profile.telegram_id
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

async fn create_teacher_student_relationships(db: &PgPool, user_ids: &[String]) -> Result<()> {
    println!("🔗 Creating teacher-student relationships...");

    let teacher1 = &user_ids[0]; // Alice
    let teacher2 = &user_ids[1]; // Bob
    let student1 = &user_ids[2]; // Charlie
    let student2 = &user_ids[3]; // Diana
    let student3 = &user_ids[4]; // Eve

    let relationships = vec![
        (
            teacher1,
            student1,
            "Charlie is progressing well in Rust fundamentals.",
        ),
        (
            teacher1,
            student2,
            "Diana shows excellent problem-solving skills.",
        ),
        (
            teacher2,
            student2,
            "Working on advanced concepts with Diana.",
        ),
        (teacher2, student3, "Eve is new but very motivated."),
    ];

    for (teacher_id, student_id, markdown) in relationships {
        sqlx::query!(
            r#"
            INSERT INTO teacher_student (teacher_id, student_id, status, markdown, student_telegram_id)
            VALUES ($1, $2, 'active', $3, '900828558')
            "#,
            teacher_id,
            student_id,
            markdown
        )
        .execute(db)
        .await?;
    }

    println!("  ✓ Created 5 teacher-student relationships");
    Ok(())
}

async fn create_lessons(db: &PgPool, user_ids: &[String]) -> Result<()> {
    println!("📚 Creating lessons...");

    let teacher1 = &user_ids[0];
    let teacher2 = &user_ids[1];
    let student1 = &user_ids[2];
    let student2 = &user_ids[3];
    let student3 = &user_ids[4];

    let lessons = vec![
        (
            teacher1,
            student1,
            "Rust Basics",
            "Ownership and Borrowing",
            "# Ownership in Rust\n\nOwnership is Rust's most unique feature...\n\n## Rules\n1. Each value has a single owner\n2. When owner goes out of scope, value is dropped",
        ),
        (
            teacher1,
            student2,
            "Advanced Rust",
            "Async Programming",
            "# Async/Await in Rust\n\nAsynchronous programming in Rust uses futures...\n\n```rust\nasync fn fetch_data() -> Result<String, Error> {\n    // async code here\n}\n```",
        ),
        (
            teacher2,
            student2,
            "Web Development",
            "Axum Framework",
            "# Building APIs with Axum\n\nAxum is a modern web framework for Rust...\n\n## Key Features\n- Type-safe routing\n- Extractors\n- Middleware support",
        ),
        (
            teacher2,
            student3,
            "Database Design",
            "PostgreSQL Fundamentals",
            "# PostgreSQL Basics\n\nRelational databases and SQL...\n\n## Tables\n- Primary keys\n- Foreign keys\n- Constraints",
        ),
    ];

    for (created_by, assignee, title, topic, markdown) in lessons {
        let lesson_id = nanoid::nanoid!();

        sqlx::query!(
            r#"
            INSERT INTO lessons (id, title, topic, markdown, created_by, assignee)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            lesson_id,
            title,
            topic,
            markdown,
            created_by,
            assignee
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

async fn create_tasks(db: &PgPool, user_ids: &[String]) -> Result<()> {
    println!("✅ Creating tasks...");

    let teacher1 = &user_ids[0];
    let teacher2 = &user_ids[1];
    let student1 = &user_ids[2];
    let student2 = &user_ids[3];
    let student3 = &user_ids[4];

    let now = Utc::now();

    let tasks = vec![
        (
            teacher1,
            student1,
            "Complete Ownership Exercise",
            "# Ownership Exercise\n\nImplement a simple string manipulation function that demonstrates ownership transfer.\n\n## Requirements\n- Take ownership of a String\n- Return modified String\n- No cloning allowed",
            false,
            Some(now + chrono::Duration::days(7)),
        ),
        (
            teacher1,
            student2,
            "Build Async Web Scraper",
            "# Web Scraper Project\n\nCreate an async web scraper using reqwest and tokio.\n\n## Features\n- Concurrent requests\n- Error handling\n- Rate limiting",
            false,
            Some(now + chrono::Duration::days(14)),
        ),
        (
            teacher2,
            student2,
            "Design Database Schema",
            "# E-commerce Database\n\nDesign a database schema for an e-commerce platform.\n\n## Tables Needed\n- Users\n- Products\n- Orders\n- Order Items",
            true,
            None,
        ),
        (
            teacher2,
            student3,
            "SQL Practice Queries",
            "# SQL Exercises\n\nComplete the following SQL queries:\n\n1. Join tables\n2. Aggregate functions\n3. Subqueries\n4. Window functions",
            false,
            Some(now + chrono::Duration::days(3)),
        ),
    ];

    for (created_by, assignee, title, markdown, completed, due_date) in tasks {
        let task_id = nanoid::nanoid!();

        sqlx::query!(
            r#"
            INSERT INTO tasks (id, title, markdown, created_by, assignee, completed, due_date, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            "#,
            task_id,
            title,
            markdown,
            created_by,
            assignee,
            completed,
            due_date,
            now
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

async fn create_decks(db: &PgPool, user_ids: &[String]) -> Result<()> {
    println!("🃏 Creating decks...");

    let teacher1 = &user_ids[0];
    let teacher2 = &user_ids[1];
    let student1 = &user_ids[2];
    let student2 = &user_ids[3];
    let student3 = &user_ids[4];

    let decks = vec![
        (
            teacher1,
            student1,
            "Rust Vocabulary",
            "Essential Rust terms and concepts for beginners",
        ),
        (
            teacher1,
            student2,
            "Advanced Rust Patterns",
            "Complex patterns and idioms in Rust programming",
        ),
        (
            teacher2,
            student2,
            "Web Development Terms",
            "HTTP, REST, and web framework concepts",
        ),
        (
            teacher2,
            student3,
            "Database Fundamentals",
            "SQL commands and database design principles",
        ),
    ];

    for (created_by, assignee, name, description) in decks {
        let deck_id = nanoid::nanoid!();

        sqlx::query!(
            r#"
            INSERT INTO decks (id, name, description, created_by, assignee)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            deck_id,
            name,
            description,
            created_by,
            assignee
        )
        .execute(db)
        .await?;
    }

    Ok(())
}
