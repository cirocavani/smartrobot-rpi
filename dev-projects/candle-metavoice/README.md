# Candle Example - MetaVoice

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/metavoice>

<https://huggingface.co/metavoiceio/metavoice-1B-v0.1>

## Builds

```sh
# Dev build

cargo build

file ../../target/debug/candle-metavoice

# ../../target/debug/candle-metavoice: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=a24e15ed4b4d1e5f33cde7fda13294b758ac9631, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/candle-metavoice

# -rwxr-xr-x 2 cavani cavani 197M Feb 27 12:02 ../../target/debug/candle-metavoice


# Release build

cargo build --release

file ../../target/release/candle-metavoice

# ../../target/release/candle-metavoice: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=e3bb62784943e480829a306804d99a8c10ddd27d, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/candle-metavoice

# -rwxr-xr-x 2 cavani cavani 9.8M Feb 27 12:11 ../../target/release/candle-metavoice


# LTO build

cargo build --profile release-lto

file ../../target/release-lto/candle-metavoice

# ../../target/release-lto/candle-metavoice: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=e30d805341890ecfd5beae1336990d728e9a1929, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/candle-metavoice

# -rwxr-xr-x 2 cavani cavani 5.9M Feb 27 12:17 ../../target/release-lto/candle-metavoice
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: candle-metavoice [OPTIONS] --prompt <PROMPT>

Options:
      --cpu
          Run on CPU rather than on GPU
      --tracing
          Enable tracing (generates a trace-timestamp.json file)
      --prompt <PROMPT>

      --quantized
          Use the quantized version of the model
      --guidance-scale <GUIDANCE_SCALE>
          The guidance scale [default: 3]
      --temperature <TEMPERATURE>
          The temperature used to generate samples [default: 1]
      --seed <SEED>
          The seed to use when generating random samples [default: 299792458]
      --max-tokens <MAX_TOKENS>
          The maximum number of tokens to generate for the first stage [default: 2000]
      --out-file <OUT_FILE>
          The output file using the wav format [default: out.wav]
      --first-stage-meta <FIRST_STAGE_META>

      --first-stage-weights <FIRST_STAGE_WEIGHTS>

      --second-stage-weights <SECOND_STAGE_WEIGHTS>

       <ENCODEC_WEIGHTS>

      --spk-emb <SPK_EMB>

      --dtype <DTYPE>
          [default: f32] [possible values: f32, f16, bf16]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Download Models

> Only required if hf-hub does not work (model download is done at the first execution).

```sh
python -m venv .hf-hub
source .hf-hub/bin/activate
pip install --upgrade huggingface-hub

huggingface-cli download lmz/candle-metavoice
huggingface-cli download facebook/encodec_24khz

deacivate
```

## Prompting

> Only quantized model works on RPi.
>
> Default model returns `Error: A weight is invalid in distribution`

<https://huggingface.co/metavoiceio/metavoice-1B-v0.1>

<https://huggingface.co/lmz/candle-metavoice>

```sh
cargo run --profile release-lto -- \
--cpu \
--quantized \
--prompt "This is a demo of text to speech by MetaVoice-1B, an open-source foundational audio model."

ls -alh out.wav

# -rw-r--r-- 1 cavani cavani 290K Feb 27 15:17 out.wav

aplay -D sysdefault:CARD=UACDemoV10 --format=S16_LE --rate=16000 out.wav
```

Output.

[WAV file](./output/out.wav) (Signed 16 bit Little Endian, Rate 24000 Hz, Mono)

```text
avx: false, neon: true, simd128: false, f16c: false
prompt: 'This is a demo of text to speech by MetaVoice-1B, an open-source foundational audio model.'
[2133, 2153, 2320, 2388, 2307, 2434, 2158, 2160, 2328, 2305, 2150, 2169, 2165, 2327, 2311, 2456, 2150, 2419, 2452, 2428, 2377, 2146, 2135, 2160, 2355, 2150, 2094, 2098, 2115, 2093, 2399, 2313, 2161, 2325, 2094, 2164, 2483, 2374, 2323, 2514, 2487, 2380, 2307, 2166, 2149, 2154, 2160, 2321, 2160, 2149, 2150, 2157, 2095, 2561]
.............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................
text ids len: 55
sampling from logits...
codes: [[[1109, 1129, 1296, ..., 1024, 1024, 1024],
  [1024, 1024, 1024, ..., 1024, 1024, 1024],
  [1024, 1024, 1024, ..., 1024, 1024, 1024],
  ...
  [1024, 1024, 1024, ..., 1024, 1024, 1024],
  [1024, 1024, 1024, ..., 1024, 1024, 1024],
  [1024, 1024, 1024, ..., 1024, 1024, 1024]]]
Tensor[[1, 8, 1024], u32]
text_ids len: 54
audio_ids shape: [1, 8, 462]
output pcm shape: [1, 1, 148210]
```

## Project

### Original Code

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/metavoice>

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
rand \
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
