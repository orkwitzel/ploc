use clap::Parser;
use ploc::{render::render, scan::scan_current_dir};

#[derive(Debug, Parser)]
#[command(version, about = "Count LOC and languages in the current directory")]
struct Cli {
    /// Include dependency, build, VCS, cache, and generated-output directories.
    #[arg(long)]
    include_noise: bool,

    /// Disable ANSI color.
    #[arg(long)]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();

    match scan_current_dir(cli.include_noise) {
        Ok(summary) => print!("{}", render(&summary, !cli.no_color)),
        Err(message) => {
            eprintln!("ploc: {message}");
            std::process::exit(1);
        }
    }
}
