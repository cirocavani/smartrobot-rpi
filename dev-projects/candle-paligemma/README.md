# Candle Example - PaliGemma

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/paligemma>

<https://huggingface.co/google/paligemma-3b-mix-224>

<https://ai.google.dev/gemma/docs/paligemma>

## Builds

```sh
# Dev build

cargo build

file ../../target/debug/candle-paligemma

# ../../target/debug/candle-paligemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=d0ea2904eee531580251cfa1b53849aa3d30eb90, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/candle-paligemma

# -rwxr-xr-x 2 cavani cavani 206M Feb 26 17:50 ../../target/debug/candle-paligemma


# Release build

cargo build --release

file ../../target/release/candle-paligemma

# ../../target/release/candle-paligemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=b83b93f2aca36c6da2c4b13f2849d64e7adef55d, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/candle-paligemma

# -rwxr-xr-x 2 cavani cavani 12M Feb 26 17:56 ../../target/release/candle-paligemma


# LTO build

cargo build --profile release-lto

file ../../target/release-lto/candle-paligemma

# ../../target/release-lto/candle-paligemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=0b6367f79594d77de3e2e95e888e5f1ccb493daf, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/candle-paligemma

# -rwxr-xr-x 2 cavani cavani 7.4M Feb 26 18:02 ../../target/release-lto/candle-paligemma
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: candle-paligemma [OPTIONS] --prompt <PROMPT> --image <IMAGE>

Options:
      --cpu
          Run on CPU rather than on GPU
      --tracing
          Enable tracing (generates a trace-timestamp.json file)
      --prompt <PROMPT>

      --temperature <TEMPERATURE>
          The temperature used to generate samples
      --top-p <TOP_P>
          Nucleus sampling probability cutoff
      --seed <SEED>
          The seed to use when generating random samples [default: 299792458]
  -n, --sample-len <SAMPLE_LEN>
          The length of the sample to generate (in tokens) [default: 10000]
      --model-id <MODEL_ID>

      --revision <REVISION>
          [default: main]
      --tokenizer-file <TOKENIZER_FILE>

      --weight-files <WEIGHT_FILES>

      --repeat-penalty <REPEAT_PENALTY>
          Penalty to be applied for repeating tokens, 1. means no penalty [default: 1.1]
      --repeat-last-n <REPEAT_LAST_N>
          The context size to consider for the repeat penalty [default: 64]
      --image <IMAGE>

  -h, --help
          Print help
  -V, --version
          Print version
```

## Hugging Face Login

> **(Only required for model download at the first run)**

<https://huggingface.co/docs/hub/security-tokens>

<https://huggingface.co/settings/tokens>

```sh
# https://huggingface.co/docs/huggingface_hub/installation

python -m venv .hf-hub
source .hf-hub/bin/activate
pip install --upgrade huggingface_hub

# [Output]
# Successfully installed certifi-2025.1.31 charset-normalizer-3.4.1 filelock-3.17.0 fsspec-2025.2.0 huggingface_hub-0.29.1 idna-3.10 packaging-24.2 pyyaml-6.0.2 requests-2.32.3 tqdm-4.67.1 typing-extensions-4.12.2 urllib3-2.3.0

# https://huggingface.co/docs/huggingface_hub/guides/cli#huggingface-cli-login
# https://huggingface.co/settings/tokens

export TF_TOKEN=hf_xxxx

huggingface-cli login --token $HF_TOKEN

# [Output]
# Token is valid (permission: read).
# The token `P247` has been saved to /home/cavani/.cache/huggingface/stored_tokens
# Your token has been saved to /home/cavani/.cache/huggingface/token
# Login successful.

unset TF_TOKEN
deactivate

# FIRST RUN (see Prompting)
```

## Prompting

<https://huggingface.co/google/paligemma-3b-mix-224>

```sh
cargo run --profile release-lto -- \
--cpu \
--model-id google/paligemma-3b-mix-224 \
--prompt "caption fr" \
--image assets/bike.jpg
```

Output.

```text
avx: false, neon: true, simd128: false, f16c: false
temp: 0.00 repeat-penalty: 1.10 repeat-last-n: 64
retrieved the files in 97.477087ms
loaded image with shape Tensor[dims 1, 3, 224, 224; f16]
loaded the model in 19.116757692s
caption fr
Un groupe de cyclistes qui sont dans la rue.
12 tokens generated (0.61 token/s)
```

## Code

### Original Code

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/paligemma>

### Dependencies

- [Candle Core](https://crates.io/crates/candle-core)
- [Candle NN](https://crates.io/crates/candle-nn)
- [Candle Transformers](https://crates.io/crates/candle-transformers)
- [Tokenizers](https://crates.io/crates/tokenizers)
- [Hugging Face Hub](https://crates.io/crates/hf-hub)

```sh
cargo add \
anyhow \
candle-core \
candle-nn \
candle-transformers \
clap \
hf-hub \
serde \
serde_json \
tracing \
tracing-chrome \
tracing-subscriber \
--features \
anyhow/backtrace,\
clap/derive,\
hf-hub/tokio,\
serde/derive

cargo add \
tokenizers \
--no-default-features \
--features \
onig

cargo add \
image \
--no-default-features \
--features \
jpeg,\
png
```

### Raspberry Pi build error

**gemm_f16: Build fails in debug mode for AArch64**

<https://github.com/sarah-quinones/gemm/issues/31#issuecomment-2254635277>

[`.cargo/config.toml`](./.cargo/config.toml)

```toml
[build]
rustflags = ["-C", "target-cpu=native"]

[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "target-feature=+fp16,+fhm"]
```
