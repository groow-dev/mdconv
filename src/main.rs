use clap::Clap;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::io::prelude::*;

/// A simple markdown to html converter
#[derive(Clap)]
#[clap(version = "0.1.0", author = "groow")]
struct Opts {
    /// Sets the markdown input file
    #[clap(short)]
    input: String,
    /// Sets the html output file
    #[clap(short)]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let markdown_content = fs::read_to_string(&opts.input)
        .unwrap_or_else(|_| panic!(format!("Couldn't open file: {}", opts.input)));

    let html_content = markdown_to_html(&markdown_content, &ComrakOptions::default());

    let mut html_file = match fs::File::create(&opts.output) {
        Err(e) => panic!("Couldn't create {}: {}", opts.output, e),
        Ok(html_file) => html_file,
    };

    if let Err(e) = html_file.write_all(html_content.as_bytes()) {
        panic!("Couldn't write to {}: {}", &opts.output, e)
    }
}
