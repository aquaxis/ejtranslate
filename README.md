# ejtranslate

A small Rust CLI that translates between **English and Japanese** using a local [Ollama](https://ollama.com) server. The source language is detected from the input file; the model translates into the opposite language.

## Prerequisites

- A Rust toolchain (stable, 1.75+).
- A running local Ollama server (`ollama serve`).
- The translation model pulled locally:

  ```bash
  ollama pull translategemma:12b
  ```

  The smaller `translategemma:4b` does not reliably follow the Japanese instruction prompt on Japanese input; the 12B variant is the default.

## Install

One-liner (uses `cargo install` to build from source):

```bash
curl -fsSL https://raw.githubusercontent.com/<owner>/ejtranslate/main/install.sh | sh
```

Or directly:

```bash
cargo install --git https://github.com/<owner>/ejtranslate ejtranslate
```

Or from a local checkout:

```bash
git clone https://github.com/<owner>/ejtranslate.git
cd ejtranslate
cargo install --path .
```

## Usage

```
ejtranslate [OPTIONS] <INPUT> [OUTPUT]
```

| Argument / Flag | Default | Description |
|-----------------|---------|-------------|
| `<INPUT>`       | —       | Path of the input text file (positional, required). |
| `[OUTPUT]`      | derived | Path of the output text file (positional, optional). When omitted: `<input_stem>_<target_lang>.md` (e.g. `notes.md` → `notes_en.md` if the input is Japanese). |
| `-m`, `--model` | `translategemma:12b` | Ollama model name. |
| `-H`, `--host`  | `http://localhost:11434` (or `$OLLAMA_HOST`) | Ollama base URL. |
| `-w`, `--overwrite` | `false` | Replace the output file if it already exists. |

### Examples

Translate a Japanese file, write to the derived path `notes_en.md`:

```bash
ejtranslate notes.md
```

Translate an English file with an explicit output path:

```bash
ejtranslate hello.md hello.ja.md
```

Re-run and overwrite the existing output:

```bash
ejtranslate notes.md --overwrite
```

Point at a remote Ollama server:

```bash
OLLAMA_HOST=http://my-ollama:11434 ejtranslate notes.md
```

## How it works

1. The input file is read as UTF-8.
2. The source language is detected locally by scanning for Hiragana, Katakana, CJK Unified Ideographs, or the Katakana-Hiragana Prolonged Sound Mark. Anything else is treated as English.
3. The fixed Japanese system prompt is prepended to the input and sent to Ollama's `/api/generate` endpoint with `stream: false`.
4. The model's `response` field is written verbatim to the resolved output path.

## License

[MIT](./LISENCE.md). Copyright © 2026.
