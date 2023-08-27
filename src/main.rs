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
    #[arg(short, long, default_value_t = false)]
    editor: bool,
}

#[derive(Subcommand)]
enum Commands {
    Hash(HashArgs),
}

#[derive(Args)]
#[command(about = format!("Hash type to use, {}", valid_hash_types()))]
struct HashArgs {
    hash_type: String,
    content: Option<MaybeStdin<String>>,
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

    let use_editor = args.editor;

    match args.command {
        Commands::Hash(hash_args) => {
            if !use_editor && hash_args.content.is_none() {
                eprintln!("No data provided");
                exit(exitcode::USAGE);
            }

            let hash_type = match hash_args.hash_type.as_str() {
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

            let mut content: String = String::from("");
            if let Some(hash_arg_content) = hash_args.content {
                content = hash_arg_content.as_str().to_string();
            }

            if args.editor {
                content = match edit::edit(content) {
                    Ok(content) => content.trim().to_string(),
                    Err(e) => {
                        eprintln!("Error while using editor: {}", e);
                        exit(exitcode::SOFTWARE);
                    }
                };
            }

            let content_str = content.as_str();

            match hash_type {
                HashType::Md5 => println!("{}", dev_utils::hash::md5(content_str)),
                HashType::Sha256 => println!("{}", dev_utils::hash::sha256(content_str)),
                HashType::Sha512 => println!("{}", dev_utils::hash::sha512(content_str)),
            }
        }
    }

    exit(exitcode::OK)
}
