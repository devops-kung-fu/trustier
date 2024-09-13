pub mod models;

use async_std::task;
use clap::Parser;
// use clap_stdin::{FileOrStdin, MaybeStdin};
use colored::*;
use cyclonedx_bom::prelude::*;
use models::TrustyResponse;
use packageurl::PackageUrl;
use serde_json;
use std::{fs, str::FromStr};
use surf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    //The SBOM to process
    #[arg(required = true)]
    //sbom: FileOrStdin<String>,  //need to handle this scenario
    sbom: String,

    //The time to pause before requests to https://trustypkg.dev
    #[arg(short, long, default_value_t = 500)]
    ratelimit: u64,

    //Optional file name to write json output to
    #[arg(short, long, required = false)]
    output_file: Option<String>,
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

    let bom = match Bom::parse_from_json_v1_5(file_contents.as_bytes()) {
        Ok(bom) => bom,
        Err(_) => {
            eprintln!("Failed to parse BOM");
            return;
        }
    };

    if !bom.validate().passed() {
        eprintln!("* Provided file {} is not a valid SBOM", &args.sbom);
        return;
    }

    println!("* SBOM is valid");
    if let Some(serial_number) = &bom.serial_number {
        println!("* SBOM Serial Number: {}", serial_number);
    }
    if let Err(err) = task::block_on(process_sbom(&bom, args.output_file, args.ratelimit)) {
        eprintln!("Error processing SBOM (process_sbom): {}", err);
    }

    println!("{}", "DONE!".green().bold());
}

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
    output_file: Option<String>,
    rate_limit_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut collected_purls = if let Some(components) = &bom.components {
        components
            .0
            .iter()
            .filter_map(|component| component.purl.as_ref().map(|purl| purl.to_string()))
            .collect::<Vec<_>>()
        //TODO: may have to dedupe
    } else {
        Vec::new()
    };

    let original_count = collected_purls.len();
    filter_purls(&mut collected_purls);

    if collected_purls.len() < original_count {
        println!(
            "* {}",
            r"trustypkg.dev only supports the following ecosystems: pypi, npm, crates, maven, go"
                .red()
        );
        println!(
            r"* Removed {} out of {} detected Purls in the SBOM",
            original_count - collected_purls.len(),
            original_count
        );
    }

    if collected_purls.len() != 0 {
        println!("* Processing {} Purls...\n", collected_purls.len());
    } else {
        println!("* Nothing to do...\n")
    }

    let responses = fetch_purl_bodies(&collected_purls, rate_limit_ms).await?;

    if let Some(of) = output_file.clone() {
        let json = serde_json::to_string_pretty(&responses).unwrap();
        let of_clone = of.clone();
        let output_path = std::path::Path::new(&of_clone);
        if let Some(parent_dir) = output_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).expect("Failed to create output directory");
            }
        }
        fs::write(of_clone, json).expect("Failed to write JSON to file");
        println!("\n* JSON written to file: {}\n", of);
    } else {
        let json = serde_json::to_string_pretty(&responses).unwrap();
        println!("{}", json);
    }

    Ok(())
}

async fn fetch_purl_bodies(
    purls: &[String],
    rate_limit_ms: u64,
) -> Result<Vec<TrustyResponse>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut responses: Vec<TrustyResponse> = Vec::new();

    for p in purls {
        match PackageUrl::from_str(p) {
            Ok(purl) => {
                let url = format!(
                    "https://api.trustypkg.dev/v2/pkg?package_name={}&package_type={}",
                    purl.name(),
                    purl.ty()
                );

                println!("* Fetching trust information for {}:", p);

                let body = surf::get(url).await?.body_string().await?;

                // eprintln!("* Response: {}", body);

                match serde_json::from_str::<TrustyResponse>(&body) {
                    Ok(mut resp) => {
                        //println!("Success: {:?}", resp);
                        resp.purl = Some(p.to_string());
                        responses.push(resp);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                    }
                }

                task::sleep(std::time::Duration::from_millis(rate_limit_ms)).await;
            }
            Err(err) => eprintln!("Error parsing PURL: {}", err),
        }
    }

    Ok(responses)
}

fn filter_purls(collected_purls: &mut Vec<String>) {
    let allowed_types = ["pypi", "npm", "cargo", "maven", "go"];

    collected_purls.retain(|purl_str| match PackageUrl::from_str(purl_str) {
        Ok(purl) => allowed_types.contains(&purl.ty()),
        Err(_) => false,
    });

    //if any of the collected purls contain the word cargo, replace it with crates (trustypkg.dev only supports crates, sboms contain cargo)
    for purl in collected_purls.iter_mut() {
        if purl.contains("cargo") {
            *purl = purl.replace("cargo", "crates");
        }
    }
}
