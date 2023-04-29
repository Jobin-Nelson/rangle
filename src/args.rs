use clap::Parser;

/// Simple org-tangle alternative built in rust
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct RangleArgs {
    /// File to tangle
    #[arg(short, long)]
    pub file: String,
}
