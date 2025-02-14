mod formatting;
mod state;

use clap::{ArgAction, Parser, Subcommand};
use formatting::{print_fancy, print_i3status, print_plain, OutputFormat};
use state::{load_state_from_disk, State};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// CLI sub commands
#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(short, long)]
        output_format: Option<OutputFormat>,
        // NOTE: see https://jwodder.github.io/kbits/posts/clap-bool-negate/
        #[arg(short, long, action = ArgAction::SetTrue, overrides_with = "_no_cache")]
        cache: Option<bool>,
        #[arg(short = 'n', long = "no-cache", action = ArgAction::SetFalse)]
        _no_cache: Option<bool>,
        #[arg(short = 'U', long, conflicts_with = "_no_cache", action = ArgAction::SetFalse)]
        no_update: Option<bool>,
        #[arg(short, long)]
        quiet: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check {
            output_format,
            cache: _,
            _no_cache,
            no_update: _,
            quiet: _,
        } => {
            // TODO: Load config
            let state: State = load_state_from_disk();

            match output_format {
                &Some(OutputFormat::I3status) => {
                    print_i3status(state);
                }
                &Some(OutputFormat::Fancy) => {
                    print_fancy(state);
                }
                &Some(OutputFormat::Plain) => {
                    print_plain(state);
                }
                None => {
                    print_plain(state);
                }
            };
        }
    }
}
