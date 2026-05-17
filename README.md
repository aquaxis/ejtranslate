# ejtranslate

A small Rust CLI that translates between **English and Japanese** using a local [Ollama](https://ollama.com) server. The translation direction is selected explicitly with `--ej` / `--je` and defaults to English → Japanese.

## Prerequisites

- A Rust toolchain (stable, 1.75+).
- A running local Ollama server (`ollama serve`).
- At least one supported translation model pulled locally (see [Models](#models)). For the default:

  ```bash
  ollama pull translategemma:12b
  ```

## Models

The `-m` / `--model` flag accepts any Ollama model name. The documented choices are:

| Model | Notes |
|-------|-------|
| `translategemma:12b` | **Default.** Reliable in both directions. |
| `translategemma:4b`  | Smaller, faster, lower memory. Reliable for EN→JA; less reliable for JA→EN (may echo the input). Use when memory is constrained. |
| `transgemma:e4b`     | Alternative model. Pull with `ollama pull transgemma:e4b`. |

Select a non-default model with `-m`:

```bash
ejtranslate -m translategemma:4b notes.md
ejtranslate -m transgemma:e4b notes.md
```

Any other Ollama model name is also accepted; an unknown name surfaces Ollama's `404 model 'X' not found` response.

## Install

One-liner (uses `cargo install` to build from source):

```bash
curl -fsSL https://raw.githubusercontent.com/aquaxis/ejtranslate/main/install.sh | sh
```

Or directly:

```bash
cargo install --git https://github.com/aquaxis/ejtranslate ejtranslate
```

Or from a local checkout:

```bash
git clone https://github.com/aquaxis/ejtranslate.git
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
| `[OUTPUT]`      | derived | Path of the output text file (positional, optional). When omitted: `<input_stem>_output.md` (e.g. `notes.md` → `notes_output.md`). |
| `--ej`          | (default) | Translate **English → Japanese**. Used when no direction flag is given. Mutually exclusive with `--je`. |
| `--je`          | —       | Translate **Japanese → English**. Mutually exclusive with `--ej`. |
| `-m`, `--model` | `translategemma:12b` | Ollama model name. |
| `-H`, `--host`  | `http://localhost:11434` (or `$OLLAMA_HOST`) | Ollama base URL. |
| `-w`, `--overwrite` | `false` | Replace the output file if it already exists. |

Supplying both `--ej` and `--je` is a usage error (non-zero exit).

### Examples

Translate an English file to Japanese (default direction), writing to the derived path `notes_output.md`:

```bash
ejtranslate notes.md
```

Translate a Japanese file to English:

```bash
ejtranslate --je notes.md
```

Translate to Japanese with an explicit output path:

```bash
ejtranslate --ej hello.md hello.ja.md
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
2. The translation direction comes from the CLI flag (`--ej` / `--je`, default `--ej`) — the source language is **not** auto-detected.
3. The direction-specific system prompt is sent in Ollama's `system` field and the input file is sent in the `prompt` field, to `/api/generate` with `stream: false`.
4. The model's `response` field is written verbatim to the resolved output path.

## License

[MIT](./LISENCE.md). Copyright ©AQUAXIS TECHNOLOGY 2026.
