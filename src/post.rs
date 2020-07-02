use crate::util::{md_to_html, read_file};
extern crate toml;

#[derive(Debug)]
pub struct Post {
    metadata: Metadata,
    content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Metadata {
    title: String,
    date: String,
    draft: bool,
    categories: Vec<toml::Value>,
    tags: Vec<toml::Value>,
}

impl Post {
    pub fn parse(path: String) -> Post {
        let inputcontent = read_file(path);

        let content_parts: Vec<&str> = inputcontent.split("<post-metadata>").collect();
        //let metadata_unparsed = format!("{:?}", content_parts[1]);
        let metadata_unparsed = content_parts[1];
        let mdcontent = content_parts[2];

        println!("{}", metadata_unparsed);
        let metadata_parsed: Metadata =
            toml::from_str(metadata_unparsed).expect("Unable to parse toml metadata");

        let htmlcontent = md_to_html(mdcontent);

        Post {
            metadata: metadata_parsed,
            content: htmlcontent,
        }
    }
}
