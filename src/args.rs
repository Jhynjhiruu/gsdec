use clap::{Parser, ValueEnum};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file
    infile: PathBuf,

    /// Output file
    outfile: Option<PathBuf>,

    /// Mode
    #[arg(short, value_enum, default_value_t = OperationMode::Dec)]
    mode: OperationMode,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum OperationMode {
    Enc,
    Dec,
}

#[derive(Debug)]
pub struct Options {
    pub infile: PathBuf,
    pub outfile: PathBuf,
    pub mode: OperationMode,
}

impl Options {
    pub fn parse() -> Self {
        let cli = Cli::parse();

        let infile = cli.infile;
        let outfile = match cli.outfile {
            Some(f) => f,
            None => infile.with_extension("dec"),
        };

        let mode = cli.mode;

        Self {
            infile,
            outfile,
            mode,
        }
    }
}
