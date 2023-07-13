use postgres::{Client, NoTls};
use serde::Deserialize;
use std::error::Error;

#[derive(Clone, Debug, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("postgres://postgres@localhost:5432", NoTls)?;

    client.execute("CREATE TABLE IF NOT EXISTS Person (
        name VARCHAR NOT NULL,
        age INT NOT NULL
    )", &[])?;

    client.execute("INSERT INTO Person (name, age) VALUES ($1, $2)",
                   &[&"Jane", &23i32])?;

    client.execute("INSERT INTO Person (name, age) VALUES ($1, $2)",
                   &[&"Alice", &32i32])?;

    let rows = client.query("SELECT name, age FROM Person", &[])?;

    let people: Vec<Person> = serde_postgres::from_rows(&rows)?;

    for person in people {
        println!("{:?}", person);
    }

    Ok(())
}