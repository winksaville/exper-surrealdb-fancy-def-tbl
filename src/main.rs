use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i64,
    is_active: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let surql = "DEFINE TABLE amt_item SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL PERMISSIONS NONE;";
    //let response = db.query(surql).await?;
    //dbg!(&response);
    match db.query(surql).await {
        Ok(response) => {
            println!("Successfully created table");
            dbg!(&response);
        }
        Err(e) => {
            println!("Error: {e}");
            return Err(e);
        }
    }

    Ok(())
}
