#![feature(proc_macro_hygiene)]
mod post;
mod util;
pub use post::Post;

fn main() {
    //    let path = "content/posts/post1.md";
    //    let post1 = Post::parse(String::from(path));
    println!("{:?}", util::find_all_posts());
}
