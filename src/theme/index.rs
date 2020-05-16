extern crate maud;
use maud::html;

pub fn render_index() {
    let name = "Lyra";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
}