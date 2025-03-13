# GStreamer Example - Remote Camera using Real-Time Streaming Protocol (RTSP)

<https://www.raspberrypi.com/documentation/computers/camera_software.html>

<https://www.raspberrypi.com/documentation/accessories/camera.html>

<https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/rtsp-server.rs>

<https://gitlab.freedesktop.org/gstreamer/gstreamer/-/blob/main/subprojects/gst-rtsp-server/examples/test-launch.c>

<https://gitlab.freedesktop.org/gstreamer/gstreamer/-/tree/main/subprojects/gst-rtsp-server>

RTP

<https://en.wikipedia.org/wiki/Real-time_Transport_Protocol>

RTSP

<https://en.wikipedia.org/wiki/Real-Time_Streaming_Protocol>

```sh
gst-launch-1.0 -v \
libcamerasrc camera-name=/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a ! \
queue ! \
x264enc tune=zerolatency ! \
rtph264pay ! \
udpsink host=192.168.72.152 port=5555

gst-launch-1.0 -v \
udpsrc address=192.168.72.152 port=5555 caps=application/x-rtp ! \
rtph264depay ! \
avdec_h264 ! \
queue ! \
autovideosink
```

<https://github.com/raspberrypi/libcamera>

<https://libcamera.org/>

<https://gstreamer.freedesktop.org/documentation/x264/index.html>

<https://gstreamer.freedesktop.org/documentation/libav/avdec_h264.html>

<https://gstreamer.freedesktop.org/documentation/rtp/rtph264pay.html>

<https://gstreamer.freedesktop.org/documentation/rtp/rtph264depay.html>

<https://gstreamer.freedesktop.org/documentation/udp/udpsink.html>

<https://gstreamer.freedesktop.org/documentation/udp/udpsrc.html>

## GStreamer setup

```sh
# Raspberry Pi OS Lite (November 19th 2024)

# https://gstreamer.freedesktop.org/documentation/rust/stable/latest/docs/gstreamer/#installation-linux
sudo apt purge rpicam-apps-lite
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
gstreamer1.0-libcamera \
rpicam-apps

rpicam-vid --list-cameras

# [Output]
# Available cameras
# -----------------
# 0 : imx708 [4608x2592 10-bit RGGB] (/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a)
#     Modes: 'SRGGB10_CSI2P' : 1536x864 [120.13 fps - (768, 432)/3072x1728 crop]
#                              2304x1296 [56.03 fps - (0, 0)/4608x2592 crop]
#                              4608x2592 [14.35 fps - (0, 0)/4608x2592 crop]


# RTP (Real-time Transport Protocol)
# https://en.wikipedia.org/wiki/Real-time_Transport_Protocol

# 192.168.72.152 -> Remote Client IP Address

# Server (Camera)

gst-launch-1.0 -v \
libcamerasrc camera-name=/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a ! \
queue ! \
x264enc tune=zerolatency ! \
rtph264pay ! \
udpsink host=192.168.72.152 port=5555

# -> [Server output]


# Client (View)

gst-launch-1.0 -v \
udpsrc address=192.168.72.152 port=5555 caps=application/x-rtp ! \
rtph264depay ! \
avdec_h264 ! \
queue ! \
autovideosink

# -> [Client output]
```

Server output.

