use diesel::RunQueryDsl;
use random_word;

use crate::{models::Post, functions::connect::connect};

fn gen_words(n: usize) -> String {
    (0..n).map(|_| random_word::gen(random_word::Lang::En)).collect::<Vec<&str>>().join(" ")
}


pub fn add_post() {
    let mut conn = connect();

    let title = gen_words(3);
    let body = gen_words(20);
    
    let new_post = Post {
        id: Some(2),
        title,
        body,
        published: false,
    };
    
    let res = diesel::insert_into(crate::schema::posts::table)
        .values(&new_post)
        .execute(&mut conn);

    if res.is_ok() {
        println!("{}", new_post.title);
        println!("{}", new_post.body);
    }
}