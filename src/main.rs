use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use ploc::{render::render, scan::scan_current_dir};

#[derive(Debug, Parser)]
#[command(version, about = "Count LOC and languages in the current directory")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Include dependency, build, VCS, cache, and generated-output directories.
    #[arg(long)]
    include_noise: bool,

    /// Disable ANSI color.
    #[arg(long)]
    no_color: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate shell completions.
    Completions {
        /// Shell to generate completions for.
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(Command::Completions { shell }) = cli.command {
        let mut command = Cli::command();
        generate(shell, &mut command, "ploc", &mut std::io::stdout());
        return;
    }

    match scan_current_dir(cli.include_noise) {
        Ok(summary) => print!("{}", render(&summary, !cli.no_color)),
        Err(message) => {
            eprintln!("ploc: {message}");
            std::process::exit(1);
        }
    }
}
