use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ja,
    En,
}

impl Lang {
    pub fn two_letter(self) -> &'static str {
        match self {
            Lang::Ja => "ja",
            Lang::En => "en",
        }
    }
}

pub fn detect_lang(input: &str) -> Lang {
    for c in input.chars() {
        let cp = c as u32;
        let is_ja = matches!(cp,
            0x3040..=0x309F  // Hiragana
            | 0x30A0..=0x30FF  // Katakana (includes Prolonged Sound Mark U+30FC)
            | 0x4E00..=0x9FFF  // CJK Unified Ideographs
        );
        if is_ja {
            return Lang::Ja;
        }
    }
    Lang::En
}

pub fn target_lang(src: Lang) -> Lang {
    match src {
        Lang::Ja => Lang::En,
        Lang::En => Lang::Ja,
    }
}

pub fn resolve_output(input: &Path, explicit: Option<&Path>, target: Lang) -> PathBuf {
    if let Some(p) = explicit {
        return p.to_path_buf();
    }
    let stem = input
        .file_stem()
        .map(|s| s.to_os_string())
        .unwrap_or_else(|| OsString::from("output"));
    let mut name = stem;
    name.push("_");
    name.push(target.two_letter());
    name.push(".md");
    match input.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.join(name),
        _ => PathBuf::from(name),
    }
}

pub fn read_input(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("failed to read input file: {}", path.display()))
}

pub fn write_output(path: &Path, body: &str, overwrite: bool) -> Result<()> {
    if path.exists() && !overwrite {
        bail!(
            "output file already exists: {}; pass --overwrite to replace",
            path.display()
        );
    }
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .with_context(|| format!("failed to open output file: {}", path.display()))?;
    file.write_all(body.as_bytes())
        .with_context(|| format!("failed to write output file: {}", path.display()))?;
    Ok(())
}