```text
Setting pipeline to PAUSED ...
[4:27:31.949640669] [36902]  INFO Camera camera_manager.cpp:327 libcamera v0.4.0+53-29156679
[4:27:31.960945084] [36905]  INFO RPI pisp.cpp:720 libpisp version v1.1.0 e7974a156008 27-01-2025 (21:50:51)
[4:27:32.053479673] [36905]  INFO RPI pisp.cpp:1179 Registered camera /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a to CFE device /dev/media0 and ISP device /dev/media1 using PiSP variant BCM2712_C0
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
[4:27:32.095649655] [36909]  INFO Camera camera.cpp:1202 configuring streams: (0) 1280x1080-YUV420
[4:27:32.095827247] [36905]  INFO RPI pisp.cpp:1484 Sensor: /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a - Selected sensor format: 2304x1296-SBGGR10_1X10 - Selected CFE format: 2304x1296-PC1B
/GstPipeline:pipeline0/GstLibcameraSrc:libcamerasrc0.GstLibcameraPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
Redistribute latency...
/GstPipeline:pipeline0/GstX264Enc:x264enc0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstX264Enc:x264enc0.GstPad:src: caps = video/x-h264, codec_data=(buffer)01640028ffe1001d67640028acb200a0044fcb80b501010140000003004000000f23c60c9201000568ebccb22c, stream-format=(string)avc, alignment=(string)au, level=(string)4, profile=(string)high, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstRtpH264Pay:rtph264pay0.GstPad:src: caps = application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264, packetization-mode=(string)1, sprop-parameter-sets=(string)"Z2QAKKyyAKAET8uAtQEBAUAAAAMAQAAADyPGDJI\=\,aOvMsiw\=", profile-level-id=(string)640028, profile=(string)high, payload=(int)96, ssrc=(uint)144238167, timestamp-offset=(uint)968847408, seqnum-offset=(uint)8380, a-framerate=(string)30
/GstPipeline:pipeline0/GstUDPSink:udpsink0.GstPad:sink: caps = application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264, packetization-mode=(string)1, sprop-parameter-sets=(string)"Z2QAKKyyAKAET8uAtQEBAUAAAAMAQAAADyPGDJI\=\,aOvMsiw\=", profile-level-id=(string)640028, profile=(string)high, payload=(int)96, ssrc=(uint)144238167, timestamp-offset=(uint)968847408, seqnum-offset=(uint)8380, a-framerate=(string)30
/GstPipeline:pipeline0/GstRtpH264Pay:rtph264pay0.GstPad:sink: caps = video/x-h264, codec_data=(buffer)01640028ffe1001d67640028acb200a0044fcb80b501010140000003004000000f23c60c9201000568ebccb22c, stream-format=(string)avc, alignment=(string)au, level=(string)4, profile=(string)high, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstRtpH264Pay:rtph264pay0: timestamp = 968889772
/GstPipeline:pipeline0/GstRtpH264Pay:rtph264pay0: seqnum = 8380
Redistribute latency...
^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:45.709384719
Setting pipeline to NULL ...
Freeing pipeline ...
```

Client output.

```text
# Window showing camera stream (video only)

Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
/GstPipeline:pipeline0/GstUDPSrc:udpsrc0.GstPad:src: caps = application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264
/GstPipeline:pipeline0/GstRtpH264Depay:rtph264depay0: extensions = <  >
New clock: GstSystemClock
/GstPipeline:pipeline0/GstRtpH264Depay:rtph264depay0.GstPad:sink: caps = application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264
/GstPipeline:pipeline0/GstRtpH264Depay:rtph264depay0.GstPad:src: caps = video/x-h264, stream-format=(string)avc, alignment=(string)au, codec_data=(buffer)01640028ffe1001d67640028acb200a0044fcb80b501010140000003004000000f23c60c9201000568ebccb22c, level=(string)4, profile=(string)high
Redistribute latency...
/GstPipeline:pipeline0/avdec_h264:avdec_h264-0.GstPad:sink: caps = video/x-h264, stream-format=(string)avc, alignment=(string)au, codec_data=(buffer)01640028ffe1001d67640028acb200a0044fcb80b501010140000003004000000f23c60c9201000568ebccb22c, level=(string)4, profile=(string)high
/GstPipeline:pipeline0/avdec_h264:avdec_h264-0.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
/GstPipeline:pipeline0/GstAutoVideoSink:autovideosink0.GstGhostPad:sink.GstProxyPad:proxypad0: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
/GstPipeline:pipeline0/GstAutoVideoSink:autovideosink0/GstXvImageSink:autovideosink0-actual-sink-xvimage.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
/GstPipeline:pipeline0/GstAutoVideoSink:autovideosink0.GstGhostPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, interlace-mode=(string)progressive, pixel-aspect-ratio=(fraction)1/1, chroma-site=(string)mpeg2, colorimetry=(string)bt709, framerate=(fraction)25/1
Redistribute latency...
^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:26.424863386
Setting pipeline to NULL ...
Freeing pipeline ...
```

## Builds

