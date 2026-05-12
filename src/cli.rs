use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "ejtranslate", about = "EN<->JA translator backed by Ollama")]
pub struct Args {
    pub input: PathBuf,

    pub output: Option<PathBuf>,

    #[arg(short = 'm', long, default_value = "translategemma:12b")]
    pub model: String,

    #[arg(
        short = 'H',
        long,
        default_value = "http://localhost:11434",
        env = "OLLAMA_HOST"
    )]
    pub host: String,

    #[arg(short = 'w', long, default_value_t = false)]
    pub overwrite: bool,
}
