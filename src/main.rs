pub mod models;
use clap::Parser;
use cyclonedx_bom::prelude::*;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]

struct Args {
    #[arg(required(true))]
    sbom: String,
}

fn main() {
    print_ascii_header();
    let args = Args::parse();

    let file_contents = match fs::read_to_string(&args.sbom) {
        Ok(contents) => {
            println!("* Loaded SBOM from file: {}", &args.sbom);
            contents
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let bom = Bom::parse_from_json_v1_5(file_contents.as_bytes()).expect("Failed to parse BOM");

    let validation_result = bom.validate();
    if validation_result.passed() {
        println!("* SBOM is valid")
    } else {
        println!("* Provided file is not a valid SBOM");
        return;
    }

    if let Some(serial_number) = &bom.serial_number {
        println!("* Serial Number: {}", serial_number);
    }

    let mut collected_purls: Vec<String> = Vec::new();

    if let Some(components) = &bom.components {
        collected_purls = components
            .0
            .iter()
            .filter_map(|component| component.purl.as_ref().map(|purl| purl.to_string()))
            .collect();
    }

    println!("* Number of Purls: {}\n", collected_purls.len());
}

//Produces an ascii header for trustier and prints it to the command line
fn print_ascii_header() {
    let header = r#"
  __               __  _       
 / /_______ _____ / /_(_)__ ____
/ __/ __/ // (_-</ __/ / -_) __/
\__/_/  \_,_/___/\__/_/\__/_/   

©2024 DevOps Kung Fu Mafia
https://github.com/devops-kung-fu/trustier

    "#;
    println!("{}", header);
}
