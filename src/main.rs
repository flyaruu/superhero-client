use std::{thread::sleep, time::Duration};

use generated::{add_hero, DbConnection, Hero};
use log::info;
use sql::heroes::SqlHero;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query_as};
use tokio::spawn;

pub mod generated;
pub mod sql;

const DB_NAME: &str = "superhero-server";

#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .connect("postgres://superman:superman@localhost:5432/heroes_database")
        .await
        .unwrap();
    println!("Pool created");
    let db = connect_to_client();
    let handle = db.run_threaded();
    for hero in all_heroes(&pool).await {
        let name = hero.name.clone();
        let converted: Hero = hero.into();
        let id = converted.id;
        db.reducers.add_hero(converted).unwrap();
        sleep(Duration::from_millis(1));        
        println!("Hero: {} inserted. Id: {}", name, id);
    }
}

fn connect_to_client()->DbConnection {
    DbConnection::builder()
        .with_uri("http://localhost:3000")
        .with_module_name(DB_NAME)
        .build()
        .unwrap()
}

async fn all_heroes(pool: &Pool<Postgres>) -> Vec<SqlHero> {
    query_as("select * from Hero")
        .fetch_all(pool)
        .await
        .unwrap()
}
