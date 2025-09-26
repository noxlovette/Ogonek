// src/bin/seed_db.rs
use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use fake::{
    Fake,
    faker::{
        internet::en::SafeEmail,
        lorem::en::{Paragraph, Sentence},
        name::en::Name,
    },
};
use ogonek::{auth::password::hash_password, db::init_db};
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

    let roles = vec![
        ("dev_teacher1", "teacher"),
        ("dev_teacher2", "teacher"),
        ("dev_student1", "student"),
        ("dev_student2", "student"),
        ("dev_student3", "student"),
        ("dev_god", "god"),
        ("dev_moderator", "moderator"),
        ("dev_admin", "admin"),
    ];

    let mut user_ids = Vec::new();

    for (username, role) in roles {
        let user_id = nanoid::nanoid!();
        let name: String = Name().fake();
        let email: String = SafeEmail().fake();

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

    for user_id in user_ids.iter() {
        let video_call_url = if (0..10).fake::<i32>() < 6 {
            // 60% chance of having video call URL
            let meeting_id: u64 = (100000000000..999999999999).fake();
            let password: u32 = (100000..999999).fake();
            Some(format!("https://zoom.us/j/{}?pwd={}", meeting_id, password))
        } else {
            None
        };

        let avatar_url = if (0..10).fake::<i32>() < 8 {
            // 80% chance of having avatar
            let avatar_id: u32 = (1..1000).fake();
            Some(format!("https://avatars.dev/avatar_{}.png", avatar_id))
        } else {
            None
        };

        let telegram_id = if (0..10).fake::<i32>() < 7 {
            Some("900828558")
        } else {
            None
        };

        sqlx::query!(
            r#"
            UPDATE profile
            SET video_call_url = $2, avatar_url = $3, telegram_id = $4
            WHERE user_id = $1
            "#,
            user_id,
            video_call_url,
            avatar_url,
            telegram_id
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

    let assignments = vec![
        (teacher1, student1),
        (teacher1, student2),
        (teacher2, student2),
        (teacher2, student3),
    ];

    for (created_by, assignee) in assignments {
        let lesson_id = nanoid::nanoid!();
        let title: String = Sentence(3..6).fake();
        let topic: String = Sentence(2..4).fake();
        let markdown = format!(
            "# {}\n\n{}\n\n## Key Points\n\n{}\n\n## Examples\n\n```\n{}\n```",
            topic,
            Paragraph(3..5).fake::<String>(),
            Paragraph(2..3).fake::<String>(),
            Paragraph(1..2).fake::<String>()
        );

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

    let assignments = vec![
        (teacher1, student1),
        (teacher1, student2),
        (teacher2, student2),
        (teacher2, student3),
    ];

    for (created_by, assignee) in assignments {
        let task_id = nanoid::nanoid!();
        let title: String = Sentence(3..6).fake();
        let markdown = format!(
            "# {}\n\n{}\n\n## Requirements\n\n{}\n\n## Guidelines\n\n{}",
            Sentence(2..4).fake::<String>(),
            Paragraph(2..4).fake::<String>(),
            Paragraph(2..3).fake::<String>(),
            Paragraph(1..2).fake::<String>()
        );
        let completed: bool = (0..10).fake::<i32>() < 3; // 30% chance of being completed
        let due_date = if (0..10).fake::<i32>() < 7 {
            // 70% chance of having a due date
            Some(now + chrono::Duration::days((1..30).fake::<i64>()))
        } else {
            None
        };

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

    let assignments = vec![
        (teacher1, student1),
        (teacher1, student2),
        (teacher2, student2),
        (teacher2, student3),
    ];

    for (created_by, assignee) in assignments {
        let deck_id = nanoid::nanoid!();
        let title: String = Sentence(2..5).fake();
        let description: String = Sentence(5..10).fake();

        sqlx::query!(
            r#"
            INSERT INTO decks (id, title, description, created_by, assignee)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            deck_id,
            title,
            description,
            created_by,
            assignee
        )
        .execute(db)
        .await?;
    }

    Ok(())
}
