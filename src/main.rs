mod formatting;
mod state;

use clap::{Parser, Subcommand};
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
    GetState {
        #[arg(short, long)]
        output_format: Option<OutputFormat>,
    },
}

fn main() {
    let cli = Cli::parse();

    let state: State = load_state_from_disk();

    match &cli.command {
        Commands::GetState { output_format } => {
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
