use crate::{models::Post, functions::connect::connect};
use diesel::prelude::*;

pub fn show_posts() {
    use crate::schema::posts::dsl::*;    

    let connection = &mut connect();
    let results = posts
        .select(Post::as_select())
        .load(connection).expect("Error loading posts");

    for post in results {
        println!("{:?}", post)
    }
}