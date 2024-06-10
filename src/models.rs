use super::schema::items;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub description: String,
}