```sh
# Dev build

cargo build

file -b ../../target/debug/gst-camera

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=657f17059dcbc97b83d3a56476890b80e1c9beaa, for GNU/Linux 3.7.0, with debug_info, not stripped

ls -alh ../../target/debug/gst-camera

# -rwxr-xr-x 2 cavani cavani 21M Mar 11 08:25 ../../target/debug/gst-camera


# Release build

cargo build --release

file -b ../../target/release/gst-camera

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=d54f941f24d2a62ef3eb64cd8a453a9b89d755ef, for GNU/Linux 3.7.0, not stripped

ls -alh ../../target/release/gst-camera

# -rwxr-xr-x 2 cavani cavani 533K Mar 11 08:28 ../../target/release/gst-camera


# LTO build

cargo build --profile release-lto

file -b ../../target/release-lto/gst-camera

# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=4545227907b1bfd229de04677984329c86fbc120, for GNU/Linux 3.7.0, stripped

ls -alh ../../target/release-lto/gst-camera

# -rwxr-xr-x 2 cavani cavani 387K Mar 11 08:32 ../../target/release-lto/gst-camera

ldd ../../target/release-lto/gst-camera

# linux-vdso.so.1 (0x00007fff6cd48000)
# libgstrtspserver-1.0.so.0 => /lib/aarch64-linux-gnu/libgstrtspserver-1.0.so.0 (0x00007fff6cc00000)
# libgstreamer-1.0.so.0 => /lib/aarch64-linux-gnu/libgstreamer-1.0.so.0 (0x00007fff6ca80000)
# libgobject-2.0.so.0 => /lib/aarch64-linux-gnu/libgobject-2.0.so.0 (0x00007fff6ca00000)
# libglib-2.0.so.0 => /lib/aarch64-linux-gnu/libglib-2.0.so.0 (0x00007fff6c8a0000)
# libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x00007fff6c860000)
# libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x00007fff6c6a0000)
# libgstrtsp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstrtsp-1.0.so.0 (0x00007fff6c650000)
# libgstsdp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstsdp-1.0.so.0 (0x00007fff6c610000)
# libgio-2.0.so.0 => /lib/aarch64-linux-gnu/libgio-2.0.so.0 (0x00007fff6c400000)
# libgstrtp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstrtp-1.0.so.0 (0x00007fff6c3b0000)
# libgstbase-1.0.so.0 => /lib/aarch64-linux-gnu/libgstbase-1.0.so.0 (0x00007fff6c310000)
# libgstnet-1.0.so.0 => /lib/aarch64-linux-gnu/libgstnet-1.0.so.0 (0x00007fff6c2d0000)
# libgstapp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstapp-1.0.so.0 (0x00007fff6c290000)
# /lib/ld-linux-aarch64.so.1 (0x00007fff6cd10000)
# libgmodule-2.0.so.0 => /lib/aarch64-linux-gnu/libgmodule-2.0.so.0 (0x00007fff6c260000)
# libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x00007fff6c1c0000)
# libunwind.so.8 => /lib/aarch64-linux-gnu/libunwind.so.8 (0x00007fff6c170000)
# libdw.so.1 => /lib/aarch64-linux-gnu/libdw.so.1 (0x00007fff6c0b0000)
# libffi.so.8 => /lib/aarch64-linux-gnu/libffi.so.8 (0x00007fff6c080000)
# libpcre2-8.so.0 => /lib/aarch64-linux-gnu/libpcre2-8.so.0 (0x00007fff6bfd0000)
# libgstpbutils-1.0.so.0 => /lib/aarch64-linux-gnu/libgstpbutils-1.0.so.0 (0x00007fff6bf70000)
# libz.so.1 => /lib/aarch64-linux-gnu/libz.so.1 (0x00007fff6bf30000)
# libmount.so.1 => /lib/aarch64-linux-gnu/libmount.so.1 (0x00007fff6bea0000)
# libselinux.so.1 => /lib/aarch64-linux-gnu/libselinux.so.1 (0x00007fff6be50000)
# liblzma.so.5 => /lib/aarch64-linux-gnu/liblzma.so.5 (0x00007fff6be00000)
# libelf.so.1 => /lib/aarch64-linux-gnu/libelf.so.1 (0x00007fff6bdc0000)
# libbz2.so.1.0 => /lib/aarch64-linux-gnu/libbz2.so.1.0 (0x00007fff6bd90000)
# libgstvideo-1.0.so.0 => /lib/aarch64-linux-gnu/libgstvideo-1.0.so.0 (0x00007fff6bcb0000)
# libgstaudio-1.0.so.0 => /lib/aarch64-linux-gnu/libgstaudio-1.0.so.0 (0x00007fff6bc10000)
# libgsttag-1.0.so.0 => /lib/aarch64-linux-gnu/libgsttag-1.0.so.0 (0x00007fff6bbb0000)
# libblkid.so.1 => /lib/aarch64-linux-gnu/libblkid.so.1 (0x00007fff6bb30000)
# liborc-0.4.so.0 => /lib/aarch64-linux-gnu/liborc-0.4.so.0 (0x00007fff6ba70000)
```

