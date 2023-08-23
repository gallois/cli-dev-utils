mod dev_utils;

use clap::{command, Args, Parser, Subcommand};

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
struct HashArgs {
    #[command(subcommand)]
    hash_type: HashSubcommand,
}

#[derive(Subcommand)]
enum HashSubcommand {
    Md5 {
        #[arg(short, long)]
        content: String,
    },
    Sha256 {
        #[arg(short, long)]
        content: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Hash(contents) => match contents.hash_type {
            HashSubcommand::Md5 { content } => println!("{}", dev_utils::hash::md5(&content)),
            HashSubcommand::Sha256 { content } => println!("{}", dev_utils::hash::sha256(&content)),
        },
    }
}
