use quicli::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(
        long = "output",
        short = "o",
        default_value = "./",
        help = "Write to <filename>"
    )]
    output: String,
    #[structopt(
        long = "delimiter",
        short = "d",
        default_value = "/",
        help = "Join region names with delimiter"
    )]
    delimiter: String,
    #[structopt(help = "The source file")]
    file: String,
    #[structopt(long = "verbose", short = "v", help = "Use verbose output")]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct Region {
    code: String,
    name: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    //println!("{:?}", args);

    let mut all_regions = Vec::new();
    let mut processed_results = Vec::new();
    let mut processed_lines = 0;

    let input_file = File::open(&args.file)?;
    let reader = BufReader::new(input_file);
    println!("Starting to read lines from {}", args.file);
    for line in reader.lines() {
        processed_lines += 1;
        let text = line.unwrap();
        if text.trim().is_empty() {
            continue;
        }
        let mut iter = text.trim().split_ascii_whitespace();
        let code = match iter.next() {
            Some(value) => value,
            None => {
                println!("Invalid line[{}]: {}", processed_lines, text);
                continue;
            }
        };
        let name = match iter.next() {
            Some(value) => value,
            None => {
                println!("Invalid line[{}]: {}", processed_lines, text);
                continue;
            }
        };
        if code.chars().all(char::is_numeric) {
            all_regions.push(Region {
                code: code.to_string(),
                name: name.to_string(),
            });
        }
    }

    println!(
        "Read: {} lines, Effective: {} lines",
        processed_lines,
        all_regions.len()
    );

    println!("Starting to process...");
    processed_lines = 0;
    for region in &all_regions {
        let item = if region.code.ends_with("0000") {
            Region {
                code: region.code.clone(),
                name: region.name.clone(),
            }
        } else if region.code.ends_with("00") {
            let province_name =
                find_province_name_by_code(&all_regions, &region.code).unwrap_or_default();
            Region {
                code: region.code.clone(),
                name: format!("{}{}{}", province_name, args.delimiter, region.name),
            }
        } else {
            let province_name =
                find_province_name_by_code(&all_regions, &region.code).unwrap_or_default();
            let city_name = find_city_name_by_code(&all_regions, &region.code).unwrap_or("");
            Region {
                code: region.code.clone(),
                name: format!(
                    "{}{}{}{}{}",
                    province_name, args.delimiter, city_name, args.delimiter, region.name
                ),
            }
        };
        processed_lines += 1;

        if args.verbose {
            println!("{} => {} : {}", processed_lines, &item.code, &item.name);
        }
        processed_results.push(item);
    }

    println!("Processed: {} lines", processed_lines);

    let output_path = if args.output.is_empty() {
        Path::new("./")
    } else {
        Path::new(&args.output)
    };

    let output_dir = if output_path.is_dir() {
        output_path
    } else {
        match output_path.parent() {
            Some(p) => {
                if p.to_str().unwrap().is_empty() {
                    Path::new("./")
                } else {
                    p
                }
            }
            None => Path::new("./"),
        }
    };

    if !output_dir.exists() {
        create_dir_all(output_dir)?;
    }

    let output_file_name = if output_path.is_dir() {
        let input_path = Path::new(&args.file);
        let mut builder = String::from("_");
        builder.push_str(input_path.file_name().unwrap().to_str().unwrap());
        output_path.join(&builder).to_str().unwrap().to_owned()
    } else {
        args.output.clone()
    };

    if args.verbose {
        println!("Output dir: {:?}", output_dir);
        println!("Output file: {}", output_file_name);
    }
    println!("Starting to write to {}", output_file_name);
    let mut buffer = File::create(&output_file_name)?;
    for item in &processed_results {
        buffer.write_all(format!("{},{}\n", &item.code, &item.name).as_bytes())?;
    }

    println!("Completed!");

    Ok(())
}

fn find_province_name_by_code<'a>(regions: &'a Vec<Region>, code: &str) -> Option<&'a str> {
    let provinces: Vec<_> = regions
        .iter()
        .filter(|x| x.code != code && x.code.starts_with(&code[0..2]) && x.code.ends_with("0000"))
        .collect();
    if !provinces.is_empty() {
        Some(&provinces[0].name)
    } else {
        None
    }
}

fn find_city_name_by_code<'a>(regions: &'a Vec<Region>, code: &str) -> Option<&'a str> {
    let cities: Vec<_> = regions
        .iter()
        .filter(|x| x.code != code && x.code.starts_with(&code[0..4]) && x.code.ends_with("00"))
        .collect();
    if !cities.is_empty() {
        Some(&cities[0].name)
    } else {
        None
    }
}
