# GStreamer Example - Mic (capturing audio from microphone)

```sh
gst-launch-1.0 -v -m alsasrc device=sysdefault:CARD=Device ! queue ! audioconvert ! wavenc ! filesink location=out.wav
```

<https://gstreamer.freedesktop.org/documentation/alsa/alsasrc.html>

<https://gstreamer.freedesktop.org/documentation/audioconvert/index.html>

<https://gstreamer.freedesktop.org/documentation/wavenc/index.html>

<https://gstreamer.freedesktop.org/documentation/coreelements/filesink.html>

## GStreamer setup

```sh
# https://gstreamer.freedesktop.org/documentation/rust/stable/latest/docs/gstreamer/#installation-linux
sudo apt install -y \
--no-install-recommends \
libgstreamer1.0-dev \
libgstreamer-plugins-base1.0-dev \
gstreamer1.0-plugins-base \
gstreamer1.0-plugins-good \
gstreamer1.0-plugins-bad \
gstreamer1.0-plugins-ugly \
gstreamer1.0-libav \
libgstrtspserver-1.0-dev \
libges-1.0-dev \
gstreamer1.0-tools \
gstreamer1.0-alsa \
alsa-utils

arecord --list-devices

# [Output]
# **** List of CAPTURE Hardware Devices ****
# card 0: Device [USB PnP Sound Device], device 0: USB Audio [USB Audio]
#   Subdevices: 1/1
#   Subdevice #0: subdevice #0

gst-launch-1.0 -v alsasrc device=sysdefault:CARD=Device ! queue ! audioconvert ! wavenc ! filesink location=out.wav

# [Output -> out.wav file]
# Setting pipeline to PAUSED ...
# Pipeline is live and does not need PREROLL ...
# Pipeline is PREROLLED ...
# Setting pipeline to PLAYING ...
# New clock: GstAudioSrcClock
# /GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-buffer-time = 191836
# /GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-latency-time = 21315
# Redistribute latency...
# /GstPipeline:pipeline0/GstAlsaSrc:alsasrc0.GstPad:src: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:src: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstWavEnc:wavenc0.GstPad:sink: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:sink: caps = audio/x-raw, format=(string)S16LE, layout=(string)interleaved, rate=(int)44100, channels=(int)2, channel-mask=(bitmask)0x0000000000000003
# /GstPipeline:pipeline0/GstWavEnc:wavenc0.GstPad:src: caps = audio/x-wav
# /GstPipeline:pipeline0/GstFileSink:filesink0.GstPad:sink: caps = audio/x-wav
# Redistribute latency...
# ^Chandling interrupt.
# Interrupt: Stopping pipeline ...
# Execution ended after 0:00:08.242091700
# Setting pipeline to NULL ...
# Freeing pipeline ...

file -b out.wav

# RIFF (little-endian) data, WAVE audio, Microsoft PCM, 16 bit, stereo 44100 Hz

aplay -D sysdefault:CARD=UACDemoV10 --format=S16_LE --rate=44100 out.wav
```

## Builds

```sh
# Dev build

cargo build

file -b ../../target/debug/gst-mic

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=1927503304e70bf0874835777e2b97640b2bdb99, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/gst-mic

# -rwxr-xr-x 2 cavani cavani 34M Mar  1 10:23 ../../target/debug/gst-mic


# Release build

cargo build --release

file -b ../../target/release/gst-mic

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=05f928328d56ff8500efe1ea79edb701fa459638, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/gst-mic

# -rwxr-xr-x 2 cavani cavani 1.6M Mar  1 10:23 ../../target/release/gst-mic


# LTO build

cargo build --profile release-lto

file -b ../../target/release-lto/gst-mic

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=4893dab7adfe74ba4b0b648d857eae253fcc59c6, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/gst-mic

# -rwxr-xr-x 2 cavani cavani 899K Mar  1 10:21 ../../target/release-lto/gst-mic

ldd ../../target/release-lto/gst-mic

# linux-vdso.so.1 (0x00007fff2bfe8000)
# libgstreamer-1.0.so.0 => /lib/aarch64-linux-gnu/libgstreamer-1.0.so.0 (0x00007fff2bd20000)
# libgobject-2.0.so.0 => /lib/aarch64-linux-gnu/libgobject-2.0.so.0 (0x00007fff2bca0000)
# libglib-2.0.so.0 => /lib/aarch64-linux-gnu/libglib-2.0.so.0 (0x00007fff2bb40000)
# libgio-2.0.so.0 => /lib/aarch64-linux-gnu/libgio-2.0.so.0 (0x00007fff2b930000)
# libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x00007fff2b8f0000)
# libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x00007fff2b730000)
# libgmodule-2.0.so.0 => /lib/aarch64-linux-gnu/libgmodule-2.0.so.0 (0x00007fff2b700000)
# libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x00007fff2b660000)
# libunwind.so.8 => /lib/aarch64-linux-gnu/libunwind.so.8 (0x00007fff2b610000)
# libdw.so.1 => /lib/aarch64-linux-gnu/libdw.so.1 (0x00007fff2b550000)
# /lib/ld-linux-aarch64.so.1 (0x00007fff2bfb0000)
# libffi.so.8 => /lib/aarch64-linux-gnu/libffi.so.8 (0x00007fff2b520000)
# libpcre2-8.so.0 => /lib/aarch64-linux-gnu/libpcre2-8.so.0 (0x00007fff2b470000)
# libz.so.1 => /lib/aarch64-linux-gnu/libz.so.1 (0x00007fff2b430000)
# libmount.so.1 => /lib/aarch64-linux-gnu/libmount.so.1 (0x00007fff2b3a0000)
# libselinux.so.1 => /lib/aarch64-linux-gnu/libselinux.so.1 (0x00007fff2b350000)
# liblzma.so.5 => /lib/aarch64-linux-gnu/liblzma.so.5 (0x00007fff2b300000)
# libelf.so.1 => /lib/aarch64-linux-gnu/libelf.so.1 (0x00007fff2b2c0000)
# libbz2.so.1.0 => /lib/aarch64-linux-gnu/libbz2.so.1.0 (0x00007fff2b290000)
# libblkid.so.1 => /lib/aarch64-linux-gnu/libblkid.so.1 (0x00007fff2b210000)
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: gst-mic [OPTIONS] --device <DEVICE>

Options:
      --tracing              Enable tracing (generates a trace-timestamp.json file)
      --device <DEVICE>      ALSA device, as defined in an asound configuration file
      --out-file <OUT_FILE>  The output file using the wav format [default: out.wav]
  -h, --help                 Print help
  -V, --version              Print version
```

## Running

```sh
# arecord --list-devices
cargo run --profile release-lto -- \
--device=sysdefault:CARD=Device \
--out-file out.wav

aplay -D sysdefault:CARD=UACDemoV10 --format=S16_LE --rate=44100 out.wav
```

Output.

```text
Pipeline state changed from Null to Ready
Pipeline state changed from Ready to Paused
Pipeline state changed from Paused to Playing
^C
```

## Code

### Original Code

<https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/tutorials/src/bin/basic-tutorial-3.rs>

<https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/audio_multichannel_interleave.rs>

<https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/appsink.rs>

### Dependencies

```sh
# Raspberry Pi OS Lite (November 19th 2024) -> Debian 12 (bookworm) -> libgstreamer1.0-dev 1.22.0-2+deb12u1

cargo add \
anyhow \
clap \
gstreamer \
tracing \
tracing-chrome \
tracing-subscriber \
--features \
anyhow/backtrace,\
clap/derive,\
gstreamer/v1_22
```
