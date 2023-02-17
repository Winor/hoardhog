
use crate::db::PgDbConn;
use crate::schema::items;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::{Debug, status::Created};
pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
use diesel::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(primary_key(id))]
#[table_name="items"]
pub struct Item {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub title: String,
    #[diesel(deserialize_as = i32)]
    pub quantity: Option<i32>,
    #[diesel(deserialize_as = String)]
    pub unit: Option<String>,
    pub about: Option<String>,
    #[diesel(deserialize_as = bool)]
    pub instock: Option<bool>,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}


#[get("/items")]
pub async fn list_items(db: PgDbConn) ->  Result<Json<Vec<Item>>> {
    let ids= db.run(move |conn| {
        items::table

            .load::<Item>(conn)
    }).await?;
    Ok(Json(ids))
}

#[post("/items", data = "<post>")]
pub async fn create_items(db: PgDbConn, post: Json<Item>) -> Result<Created<Json<Item>>> {
    let post_value = post.clone();
    db.run(move |conn| {
        diesel::insert_into(items::table)
            .values(&*post_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(post))
}

#[put("/items/<id>", data = "<post>")]
pub async fn edit_items(db: PgDbConn, post: Json<Item>, id: i32) -> Result<Created<Json<Item>>> {
    let post_value = post.clone();
    db.run(move |conn| {
        diesel::update(items::table)
            .filter(items::id.eq(id))
            .set(&*post_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(post))
}

#[delete("/items/<id>")]
pub async fn delete_items(db: PgDbConn, id: i32) -> Result<Option<()>> {
    let affected = db.run(move |conn| {
        diesel::delete(items::table)
            .filter(items::id.eq(id))
            .execute(conn)
    }).await?;

    Ok((affected == 1).then(|| ()))
}