use diesel::prelude::*;

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}