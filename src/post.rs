use crate::util::{md_to_html, read_file};
extern crate toml;
use toml::Value;

#[derive(Debug)]
pub struct Post {
    title: String,
    date: String,
    draft: bool,
    categories: Vec<toml::Value>,
    tags: Vec<toml::Value>,
    content: String,
}

impl Post {
    pub fn parse(path: String) -> Post {
        let inputcontent = read_file(path);

        let content_parts: Vec<&str> = inputcontent.split("<post-metadata>").collect();
        let metadata_unparsed = content_parts[1];
        let mdcontent = content_parts[2];

        let metadata_parsed: Value =
            toml::from_str(metadata_unparsed).expect("Unable to parse toml");

        let htmlcontent = md_to_html(mdcontent);

        Post {
            title: match metadata_parsed["title"].as_str() {
                Some(k) => k.to_string(),
                _ => String::from(""),
            },
            date: match metadata_parsed["date"].as_str() {
                Some(k) => k.to_string(),
                _ => String::from(""),
            },
            draft: match metadata_parsed["draft"].as_bool() {
                Some(k) => k,
                _ => false,
            },
            categories: match metadata_parsed["categories"].as_array() {
                Some(k) => k.clone(),
                _ => vec![],
            },
            tags: match metadata_parsed["tags"].as_array() {
                Some(k) => k.clone(),
                _ => vec![],
            },
            content: htmlcontent,
        }
    }
}
