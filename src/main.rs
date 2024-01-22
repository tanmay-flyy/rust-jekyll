use std::fs::File;
use std::io::{self, Read};
use pulldown_cmark::{html, Options, Parser};

fn main() -> io::Result<()> {
    // Replace "example.md" with the path to your Markdown file
    let file_path = "src/posts/post1.md";

    // Read the content of the Markdown file
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Set up the options for the Markdown parser
    let options = Options::empty();

    // Create a Markdown parser with the specified options
    let parser = Parser::new_ext(&content, options);

    // Create a buffer to hold the HTML output
    let mut html_output = String::new();

    // Convert the Markdown to HTML
    html::push_html(&mut html_output, parser);

    // Print or use the HTML output as needed
    println!("{}", html_output);

    Ok(())
}
