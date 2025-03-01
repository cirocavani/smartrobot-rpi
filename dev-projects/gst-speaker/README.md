# GStreamer Example - Speaker (playing audio on speaker)

```sh
gst-launch-1.0 -v -m audiotestsrc ! volume volume=0.25 ! audioconvert ! audioresample ! alsasink device=sysdefault:CARD=UACDemoV10
```

<https://gstreamer.freedesktop.org/documentation/audiotestsrc/index.html>

<https://gstreamer.freedesktop.org/documentation/volume/index.html>

<https://gstreamer.freedesktop.org/documentation/audioconvert/index.html>

<https://gstreamer.freedesktop.org/documentation/audioresample/index.html>

<https://gstreamer.freedesktop.org/documentation/alsa/alsasink.html>

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

aplay --list-devices

# [Output]
# **** List of PLAYBACK Hardware Devices ****
# card 1: vc4hdmi0 [vc4-hdmi-0], device 0: MAI PCM i2s-hifi-0 [MAI PCM i2s-hifi-0]
#   Subdevices: 1/1
#   Subdevice #0: subdevice #0
# card 2: UACDemoV10 [UACDemoV1.0], device 0: USB Audio [USB Audio]
#   Subdevices: 1/1
#   Subdevice #0: subdevice #0
# card 3: vc4hdmi1 [vc4-hdmi-1], device 0: MAI PCM i2s-hifi-0 [MAI PCM i2s-hifi-0]
#   Subdevices: 1/1
#   Subdevice #0: subdevice #0

gst-launch-1.0 audiotestsrc ! volume volume=0.25 ! audioconvert ! audioresample ! alsasink device=sysdefault:CARD=UACDemoV10

# [Output -> speaker sound]
# Setting pipeline to PAUSED ...
# Pipeline is PREROLLING ...
# Redistribute latency...
# Pipeline is PREROLLED ...
# Setting pipeline to PLAYING ...
# Redistribute latency...
# New clock: GstAudioSinkClock
# ^Chandling interrupt.
# Interrupt: Stopping pipeline ...
# Execution ended after 0:00:05.281256687
# Setting pipeline to NULL ...
# Freeing pipeline ...
```

## Builds

```sh
# Dev build

cargo build

file -b ../../target/debug/gst-speaker

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=1fd2f946079e9c0f061ed354aa4fb84444e72589, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/gst-speaker

# -rwxr-xr-x 2 cavani cavani 34M Feb 28 13:15 ../../target/debug/gst-speaker


# Release build

cargo build --release

file -b ../../target/release/gst-speaker

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=5cd8d1d203ddecdb158ee7d7b82e0060bcca353e, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/gst-speaker

# -rwxr-xr-x 2 cavani cavani 1.6M Feb 28 13:36 ../../target/release/gst-speaker


# LTO build

cargo build --profile release-lto

file -b ../../target/release-lto/gst-speaker

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=13608ad8e59158cdc8bea1b865dd692ec22a5dcf, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/gst-speaker

# -rwxr-xr-x 2 cavani cavani 899K Feb 28 13:38 ../../target/release-lto/gst-speaker

ldd ../../target/release-lto/gst-speaker

# linux-vdso.so.1 (0x00007fff172bc000)
# libgstreamer-1.0.so.0 => /lib/aarch64-linux-gnu/libgstreamer-1.0.so.0 (0x00007fff16ff0000)
# libgobject-2.0.so.0 => /lib/aarch64-linux-gnu/libgobject-2.0.so.0 (0x00007fff16f70000)
# libglib-2.0.so.0 => /lib/aarch64-linux-gnu/libglib-2.0.so.0 (0x00007fff16e10000)
# libgio-2.0.so.0 => /lib/aarch64-linux-gnu/libgio-2.0.so.0 (0x00007fff16c00000)
# libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x00007fff16bc0000)
# libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x00007fff16a00000)
# libgmodule-2.0.so.0 => /lib/aarch64-linux-gnu/libgmodule-2.0.so.0 (0x00007fff169d0000)
# libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x00007fff16930000)
# libunwind.so.8 => /lib/aarch64-linux-gnu/libunwind.so.8 (0x00007fff168e0000)
# libdw.so.1 => /lib/aarch64-linux-gnu/libdw.so.1 (0x00007fff16820000)
# /lib/ld-linux-aarch64.so.1 (0x00007fff17270000)
# libffi.so.8 => /lib/aarch64-linux-gnu/libffi.so.8 (0x00007fff167f0000)
# libpcre2-8.so.0 => /lib/aarch64-linux-gnu/libpcre2-8.so.0 (0x00007fff16740000)
# libz.so.1 => /lib/aarch64-linux-gnu/libz.so.1 (0x00007fff16700000)
# libmount.so.1 => /lib/aarch64-linux-gnu/libmount.so.1 (0x00007fff16670000)
# libselinux.so.1 => /lib/aarch64-linux-gnu/libselinux.so.1 (0x00007fff16620000)
# liblzma.so.5 => /lib/aarch64-linux-gnu/liblzma.so.5 (0x00007fff165d0000)
# libelf.so.1 => /lib/aarch64-linux-gnu/libelf.so.1 (0x00007fff16590000)
# libbz2.so.1.0 => /lib/aarch64-linux-gnu/libbz2.so.1.0 (0x00007fff16560000)
# libblkid.so.1 => /lib/aarch64-linux-gnu/libblkid.so.1 (0x00007fff164e0000)
```

## Usage

```sh
cargo run -- --help
```

Output.

```text
Usage: gst-speaker [OPTIONS] --device <DEVICE>

Options:
      --tracing          Enable tracing (generates a trace-timestamp.json file)
      --device <DEVICE>  ALSA device, as defined in an asound configuration file
      --volume <VOLUME>  Volume factor, 1.0=100% [default: 0.25]
  -h, --help             Print help
  -V, --version          Print version
```

## Running

```sh
# aplay --list-devices
cargo run --profile release-lto -- \
--device=sysdefault:CARD=UACDemoV10
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
