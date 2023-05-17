use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// SurrealDB quick start example from https://surrealdb.com/docs/integration/libraries/rust
#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
    // INFO: None deserializes to null
    // optional fields
    age: Option<u8>,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            title: "Default Title".into(),
            name: Name {
                first: "Default".into(),
                last: "Name".into(),
            },
            marketing: false,
            age: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

static DB: Surreal<Client> = Surreal::init();

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    // let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    DB.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Record = create_person().await?;
    dbg!(&created);

    // Select all people records
    let people: Vec<Person> = DB.select("person").await?;
    for person in people.iter() {
        dbg!(&person);
    }

    Ok(())
}

async fn create_person() -> surrealdb::Result<Record> {
    // Create a new person with a random id
    let created: Record = DB
        .create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            // can be useful for auto propagating new fields
            ..Default::default()
        })
        .await?;
    Ok(created)
}
