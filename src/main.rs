mod cli;
mod io_files;
mod ollama;

use anyhow::Result;
use clap::Parser;

use crate::io_files::{detect_lang, resolve_output, target_lang};

const SYSTEM_PROMPT: &str =
    "次の文章の言語を判断し、日本語の場合は英語、英語の場合は日本語に変換する。回答は変換後の文章のみとする。\n";

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let input_text = io_files::read_input(&args.input)?;
    let target = target_lang(detect_lang(&input_text));
    let out_path = resolve_output(&args.input, args.output.as_deref(), target);

    let prompt = format!("{SYSTEM_PROMPT}{input_text}");
    let response = ollama::translate(&args.host, &args.model, &prompt).await?;
    io_files::write_output(&out_path, &response, args.overwrite)?;

    Ok(())
}
