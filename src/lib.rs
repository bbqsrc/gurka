#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(trivial_bounds)]

// Workaround for #50504
#![allow(proc_macro_derive_resolution_fallback)]

extern crate base64;
#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate derive_builder;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate juniper_relay;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate uuid;
extern crate heck;

mod schema;
pub mod web;
pub mod context;
pub mod models;
pub mod graphql;

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
