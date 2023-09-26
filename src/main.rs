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
    Base64(B64Args),
    Convert(ConversionArgs),
    Datetime(DateTimeArgs),
    Date(DateArgs),
    List(ListArgs),
    #[clap(visible_alias = "color")]
    Colour(ColourArgs),
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
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<B64Action>()))]
struct B64Args {
    action: String,
    data: Option<MaybeStdin<String>>,
}

#[derive(Args)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Conversion>()))]
struct ConversionArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args)]
#[command(about = format!("Available formats: {}", dev_utils::enum_variants::<DateTimeFormat>()))]
struct DateTimeArgs {
    from: String,
    to: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<ListAction>()))]
struct ListArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
    #[arg(short, long, default_value = ",")]
    separator: String,
}
#[derive(Args)]
#[command(about = format!("Available actions: {}", dev_utils::enum_variants::<Colour>()))]
struct ColourArgs {
    action: String,
    content: Option<MaybeStdin<String>>,
}

#[derive(Args)]
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
    }
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
                Err(e) => return handle_cli_error(e),
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
                Err(e) => return handle_cli_error(e),
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
        Commands::Base64(b64_encode_args) => {
            let action = match B64Action::from_str(&b64_encode_args.action) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "Invalid action. Valid actions are: {}",
                        dev_utils::enum_variants::<B64Action>()
                    );
                    exit(exitcode::USAGE);
                }
            };

            let data = match dev_utils::get_content(b64_encode_args.data, args.editor) {
                Ok(c) => c,
                Err(e) => return handle_cli_error(e),
            };
            let data_str = data.as_str();

            match action {
                B64Action::Encode => {
                    println!("{}", dev_utils::base64::encode(data_str))
                }
                B64Action::Decode => match dev_utils::base64::decode(data_str) {
                    Ok(decoded) => println!("{}", decoded),
                    Err(e) => {
                        eprintln!("Error while decoding base64: {}", e.message);
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
                Err(e) => return handle_cli_error(e),
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
                Conversion::Json2Yaml => match dev_utils::convert::json2yaml(content_str) {
                    Ok(yaml) => println!("{}", yaml),
                    Err(e) => {
                        eprintln!("Error while converting json to yaml: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Conversion::Csv2Tsv => {
                    println!("{}", dev_utils::convert::csv2tsv(content_str))
                }
                Conversion::String2Hex => {
                    println!("{}", dev_utils::convert::string2hex(content_str))
                }
                Conversion::Hex2String => match dev_utils::convert::hex2string(content_str) {
                    Ok(data) => println!("{}", data),
                    Err(e) => {
                        eprintln!("Error while converting hex to string: {:#?}", e);
                        exit(exitcode::DATAERR);
                    }
                },
                Conversion::Text2Nato => {
                    println!("{}", dev_utils::convert::text2nato(content_str))
                }
                Conversion::Slugify => {
                    println!("{}", dev_utils::convert::slugify(content_str))
                }
                Conversion::Celsius2Fahrenheit | Conversion::C2F => {
                    match dev_utils::convert::celsius2fahrenheit(content_str) {
                        Ok(data) => println!("{}", data),
                        Err(e) => {
                            eprintln!("Error while converting hex to string: {:#?}", e);
                            exit(exitcode::DATAERR);
                        }
                    }
                }
            }
        }
        Commands::Datetime(date_time_args) => {
            let content = match dev_utils::get_content(date_time_args.content, args.editor) {
                Ok(c) => c,
                Err(e) => return handle_cli_error(e),
            };
            let content_str = content.as_str();
            match dev_utils::datetime::convert(
                &date_time_args.from,
                &date_time_args.to,
                content_str,
            ) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Error while converting datetime: {}", e.message);
                    exit(exitcode::DATAERR);
                }
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
