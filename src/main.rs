mod dev_utils;

use clap::{command, Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

use dev_utils::convert::Conversion;
use dev_utils::hash::HashType;
use dev_utils::url::UrlAction;

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
    Url(URLArgs),
    Convert(ConversionArgs),
}

#[derive(Args)]
#[command(about = format!("Available hash types: {}", dev_utils::enum_variants::<HashType>()))]
struct HashArgs {
    hash_type: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<UrlAction>()))]
struct URLArgs {
    action: String,
    url: Option<MaybeStdin<String>>,
    // TODO Add support for building the url from a struct, like https://docs.rs/serde_qs/latest/serde_qs/
}

#[derive(Args)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Conversion>()))]
struct ConversionArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

fn main() {
    let args: Cli = Cli::parse();

    let use_editor = args.editor;

    match args.command {
        Commands::Hash(hash_args) => {
            let hash_type = match hash_args.hash_type.as_str() {
                "md5" => HashType::Md5,
                "sha256" => HashType::Sha256,
                "sha512" => HashType::Sha512,
                _ => {
                    eprintln!(
                        "Invalid hash type. Valid hash types are: {}",
                        dev_utils::enum_variants::<HashType>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            if !use_editor && hash_args.content.is_none() {
                eprintln!("No data provided");
                exit(exitcode::USAGE);
            }

            let mut content: String = String::new();
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
        Commands::Url(url_encode_args) => {
            let action = match url_encode_args.action.as_str() {
                "encode" => UrlAction::Encode,
                "decode" => UrlAction::Decode,
                _ => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<UrlAction>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            if !use_editor && url_encode_args.url.is_none() {
                eprintln!("No data provided");
                exit(exitcode::USAGE);
            }

            let mut url: String = String::new();
            if let Some(args_url) = url_encode_args.url {
                url = args_url.as_str().to_string();
            }

            if args.editor {
                url = match edit::edit(url) {
                    Ok(url) => url.trim().to_string(),
                    Err(e) => {
                        eprintln!("Error while using editor: {}", e);
                        exit(exitcode::SOFTWARE);
                    }
                };
            }

            let url_str = url.as_str();

            match action {
                UrlAction::Encode => {
                    println!("{}", dev_utils::url::encode(url_str))
                }
                UrlAction::Decode => match dev_utils::url::decode(url_str) {
                    Ok(decoded) => println!("{}", decoded),
                    Err(e) => {
                        eprintln!("Error while decoding url: {}", e);
                        exit(exitcode::DATAERR);
                    }
                },
            }
        }
        Commands::Convert(convert_args) => {
            // TODO refactor this
            let action = match convert_args.action.as_str() {
                "json2csv" => Conversion::Json2Csv,
                _ => {
                    eprintln!(
                        "Invalid conversion. Valid actions are: {}",
                        dev_utils::enum_variants::<Conversion>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            if !use_editor && convert_args.content.is_none() {
                eprintln!("No data provided");
                exit(exitcode::USAGE);
            }

            let mut content: String = String::new();
            if let Some(convert_arg_content) = convert_args.content {
                content = convert_arg_content.as_str().to_string();
            }

            if args.editor {
                content = match edit::edit(content) {
                    Ok(content) => content.trim().to_string(),
                    Err(e) => {
                        eprintln!("Error while using editor: {}", e);
                        exit(exitcode::SOFTWARE);
                    }
                }
            }

            match action {
                Conversion::Json2Csv => match dev_utils::convert::json2csv(content.as_str()) {
                    Ok(csv) => println!("{}", csv),
                    Err(e) => {
                        eprintln!("Error while converting json to csv: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
            }
        }
    }

    exit(exitcode::OK)
}
