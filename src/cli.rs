use std::path::PathBuf;

use clap::Parser;

/// Translation direction (FR-12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// English → Japanese (default).
    Ej,
    /// Japanese → English.
    Je,
}

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

    /// Translate English → Japanese (default direction). Mutually
    /// exclusive with `--je`.
    #[arg(long = "ej", action = clap::ArgAction::SetTrue, conflicts_with = "je")]
    pub ej: bool,

    /// Translate Japanese → English. Mutually exclusive with `--ej`.
    #[arg(long = "je", action = clap::ArgAction::SetTrue)]
    pub je: bool,
}

impl Args {
    /// Resolve the translation direction per FR-12: `--je` selects
    /// Japanese→English; everything else (no flag, or explicit `--ej`)
    /// selects the English→Japanese default. The `--ej`/`--je` conflict
    /// is enforced by clap (`conflicts_with`), so at most one is `true`.
    pub fn direction(&self) -> Direction {
        if self.je { Direction::Je } else { Direction::Ej }
    }
}
