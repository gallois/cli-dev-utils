mod dev_utils;

use clap::{command, Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

use dev_utils::convert::Conversion;
use dev_utils::hash::HashType;
use dev_utils::url::UrlAction;

use std::process::exit;
use std::str::FromStr;

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

    match args.command {
        Commands::Hash(hash_args) => {
            let hash_type = match HashType::from_str(&hash_args.hash_type) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid hash type. Valid hash types are: {}",
                        dev_utils::enum_variants::<HashType>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            let content = match dev_utils::get_content(hash_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => match e {
                    dev_utils::CliError::NoDataProvided => {
                        eprintln!("No data provided");
                        exit(exitcode::USAGE);
                    }
                    dev_utils::CliError::EditorError => {
                        eprintln!("Error while using editor");
                        exit(exitcode::SOFTWARE)
                    }
                },
            };
            let content_str = content.as_str();

            match hash_type {
                HashType::Md5 => println!("{}", dev_utils::hash::md5(content_str)),
                HashType::Sha256 => println!("{}", dev_utils::hash::sha256(content_str)),
                HashType::Sha512 => println!("{}", dev_utils::hash::sha512(content_str)),
            }
        }
        Commands::Url(url_encode_args) => {
            let action = match UrlAction::from_str(&url_encode_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<UrlAction>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            let url = match dev_utils::get_content(url_encode_args.url, args.editor) {
                Ok(c) => c,
                Err(e) => match e {
                    dev_utils::CliError::NoDataProvided => {
                        eprintln!("No data provided");
                        exit(exitcode::USAGE);
                    }
                    dev_utils::CliError::EditorError => {
                        eprintln!("Error while using editor");
                        exit(exitcode::SOFTWARE);
                    }
                },
            };
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
            let action = match Conversion::from_str(&convert_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid conversion. Valid actions are: {}",
                        dev_utils::enum_variants::<Conversion>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            let content = match dev_utils::get_content(convert_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => match e {
                    dev_utils::CliError::NoDataProvided => {
                        eprintln!("No data provided");
                        exit(exitcode::USAGE);
                    }
                    dev_utils::CliError::EditorError => {
                        eprintln!("Error while using editor");
                        exit(exitcode::SOFTWARE);
                    }
                },
            };
            let content_str = content.as_str();

            match action {
                Conversion::Json2Csv => match dev_utils::convert::json2csv(content_str) {
                    Ok(csv) => println!("{}", csv),
                    Err(e) => {
                        eprintln!("Error while converting json to csv: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Conversion::Csv2Tsv => {
                    println!("{}", dev_utils::convert::csv2tsv(content_str))
                }
            }
        }
    }

    exit(exitcode::OK)
}
