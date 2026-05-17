mod cli;
mod io_files;
mod ollama;

use anyhow::Result;
use clap::Parser;

use crate::io_files::resolve_output;

const SYSTEM_PROMPT_EJ: &str = r#"You are a professional English-to-Japanese translator.

# Procedure
1. Translate the input English text into natural, fluent Japanese.

# Output rules (strict)
- Output ONLY the translated Japanese text.
- Do NOT add any preamble, postscript, explanation, language label, or surrounding quotation marks.
- Preserve line breaks, bullet points, code blocks, URLs, numbers, proper nouns, and original spellings of technical terms.
- Preserve the tone of the source (formal / casual) and the level of politeness.
- Use Japanese punctuation conventions (「」、。).
"#;

const SYSTEM_PROMPT_JE: &str = r#"You are a professional Japanese-to-English translator.

# Procedure
1. Translate the input Japanese text into natural, fluent English.

# Output rules (strict)
- Output ONLY the translated English text.
- Do NOT add any preamble, postscript, explanation, language label, or surrounding quotation marks.
- Preserve line breaks, bullet points, code blocks, URLs, numbers, proper nouns, and original spellings of technical terms.
- Preserve the tone of the source (formal / casual) and the level of politeness (e.g., Japanese keigo).
- Use English punctuation conventions (" " , .).
"#;

fn system_prompt(direction: cli::Direction) -> &'static str {
    match direction {
        cli::Direction::Ej => SYSTEM_PROMPT_EJ,
        cli::Direction::Je => SYSTEM_PROMPT_JE,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let input_text = io_files::read_input(&args.input)?;
    let out_path = resolve_output(&args.input, args.output.as_deref());

    let sys = system_prompt(args.direction());
    let response = ollama::translate(&args.host, &args.model, sys, &input_text).await?;
    io_files::write_output(&out_path, &response, args.overwrite)?;

    Ok(())
}
