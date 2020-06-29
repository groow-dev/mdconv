use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .value_name("FILE")
                .help("Markdown input file")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output")
                .value_name("FILE")
                .help("HTML output file")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    match (matches.value_of("input"), matches.value_of("output")) {
        (Some(input), Some(output)) => {
            let markdown_content = fs::read_to_string(&input)?;
            let options = Options::all();
            let parser = Parser::new_ext(&markdown_content, options);
            let html_file = fs::File::create(&output)?;
            html::write_html(html_file, parser)?;
        }
        (Some(input), None) => {
            let markdown_content = fs::read_to_string(&input)?;
            let options = Options::all();
            let parser = Parser::new_ext(&markdown_content, options);
            html::write_html(std::io::stdout(), parser)?;
        }
        (None, Some(output)) => {
            let mut markdown_content = String::new();
            std::io::stdin().read_to_string(&mut markdown_content)?;
            let options = Options::all();
            let parser = Parser::new_ext(&markdown_content, options);
            let html_file = fs::File::create(&output)?;
            html::write_html(html_file, parser)?;
        }
        (None, None) => {
            let mut markdown_content = String::new();
            std::io::stdin().read_to_string(&mut markdown_content)?;
            let options = Options::all();
            let parser = Parser::new_ext(&markdown_content, options);
            html::write_html(std::io::stdout(), parser)?;
        }
    }

    Ok(())
}
