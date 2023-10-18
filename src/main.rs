extern crate pulldown_cmark;
use std::{
    fs::File,
    io::{Read, Write},
};

use pulldown_cmark::{html, Parser};

fn mkdwn_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut out = String::new();
    html::push_html(&mut out, parser);

    out
}

fn main() {
    let mut file = File::open("src/markdown.txt").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let output = mkdwn_to_html(&file_content);

    let mut output_file = File::create("src/index.html").expect("Failed to create file");
    output_file
        .write_all("<html>".as_bytes())
        .expect("Failed to write <html");
    output_file
        .write_all((output).as_bytes())
        .expect("I dunno how to write...");
    output_file
        .write_all("</html>".as_bytes())
        .expect("Failed to write </html>");
}
