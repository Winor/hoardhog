
use sea_orm::DatabaseConnection;
// //use crate::schema::*;
// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}


// use crate::schema::{items, locations, groups, tags, item_location, item_group, tag_item, tag_group};

// #[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Selectable, PartialEq)]
// #[diesel(primary_key(id))]
// #[diesel(table_name = items)]
// pub struct Item {
//     #[diesel(deserialize_as = i32)]
//     pub id: Option<i32>,
//     pub name: String,
//     #[diesel(deserialize_as = i32)]
//     pub quantity: Option<i32>,
//     #[diesel(deserialize_as = String)]
//     pub unit: Option<String>,
//     pub description: Option<String>,
//     #[diesel(deserialize_as = bool)]
//     pub instock: Option<bool>,
// }

// #[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Selectable)]
// #[diesel(primary_key(id))]
// #[diesel(table_name = locations)]
// pub struct Location {
//     #[diesel(deserialize_as = i32)]
//     pub id: Option<i32>,
//     pub name: String,
//     pub description: Option<String>,
// }

// #[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Selectable)]
// #[diesel(primary_key(id))]
// #[diesel(table_name = groups)]
// pub struct Group {
//     #[diesel(deserialize_as = i32)]
//     pub id: Option<i32>,
//     pub name: String,
//     pub description: Option<String>,
// }

// #[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Insertable, AsChangeset, Selectable)]
// #[diesel(primary_key(id))]
// #[diesel(table_name = tags)]
// pub struct Tag {
//     #[diesel(deserialize_as = i32)]
//     pub id: Option<i32>,
//     pub description: Option<String>,
// }

// #[derive(Identifiable, Selectable, Queryable, Associations, Debug, Clone)]
// #[diesel(belongs_to(Item))]
// #[diesel(belongs_to(Location))]
// #[diesel(table_name = item_location)]
// #[diesel(primary_key(item_id, location_id))]
// pub struct ItemLocation {
//     pub item_id: i32,
//     pub location_id: i32,
// }

// #[derive(Identifiable, Selectable, Queryable, Associations, Insertable, Debug)]
// #[diesel(belongs_to(Item))]
// #[diesel(belongs_to(Group))]
// #[diesel(table_name = item_group)]
// #[diesel(primary_key(item_id, group_id))]
// pub struct ItemGroup {
//     pub item_id: i32,
//     pub group_id: i32,
// }

// #[derive(Identifiable, Selectable, Queryable, Associations, Insertable, Debug)]
// #[diesel(belongs_to(Tag))]
// #[diesel(belongs_to(Item))]
// #[diesel(table_name = tag_item)]
// #[diesel(primary_key(tag_id, item_id))]
// pub struct TagItem {
//     pub tag_id: i32,
//     pub item_id: i32,
// }

// #[derive(Identifiable, Selectable, Queryable, Associations, Insertable, Debug)]
// #[diesel(belongs_to(Tag))]
// #[diesel(belongs_to(Group))]
// #[diesel(table_name = tag_group)]
// #[diesel(primary_key(tag_id, group_id))]
// pub struct TagGroup {
//     pub tag_id: i32,
//     pub group_id: i32,
// }

// #[derive(Serialize)]
// pub struct ItemWithInfo {
//     #[serde(flatten)]
//     item: Item,
//     location: Location,
//     group: Group,
// }

// pub async fn list_allditems(pool: web::Data<DbPool>) ->  Result<HttpResponse, Error> {
//     let ids= db.run(move |conn| {
//         let all_items = items::table.load::<Item>(conn);
//         let all_locations = ItemLocation::belonging_to(&all_items)
//             .select(ItemLocation::as_select())
//             .load(conn);

//     }).await;
//     Ok(Json(ids))
// }



// #[get("/allitems")]
// pub async fn list_allitems(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
//     let items = web::block(move || {
//         let mut conn = pool.get()?;
//         let all_items = items::table.load::<Item>(conn);
//         let all_locations = ItemLocation::belonging_to(&all_items)
//             .select(ItemLocation::as_select())
//             .load(conn);

//     }).await;
//     Ok(Json(ids))
// }

// #[post("/items", data = "<post>")]
// pub async fn create_items(pool: web::Data<DbPool>, post: Json<Item>) -> Result<Created<Json<Item>>> {
//     let post_value = post.clone();
//     db.run(move |conn| {
//         diesel::insert_into(items::table)
//             .values(&*post_value)
//             .execute(conn)
//     }).await?;

//     Ok(Created::new("/").body(post))
// }

// #[put("/items/<id>", data = "<post>")]
// pub async fn edit_items(pool: web::Data<DbPool>, post: Json<Item>, id: i32) -> Result<Created<Json<Item>>> {
//     let post_value = post.clone();
//     db.run(move |conn| {
//         diesel::update(items::table)
//             .filter(items::id.eq(id))
//             .set(&*post_value)
//             .execute(conn)
//     }).await?;

//     Ok(Created::new("/").body(post))
// }

// #[delete("/items/<id>")]
// pub async fn delete_items(pool: web::Data<DbPool>, id: i32) -> Result<Option<()>> {
//     let affected = db.run(move |conn| {
//         diesel::delete(items::table)
//             .filter(items::id.eq(id))
//             .execute(conn)
//     }).await?;

//     Ok((affected == 1).then(|| ()))
// }