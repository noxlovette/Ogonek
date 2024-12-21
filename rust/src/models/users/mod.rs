use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id:Uuid,
    pub password: String,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub is_superuser: bool,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub is_staff: bool,
    pub is_active: bool,
    pub date_joined: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub password: &'a str,
    pub username: &'a str,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub is_superuser: Option<bool>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub is_staff: Option<bool>,
    pub is_active: Option<bool>,
    pub date_joined: chrono::NaiveDateTime,
}