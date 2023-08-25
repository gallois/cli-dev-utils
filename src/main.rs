mod dev_utils;

use clap::{command, Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

use std::process::exit;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hash(HashArgs),
}

#[derive(Args)]
#[command(about = format!("Hash type to use, {}", valid_hash_types()))]
struct HashArgs {
    hash_type: String,
    content: MaybeStdin<String>,
}

fn valid_hash_types() -> String {
    HashType::iter()
        .map(|hash_type| hash_type.as_ref().to_lowercase())
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(AsRefStr, EnumString, EnumIter)]
enum HashType {
    Md5,
    Sha256,
    Sha512,
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Hash(contents) => {
            let hash_type = match contents.hash_type.as_str() {
                "md5" => HashType::Md5,
                "sha256" => HashType::Sha256,
                "sha512" => HashType::Sha512,
                _ => {
                    eprintln!(
                        "Invalid hash type. Valid hash types are: {}",
                        valid_hash_types()
                    );
                    exit(exitcode::USAGE);
                }
            };
            match hash_type {
                HashType::Md5 => println!("{}", dev_utils::hash::md5(&contents.content)),
                HashType::Sha256 => println!("{}", dev_utils::hash::sha256(&contents.content)),
                HashType::Sha512 => println!("{}", dev_utils::hash::sha512(&contents.content)),
            }
        }
    }
}
