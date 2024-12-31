use faker_rand::en_us::names::FirstName;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Record;
use surrealdb::Error;
use surrealdb::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    id: RecordId,
    created_by: Option<RecordId>,
}

#[derive(Serialize, Deserialize)]
pub struct Params<'a> {
    name: &'a str,
    pass: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct RecordUser {
    name: String,
    pass: String,
}

pub async fn make_new_user(db: &Surreal<Client>) -> Result<RecordUser, Error> {
    let name = rand::random::<FirstName>().to_string();
    let pass = rand::random::<FirstName>().to_string();
    println!("Signing in as user {name} and password {pass}");
    let jwt = db
        .signup(Record {
            access: "account",
            namespace: "namespace",
            database: "database",
            params: Params {
                name: &name,
                pass: &pass,
            },
        })
        .await?
        .into_insecure_token();
    println!("New user created!\n\nName: {name}\nPassword: {pass}\nToken: {jwt}\n\nTo log in, use this command:\n\nsurreal sql --namespace namespace --database database --pretty --token \"{jwt}\"\n");
    Ok(RecordUser { name, pass })
}

pub async fn get_new_token(db: &Surreal<Client>, user: &RecordUser) -> Result<(), Error> {
    let jwt = db
        .signin(Record {
            access: "account",
            namespace: "namespace",
            database: "database",
            params: Params {
                name: &user.name,
                pass: &user.pass,
            },
        })
        .await?
        .into_insecure_token();
    println!("New token! Sign in with surreal sql --namespace namespace --database database --pretty --token \"{jwt}\"\n");
    Ok(())
}
