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
    //The SBOM to process
    #[arg(required = true)]
    sbom: String,

    //The time to pause before requests to https://trustypkg.dev
    #[arg(short, long, default_value_t = 500)]
    ratelimit: u64,

    //Optional file name to write json output to
    #[arg(short, long)]
    output_file: String
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

    if let Err(err) = task::block_on(process_sbom(&bom, args.ratelimit)) {
        eprintln!("Error in process_sbom: {}", err);
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
    rate_limit_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut collected_purls = if let Some(components) = &bom.components {
        components
            .0
            .iter()
            .filter_map(|component| component.purl.as_ref().map(|purl| purl.to_string()))
            .collect::<Vec<_>>()
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

    let bodies = fetch_purl_bodies(&collected_purls, rate_limit_ms).await?;

    for (i, body) in bodies.iter().enumerate() {
        println!("PURL {}\n: {}\n\n", i, body);
    }

    Ok(())
}

async fn fetch_purl_bodies(
    purls: &[String],
    rate_limit_ms: u64,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut bodies = Vec::new();

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
                bodies.push(body);

                task::sleep(std::time::Duration::from_millis(rate_limit_ms)).await;
            }
            Err(err) => eprintln!("Error parsing PURL: {}", err),
        }
    }

    Ok(bodies)
}

fn filter_purls(collected_purls: &mut Vec<String>) {
    let allowed_types = ["pypi", "npm", "crates", "maven", "go"];

    collected_purls.retain(|purl_str| {
        PackageUrl::from_str(purl_str)
            .map(|purl| allowed_types.contains(&purl.ty()))
            .unwrap_or(false)
    });
}
