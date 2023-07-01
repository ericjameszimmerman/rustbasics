use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

mod cmd;

/// The Software Engineer's Toolbox (SWET)
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "swet")]
#[command(about = "The Software Engineer's Toolbox (SWET)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Base Conversion
    #[command(arg_required_else_help = true)]
    Baseconv {
        /// The value to convert
        value: String,
    },
    
    /// pushes things
    #[command(arg_required_else_help = true)]
    Push {
        /// The remote to target
        remote: String,
    },
    /// adds things
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    Stash(StashArgs),
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommands>,

    #[command(flatten)]
    push: StashPushArgs,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
}

#[derive(Debug, Args)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Baseconv { value } => {
            cmd::baseconv::execute(value);
        }
        Commands::Push { remote } => {
            println!("Pushing to {remote}");
        }
        Commands::Add { path } => {
            println!("Adding {path:?}");
        }
        Commands::Stash(stash) => {
            let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
            match stash_cmd {
                StashCommands::Push(push) => {
                    println!("Pushing {push:?}");
                }
                StashCommands::Pop { stash } => {
                    println!("Popping {stash:?}");
                }
                StashCommands::Apply { stash } => {
                    println!("Applying {stash:?}");
                }
            }
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }

    // Continued program logic goes here...
}
