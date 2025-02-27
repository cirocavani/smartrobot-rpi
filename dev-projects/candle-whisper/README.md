# Candle Example - Whisper

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/whisper>

<https://huggingface.co/collections/openai/whisper-release-6501bba2cf999715fd953013>

<https://huggingface.co/openai/whisper-large-v3-turbo>

<https://huggingface.co/openai/whisper-large-v3>

<https://huggingface.co/distil-whisper>

## Builds

```sh
# Dev build

cargo build

file ../../target/debug/candle-whisper

# ../../target/debug/candle-whisper: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=78855e489613457b06aca14124a565a9ebc24f2a, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/candle-whisper

# -rwxr-xr-x 2 cavani cavani 211M Feb 27 08:48 ../../target/debug/candle-whisper


# Release build

cargo build --release

file ../../target/release/candle-whisper

# ../../target/release/candle-whisper: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=5738c15c0f3826f9ed78ac5006f5e66abc5d917e, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/candle-whisper

# -rwxr-xr-x 2 cavani cavani 11M Feb 27 08:49 ../../target/release/candle-whisper


# LTO build

cargo build --profile release-lto

file ../../target/release-lto/candle-whisper

# ../../target/release-lto/candle-whisper: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=6eb79fe3059e1e040a01478f88604663186bb548, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/candle-whisper

# -rwxr-xr-x 2 cavani cavani 6.9M Feb 27 08:53 ../../target/release-lto/candle-whisper
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: candle-whisper [OPTIONS]

Options:
      --cpu                  Run on CPU rather than on GPU
      --model-id <MODEL_ID>
      --revision <REVISION>  The model to use, check out available models: https://huggingface.co/models?search=whisper
      --model <MODEL>        The model to be used, can be tiny, small, medium [default: tiny.en] [possible values: tiny, tiny.en, base, base.en, small, small.en, medium, medium.en, large, large-v2, large-v3, large-v3-turbo, distil-medium.en, distil-large-v2, distil-large-v3]
      --input <INPUT>        The input to be processed, in wav format, will default to `jfk.wav`. Alternatively this can be set to sample:jfk, sample:gb1, ... to fetch a sample from the following repo: https://huggingface.co/datasets/Narsil/candle_demo/
      --seed <SEED>          The seed to use when generating random samples [default: 299792458]
      --tracing              Enable tracing (generates a trace-timestamp.json file)
      --quantized
      --language <LANGUAGE>  Language
      --task <TASK>          Task, when no task is specified, the input tokens contain only the sot token which can improve things when in no-timestamp mode [possible values: transcribe, translate]
      --timestamps           Timestamps mode, this is not fully implemented yet
      --verbose              Print the full DecodingResult structure rather than just the text
  -h, --help                 Print help
  -V, --version              Print version
```

## Prompting

<https://huggingface.co/openai/whisper-large-v3-turbo>

```sh
cargo run --profile release-lto -- \
--cpu \
--model large-v3-turbo \
--input assets/samples_jfk.wav
```

Output.

```text
pcm data loaded 176000
loaded mel: [1, 128, 3000]
english: 0.9238923
spanish: 0.015913116
german: 0.012802324
french: 0.01188421
portuguese: 0.008657803
0.0s -- 30.0s:  And so, my fellow Americans, ask not what your country can do for you, ask what you can do for your country.
```

## Code

### Original Code

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/whisper>

### Dependencies

- [Candle Core](https://crates.io/crates/candle-core)
- [Candle NN](https://crates.io/crates/candle-nn)
- [Candle Transformers](https://crates.io/crates/candle-transformers)
- [Tokenizers](https://crates.io/crates/tokenizers)
- [Hugging Face Hub](https://crates.io/crates/hf-hub)

```sh
cargo add \
anyhow \
byteorder \
candle-core \
candle-nn \
candle-transformers \
clap \
hf-hub \
rand \
serde \
serde_json \
symphonia \
tracing \
tracing-chrome \
tracing-subscriber \
--features \
anyhow/backtrace,\
clap/derive,\
hf-hub/tokio,\
serde/derive,\
symphonia/all

cargo add \
tokenizers \
--no-default-features \
--features \
onig
```

### Raspberry Pi build error

**gemm_f16: Build fails in debug mode for AArch64**

<https://github.com/sarah-quinones/gemm/issues/31#issuecomment-2254635277>

[`.cargo/config.toml`](./.cargo/config.toml)

```toml
[build]
rustflags = ["-Ctarget-feature=+fp16,+fhm", "-Ctarget-cpu=native"]
```
