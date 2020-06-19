use clap::Clap;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::io;
use std::io::prelude::*;

/// A simple markdown to html converter
#[derive(Clap)]
#[clap(version = "0.1.0", author = "groow-dev")]
struct Opts {
    /// Sets the markdown input file
    #[clap(short)]
    input: Option<String>,
    /// Sets the html output file
    #[clap(short)]
    output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    match (opts.input, opts.output) {
        (Some(input), Some(output)) => {
            let markdown_content = fs::read_to_string(&input)?;
            let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
            let mut html_file = fs::File::create(&output)?;
            html_file.write_all(html_content.as_bytes())?
        }
        (Some(input), None) => {
            let markdown_content = fs::read_to_string(&input)?;
            let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
            io::stdout().write_all(html_content.as_bytes())?;
        }
        (None, Some(output)) => {
            let mut markdown_content = String::new();
            io::stdin().read_to_string(&mut markdown_content)?;
            let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
            let mut html_file = fs::File::create(&output)?;
            html_file.write_all(html_content.as_bytes())?
        }
        (None, None) => {
            let mut markdown_content = String::new();
            io::stdin().read_to_string(&mut markdown_content)?;
            let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());
            io::stdout().write_all(html_content.as_bytes())?;
        }
    }

    Ok(())
}
