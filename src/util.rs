pub use crate::post::Post;
use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path: String) -> String {
    let mut file = File::open(file_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the content of file to a string");
    contents
}

pub fn md_to_html(input_content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn find_all_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();
    for entry in glob("content/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => posts.push(Post::parse(path.display().to_string())),
            Err(e) => println!("{:?}", e),
        }
    }

    posts
}
