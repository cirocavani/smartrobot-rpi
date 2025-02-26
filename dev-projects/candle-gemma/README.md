# Candle Example - Gemma

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/gemma>

<https://huggingface.co/collections/google/gemma-2-2b-release-66a20f3796a2ff2a7c76f98f>

## Build

```sh
# dev build

cargo build

file ../../target/debug/candle-gemma

# ../../target/debug/candle-gemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=34a10a3e0fe967901008789cb5c17a0384618ec7, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/candle-gemma

# -rwxr-xr-x 2 cavani cavani 200M Feb 26 12:59 ../../target/debug/candle-gemma


# release build

cargo build --release

file ../../target/release/candle-gemma

# ../../target/release/candle-gemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=f1479b81d05ffdc4002a42023da52b9ec09c3db4, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/candle-gemma

# -rwxr-xr-x 2 cavani cavani 12M Feb 26 13:34 ../../target/release/candle-gemma

# LTO build

cargo build --profile release-lto

file ../../target/release-lto/candle-gemma

# ../../target/release-lto/candle-gemma: ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=0c175911a6adfb4dcfb4a56ebe6277dd30cdd514, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/candle-gemma

# -rwxr-xr-x 2 cavani cavani 7.0M Feb 26 14:27 ../../target/release-lto/candle-gemma
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: candle-gemma [OPTIONS] --prompt <PROMPT>

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

      --config-file <CONFIG_FILE>

      --weight-files <WEIGHT_FILES>

      --repeat-penalty <REPEAT_PENALTY>
          Penalty to be applied for repeating tokens, 1. means no penalty [default: 1.1]
      --repeat-last-n <REPEAT_LAST_N>
          The context size to consider for the repeat penalty [default: 64]
      --which <WHICH>
          The model to use [default: 2-2b] [possible values: 2b, 7b, 2b-it, 7b-it, 1.1-2b-it, 1.1-7b-it, code-2b, code-7b, code-2b-it, code-7b-it, 2-2b, 2-2b-it, 2-9b, 2-9b-it]
      --use-flash-attn

  -h, --help
          Print help
  -V, --version
```

## Prompting

<https://huggingface.co/google/gemma-2-2b-it>

Hugging Face Login.

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
```

```sh
cargo run --profile release-lto -- \
--cpu \
--which 2-2b-it \
--prompt "Here is a proof that square root of 2 is not rational: "
```

Output.

```text
avx: false, neon: true, simd128: false, f16c: false
temp: 0.00 repeat-penalty: 1.10 repeat-last-n: 64
retrieved the files in 84.55941ms
loaded the model in 11.564467323s
Here is a proof that square root of 2 is not rational:

**Proof by contradiction:**

1. **Assumption:** Assume that the square root of 2 is rational. This means it can be expressed as a fraction in its lowest terms, where the numerator and denominator are integers with no common factors other than 1.  Let's represent this as √2 = a/b, where a and b are integers and b ≠ 0.

2. **Squaring both sides:** Squaring both sides of the equation gives us 2 = a²/b².

3. **Rearranging:** Multiplying both sides by b² gives us 2b² = a².

4. **Even number:** This implies that a² is an even number (since it's equal to 2 times another integer). If a² is even, then 'a' itself must also be even (because the square of an odd number is always odd).

5. **Expressing 'a':**  Since 'a' is even, we can write it as 2k, where k is another integer.

6. **Substituting:** Substituting this value of 'a' into the equation 2b² = a², we get 2b² = (2k)² = 4k².

7. **Simplifying:** Dividing both sides by 2 gives us b² = 2k².

8. **Even number again:** This implies that b² is also an even number, meaning 'b' itself must be even.

9. **Contradiction:** We have now shown that both 'a' and 'b' are even numbers.  This contradicts our initial assumption that a/b was in its lowest terms (meaning they had no common factors).


**Conclusion:** Since our initial assumption leads to a contradiction, it must be false. Therefore, the square root of 2 cannot be expressed as a fraction in its lowest terms and is irrational.

398 tokens generated (2.07 token/s)
```

## Code

### Original Code

<https://github.com/huggingface/candle/tree/26c16923b92bddda6b05ee1993af47fb6de6ebd7/candle-examples/examples/gemma>

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
```

### Raspberry Pi build error

**gemm_f16: Build fails in debug mode for AArch64**

<https://github.com/sarah-quinones/gemm/issues/31#issuecomment-2254635277>

[`.cargo/config.toml`](./.cargo/config.toml)

```toml
[build]
rustflags = ["-Ctarget-feature=+fp16,+fhm"]
```
