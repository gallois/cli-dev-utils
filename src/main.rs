mod dev_utils;

use clap::{command, Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

use dev_utils::base64::B64Action;
use dev_utils::colour::Colour;
use dev_utils::convert::Conversion;
use dev_utils::date::DateAction;
use dev_utils::datetime::DateTimeFormat;
use dev_utils::hash::HashType;
use dev_utils::list::ListAction;
use dev_utils::url::UrlAction;
use dev_utils::CliError;

use std::process::exit;
use std::str::FromStr;

#[derive(Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value_t = false)]
    editor: bool,
}

#[derive(Clone, Subcommand)]
enum Commands {
    Hash(HashArgs),
    Url(URLArgs),
    Base64(B64Args),
    Convert(ConversionArgs),
    Datetime(DateTimeArgs),
    Date(DateArgs),
    List(ListArgs),
    #[clap(visible_alias = "color")]
    Colour(ColourArgs),
}

#[derive(Args, Clone)]
#[command(about = format!("Available hash types: {}", dev_utils::enum_variants::<HashType>()))]
pub struct HashArgs {
    hash_type: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<UrlAction>()))]
pub struct URLArgs {
    action: String,
    url: Option<MaybeStdin<String>>,
    // TODO Add support for building the url from a struct, like https://docs.rs/serde_qs/latest/serde_qs/
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<B64Action>()))]
pub struct B64Args {
    action: String,
    data: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Conversion>()))]
pub struct ConversionArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available formats: {}", dev_utils::enum_variants::<DateTimeFormat>()))]
pub struct DateTimeArgs {
    from: String,
    to: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<ListAction>()))]
struct ListArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
    #[arg(short, long, default_value = ",")]
    separator: String,
}
#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Colour>()))]
struct ColourArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<DateAction>()))]
struct DateArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

fn handle_cli_error(e: CliError) {
    match e {
        CliError::NoDataProvided => {
            eprintln!("No data provided");
            exit(exitcode::USAGE);
        }
        CliError::EditorError => {
            eprintln!("Error while using editor");
            exit(exitcode::SOFTWARE)
        }
        CliError::InvalidArgs(message) => {
            eprintln!("{}", message);
            exit(exitcode::USAGE);
        }
        CliError::UrlError(message) => {
            eprintln!("{}", message);
            exit(exitcode::DATAERR);
        }
        CliError::B64Error(e) => {
            eprintln!("Error while decoding base64: {}", e);
            exit(exitcode::DATAERR);
        }
        CliError::ConversionError(e) => {
            eprintln!("Error while converting: {}", e);
            exit(exitcode::DATAERR);
        }
        CliError::DateTimeError(e) => {
            eprintln!("Error while processing date time: {}", e);
            exit(exitcode::DATAERR);
        }
    }
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Hash(ref hash_args) => {
            match dev_utils::command_matchers::hash(hash_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Url(ref url_encode_args) => {
            match dev_utils::command_matchers::url(url_encode_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Base64(ref b64_encode_args) => {
            match dev_utils::command_matchers::base64(b64_encode_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Convert(ref convert_args) => {
            match dev_utils::command_matchers::conversion(convert_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Datetime(ref date_time_args) => {
            match dev_utils::command_matchers::date_time(date_time_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Date(date_args) => {
            let action = match DateAction::from_str(&date_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<DateAction>()
                    );
                    exit(exitcode::USAGE);
                }
            };
            let content = match dev_utils::get_content(date_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => return handle_cli_error(e),
            };
            let content_str = content.as_str();

            match action {
                DateAction::Delta => match dev_utils::date::delta(content_str, -1) {
                    Ok(result) => println!("{}", result),
                    Err(e) => {
                        eprintln!("Error while converting date: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
            }
        }
        Commands::List(list_args) => {
            let action = match ListAction::from_str(&list_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<ListAction>()
                    );
                    exit(exitcode::USAGE);
                }
            };
            let content = match dev_utils::get_content(list_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => return handle_cli_error(e),
            };
            let content_str = content.as_str();
            let separator = list_args.separator.as_str();

            match action {
                ListAction::Sort => {
                    println!("{}", dev_utils::list::sort(content_str, separator))
                }
                ListAction::Lowercase => {
                    println!("{}", dev_utils::list::lowercase(content_str, separator))
                }
                ListAction::Uppercase => {
                    println!("{}", dev_utils::list::uppercase(content_str, separator))
                }
                ListAction::Capitalise | ListAction::Capitalize => {
                    println!("{}", dev_utils::list::capitalise(content_str, separator))
                }
                ListAction::Reverse => {
                    println!("{}", dev_utils::list::reverse(content_str, separator))
                }
            }
        }
        Commands::Colour(colour_args) => {
            let action = match Colour::from_str(&colour_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<Colour>()
                    );
                    exit(exitcode::USAGE);
                }
            };
            let content = match dev_utils::get_content(colour_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => return handle_cli_error(e),
            };
            let content_str = content.as_str();

            match action {
                Colour::Hex2Rgb => match dev_utils::colour::hex2rgb(content_str) {
                    Ok(rgb) => println!("{}", rgb),
                    Err(e) => {
                        eprintln!("Error while converting hex to rgb: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Colour::Hex2Hsl => match dev_utils::colour::hex2hsl(content_str) {
                    Ok(hsl) => println!("{}", hsl),
                    Err(e) => {
                        eprintln!("Error while converting hex to hsl: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Colour::Rgb2Hex => match dev_utils::colour::rgb2hex(content_str) {
                    Ok(rgb) => println!("{}", rgb),
                    Err(e) => {
                        eprintln!("Error while converting rgb to hex: {:#?}\nFormat should be `rgb(r,g,b)`", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Colour::Hsl2Hex => match dev_utils::colour::hsl2hex(content_str) {
                    Ok(hsl) => println!("{}", hsl),
                    Err(e) => {
                        eprintln!("Error while converting hsl to hex: {:#?}\nFormat should be `hsl(h,s,l)`", e);
                    }
                },
            }
        }
    }

    exit(exitcode::OK)
}
