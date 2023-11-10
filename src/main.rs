mod dev_utils;

use clap::{command, Args, Parser, Subcommand};
use clap_stdin::MaybeStdin;

use dev_utils::base64::B64Action;
use dev_utils::colour::Colour;
use dev_utils::convert::Conversion;
use dev_utils::date::DateAction;
use dev_utils::datetime::DateTimeFormat;
use dev_utils::generate::GenerateSubcommands;
use dev_utils::hash::HashType;
use dev_utils::list::ListAction;
use dev_utils::percentage::PercentageAction;
use dev_utils::url::UrlAction;
use dev_utils::CliError;

use std::process::exit;

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
    Generate(GenerateArgs),
    Percentage(PercentageArgs),
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
    #[arg(short, long, default_value = "0")]
    index: usize,
    #[arg(short, long, default_value = "0")]
    length: usize,
}
#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Colour>()))]
struct ColourArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<DateAction>()))]
pub struct DateArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}
#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<GenerateSubcommands>()))]
pub struct GenerateArgs {
    #[command(subcommand)]
    type_: GenerateSubcommands,
}

#[derive(Args, Clone)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<PercentageAction>()))]
pub struct PercentageArgs {
    action: String,
    #[arg(short, long)]
    from_number: Option<MaybeStdin<f64>>,
    #[arg(short, long)]
    to_number: Option<MaybeStdin<f64>>,
    #[arg(short, long)]
    percentage: Option<MaybeStdin<f32>>,
    #[arg(long, default_value = "0")]
    precision: Option<MaybeStdin<u8>>,
    #[arg(short, long)]
    of_number: Option<MaybeStdin<f64>>,
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
        CliError::DateError(e) => {
            eprintln!("Error while processing date: {}", e);
            exit(exitcode::DATAERR);
        }
        CliError::ColourError(e) => {
            eprintln!("Error while processing colour: {}", e);
            exit(exitcode::DATAERR);
        }
        CliError::GenerateError(e) => {
            eprintln!("Error while generating: {}", e);
            exit(exitcode::DATAERR);
        }
        CliError::PercentageError(e) => {
            eprintln!("Error while processing percentage: {}", e);
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
        Commands::Date(ref date_args) => {
            match dev_utils::command_matchers::date(date_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::List(ref list_args) => {
            match dev_utils::command_matchers::list(list_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Colour(ref colour_args) => {
            match dev_utils::command_matchers::colour(colour_args.clone(), args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Generate(ref generate_args) => {
            match dev_utils::command_matchers::generate(generate_args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
        Commands::Percentage(ref percentage_args) => {
            match dev_utils::command_matchers::percentage(percentage_args.clone()) {
                Ok(s) => println!("{}", s),
                Err(e) => handle_cli_error(e),
            }
        }
    }

    exit(exitcode::OK)
}
