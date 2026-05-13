mod cli;
mod io_files;
mod ollama;

use anyhow::Result;
use clap::Parser;

use crate::io_files::resolve_output;

const SYSTEM_PROMPT: &str = r#"You are a professional Japanese-English bilingual translator.

# Procedure
1. Detect the primary language of the input (Japanese or English).
2. If the '#Input message' section is Japanese, translate it into natural, fluent English.
   If the '#Input message' section is English, translate it into natural, fluent Japanese.
3. If the text is mixed, translate the whole into the less-dominant language.

# Output rules (strict)
- Output ONLY the translated text.
- Do NOT add any preamble, postscript, explanation, language label, or surrounding quotation marks.
- Preserve line breaks, bullet points, code blocks, URLs, numbers, proper nouns, and original spellings of technical terms.
- Preserve the tone of the source (formal / casual) and the level of politeness (e.g., Japanese keigo).
- Use punctuation conventions of the target language (Japanese: 「」、。 / English: " " , .).

# Input message

---

<FILETEXT>

---
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let input_text = io_files::read_input(&args.input)?;
    let out_path = resolve_output(&args.input, args.output.as_deref());

    let prompt = SYSTEM_PROMPT.replace("<FILETEXT>", &input_text);
    let response = ollama::translate(&args.host, &args.model, &prompt).await?;
    io_files::write_output(&out_path, &response, args.overwrite)?;

    Ok(())
}
