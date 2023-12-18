use functions::{show_posts::show_posts, add_post::add_post};

pub mod schema;
pub mod models;
pub mod functions;

fn main() {
    add_post();
    show_posts();
}
