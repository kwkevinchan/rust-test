#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use diesel::prelude::*;
use models::{Item, NewItem};

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[post("/item", format = "json", data = "<new_item>")]
async fn create_item(conn: DbConn, new_item: Json<NewItem>) -> Json<Item> {
    conn.run(|c| {
        diesel::insert_into(schema::items::dsl::items)
            .values(&*new_item)
            .execute(c)
            .expect("Error inserting new item");

        schema::items::dsl::items
            .order(schema::items::dsl::id.desc())
            .first::<Item>(c)
            .expect("Error loading created item")
    }).await.into()
}

#[get("/item/<id>")]
async fn get_item(conn: DbConn, id: i32) -> Json<Item> {
    conn.run(move |c| {
        schema::items::dsl::items
            .find(id)
            .first(c)
            .expect("Error loading item")
    }).await.into()
}

#[put("/item/<id>", format = "json", data = "<updated_item>")]
async fn update_item(conn: DbConn, id: i32, updated_item: Json<NewItem>) -> Json<Item> {
    conn.run(move |c| {
        diesel::update(schema::items::dsl::items.find(id))
            .set(&*updated_item)
            .execute(c)
            .expect("Error updating item");

        schema::items::dsl::items
            .find(id)
            .first(c)
            .expect("Error loading updated item")
    }).await.into()
}

#[delete("/item/<id>")]
async fn delete_item(conn: DbConn, id: i32) -> &'static str {
    conn.run(move |c| {
        diesel::delete(schema::items::dsl::items.find(id))
            .execute(c)
            .expect("Error deleting item");
    }).await;

    "Item deleted"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![create_item, get_item, update_item, delete_item])
}