## Running

<https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/rtsp-server.rs>

<https://gitlab.freedesktop.org/gstreamer/gstreamer/-/blob/main/subprojects/gst-rtsp-server/docs/README>

```sh
# RTSP (Real-Time Streaming Protocol)
# https://en.wikipedia.org/wiki/Real-Time_Streaming_Protocol

# Server (Camera)

cargo run --profile release-lto -q -- \
"( libcamerasrc camera-name=/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a ! queue ! x264enc tune=zerolatency ! rtph264pay name=pay0 )"

# Server (Test Video Source) - Alternative

cargo run --profile release-lto -q -- \
"( videotestsrc ! x264enc ! rtph264pay name=pay0 )"


# Client (View)

# 192.168.72.123 -> Server IP Address

gst-launch-1.0 \
rtspsrc location=rtsp://192.168.72.123:8554/test latency=0 ! \
rtph264depay ! \
avdec_h264 ! \
queue ! \
autovideosink

# vlc rtsp://192.168.72.123:8554/test
# ffplay rtsp://192.168.72.123:8554/test -vf "setpts=N/30" -fflags nobuffer -flags low_delay -framedrop
```

Server output.

```text
Stream ready at rtsp://0.0.0.0:8554/test
[4:32:11.166046643] [37456]  INFO Camera camera_manager.cpp:327 libcamera v0.4.0+53-29156679
[4:32:11.177700686] [37457]  INFO RPI pisp.cpp:720 libpisp version v1.1.0 e7974a156008 27-01-2025 (21:50:51)
[4:32:11.269483445] [37457]  INFO RPI pisp.cpp:1179 Registered camera /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a to CFE device /dev/media0 and ISP device /dev/media1 using PiSP variant BCM2712_C0
[4:32:11.313167664] [37461]  INFO Camera camera.cpp:1202 configuring streams: (0) 1280x1080-YUV420
[4:32:11.313323867] [37457]  INFO RPI pisp.cpp:1484 Sensor: /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a - Selected sensor format: 2304x1296-SBGGR10_1X10 - Selected CFE format: 2304x1296-PC1B
^C
```

Client output.

```text
# Window showing camera stream (video only)

Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Progress: (open) Opening Stream
Pipeline is PREROLLED ...
Prerolled, waiting for progress to finish...
Progress: (connect) Connecting to rtsp://192.168.72.123:8554/test
Progress: (open) Retrieving server options
Progress: (open) Retrieving media info
Progress: (request) SETUP stream 0
Progress: (open) Opened Stream
Setting pipeline to PLAYING ...
New clock: GstSystemClock
Progress: (request) Sending PLAY request
Redistribute latency...
Progress: (request) Sending PLAY request
Redistribute latency...
Progress: (request) Sent PLAY request
Redistribute latency...
Redistribute latency...
Redistribute latency...
Redistribute latency...
^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:27.989928963
Setting pipeline to NULL ...
Freeing pipeline ...
```

## Project

- [gstreamer](https://crates.io/crates/gstreamer): Rust bindings for GStreamer
- [gstreamer-rtsp-server](https://crates.io/crates/gstreamer-rtsp-server): Rust bindings for GStreamer RTSP Server library
- [glib](https://crates.io/crates/glib): Rust bindings for the GLib library

```sh
# Raspberry Pi OS Lite (November 19th 2024) -> Debian 12 (bookworm) -> libgstreamer1.0-dev 1.22.0-2+deb12u1

cargo add \
anyhow \
derive_more \
glib \
gstreamer-rtsp-server \
gstreamer \
--features \
anyhow/backtrace,\
derive_more/display,\
derive_more/error,\
gstreamer/v1_22
```
