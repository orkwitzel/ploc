use clap::Parser;

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
    let _cli = Cli::parse();
    println!("ploc");
}
