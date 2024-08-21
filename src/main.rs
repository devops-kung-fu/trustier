pub mod models;
use async_std::task;
use clap::Parser;
use colored::*;
use cyclonedx_bom::prelude::*;
use packageurl::PackageUrl;
use std::{fs, str::FromStr};
use surf;

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
        println!("* Provided file {} is not a valid SBOM", &args.sbom);
        return;
    }

    if let Some(serial_number) = &bom.serial_number {
        println!("* SBOM Serial Number: {}", serial_number);
    }

    let rate_limit_ms = 100; // Rate limit in milliseconds
    let fut = process_sbom(&bom, rate_limit_ms);
    match task::block_on(fut) {
        Ok(_) => println!("process_sbom completed successfully"),
        Err(err) => eprintln!("Error in process_sbom: {}", err),
    }
}

//Produces an ascii header for trustier and prints it to the command line
fn print_ascii_header() {
    let header = r#"
  __               __  _       
 / /_______ _____ / /_(_)__ ____
/ __/ __/ // (_-</ __/ / -_) __/
\__/_/  \_,_/___/\__/_/\__/_/   
    "#;
    println!(
        "{}\n{}\n{}\n",
        header,
        "DevOps Kung Fu Mafia".bold(),
        "https://github.com/devops-kung-fu/trustier"
    );
}

async fn process_sbom(
    bom: &Bom,
    rate_limit_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut collected_purls: Vec<String> = Vec::new();

    if let Some(components) = &bom.components {
        collected_purls = components
            .0
            .iter()
            .filter_map(|component| component.purl.as_ref().map(|purl| purl.to_string()))
            .collect();
    }

    let original_count = collected_purls.len();

    filter_purls(&mut collected_purls);

    if collected_purls.len() < original_count {
        println!(
            "{}",
            r"* trustypkg.dev only supports the following ecosystems: pypi, npm, crates, maven, go"
                .red()
        );
        println!(
            r"* Removed {} out of {} detected Purls in the SBOM",
            original_count - collected_purls.len(),
            original_count
        )
    }

    println!("* Number of Purls: {}\n", collected_purls.len());

    // Call fetch_purl_bodies
    let bodies = fetch_purl_bodies(&collected_purls, rate_limit_ms).await?;

    // Process the fetched bodies
    for (i, body) in bodies.iter().enumerate() {
        println!("PURL {}: {}", i, body);
    }

    Ok(())
}

async fn fetch_purl_bodies(
    purls: &Vec<String>,
    rate_limit_ms: u64,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut bodies = Vec::new();

    for p in purls {
        match PackageUrl::from_str(p) {
            Ok(purl) => {
                let url = format!(
                    r"https://api.trustypkg.dev/v2/pkg?package_name={}&package_type={}",
                    purl.name(),
                    purl.ty()
                );

                println!("* Fetching trust information for {}:", p);

                let mut response = surf::get(url).await?;
                let body = response.body_string().await?;
                bodies.push(body);

                // Rate limiting
                task::sleep(std::time::Duration::from_millis(rate_limit_ms)).await;
            }
            Err(err) => {
                eprintln!("Error parsing PURL: {}", err);
            }
        }
    }

    Ok(bodies)
}

fn filter_purls(collected_purls: &mut Vec<String>) {
    let allowed_types = ["pypi", "npm", "crates", "maven", "go"];

    collected_purls.retain(|purl_str| {
        if let Ok(purl) = PackageUrl::from_str(purl_str) {
            allowed_types.contains(&purl.ty().to_string().as_str())
        } else {
            false
        }
    });
}
