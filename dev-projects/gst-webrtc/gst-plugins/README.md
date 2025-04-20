# GStreamer WebRTC Plugin build

```sh
# PWD <- Project Home 'smartrobot-rpi/dev-projects/gst-webrtc/'


# Raspberry Pi OS Lite (November 19th 2024) -> Debian 12 (bookworm) -> libgstreamer1.0-dev 1.22.0-2+deb12u1

# https://gstreamer.freedesktop.org/documentation/rust/stable/latest/docs/gstreamer/#installation-linux
sudo apt install -y \
--no-install-recommends \
libgstreamer1.0-dev \
libgstreamer-plugins-base1.0-dev \
libgstreamer-plugins-bad1.0-dev \
gstreamer1.0-plugins-base \
gstreamer1.0-plugins-good \
gstreamer1.0-plugins-bad \
gstreamer1.0-plugins-ugly \
gstreamer1.0-libav \
gstreamer1.0-nice \
libgstrtspserver-1.0-dev \
libges-1.0-dev \
gstreamer1.0-tools


cargo add \
gst-plugin-webrtc \
gst-plugin-rtp \
--no-default-features \
--features \
gst-plugin-webrtc/v1_22


#
# Raspberry Pi
#

cargo build --profile release-lto

mkdir -p gst-plugins/aarch64-linux-gnu

find ../../target/release-lto/ -name 'libgstrs*.so'

# ../../target/release-lto/deps/libgstrswebrtc-15eecc23ff7fed17.so
# ../../target/release-lto/deps/libgstrsrtp-e80ac5fa54a6413a.so

cp ../../target/release-lto/deps/libgstrswebrtc-15eecc23ff7fed17.so gst-plugins/aarch64-linux-gnu/libgstrswebrtc.so
cp ../../target/release-lto/deps/libgstrsrtp-e80ac5fa54a6413a.so gst-plugins/aarch64-linux-gnu/libgstrsrtp.so

ls -alh gst-plugins/aarch64-linux-gnu/libgstrs*.so

# -rwxr-xr-x 1 cavani cavani 2.3M Mar 12 14:24 gst-plugins/aarch64-linux-gnu/libgstrsrtp.so
# -rwxr-xr-x 1 cavani cavani 2.5M Mar 12 12:07 gst-plugins/aarch64-linux-gnu/libgstrswebrtc.so

file -b gst-plugins/aarch64-linux-gnu/libgstrs*.so

# ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, BuildID[sha1]=8e1aaa26da6f95b6fecc6e5ad02871a5dbae6cff, stripped
# ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, BuildID[sha1]=97c7090bab0447b68b94a8aeba9d8f0cd9cefdbf, stripped

ldd gst-plugins/aarch64-linux-gnu/libgstrswebrtc.so

# linux-vdso.so.1 (0x00007fff94af8000)
# libgstnet-1.0.so.0 => /lib/aarch64-linux-gnu/libgstnet-1.0.so.0 (0x00007fff947e0000)
# libgstreamer-1.0.so.0 => /lib/aarch64-linux-gnu/libgstreamer-1.0.so.0 (0x00007fff94660000)
# libgobject-2.0.so.0 => /lib/aarch64-linux-gnu/libgobject-2.0.so.0 (0x00007fff945e0000)
# libglib-2.0.so.0 => /lib/aarch64-linux-gnu/libglib-2.0.so.0 (0x00007fff94480000)
# libgstaudio-1.0.so.0 => /lib/aarch64-linux-gnu/libgstaudio-1.0.so.0 (0x00007fff943e0000)
# libgstbase-1.0.so.0 => /lib/aarch64-linux-gnu/libgstbase-1.0.so.0 (0x00007fff94340000)
# libgstwebrtc-1.0.so.0 => /lib/aarch64-linux-gnu/libgstwebrtc-1.0.so.0 (0x00007fff94310000)
# libgstsdp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstsdp-1.0.so.0 (0x00007fff942d0000)
# libgstvideo-1.0.so.0 => /lib/aarch64-linux-gnu/libgstvideo-1.0.so.0 (0x00007fff941f0000)
# libgstapp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstapp-1.0.so.0 (0x00007fff941b0000)
# libgstrtp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstrtp-1.0.so.0 (0x00007fff94160000)
# libssl.so.3 => /lib/aarch64-linux-gnu/libssl.so.3 (0x00007fff940a0000)
# libcrypto.so.3 => /lib/aarch64-linux-gnu/libcrypto.so.3 (0x00007fff93c20000)
# libgio-2.0.so.0 => /lib/aarch64-linux-gnu/libgio-2.0.so.0 (0x00007fff93a10000)
# libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x00007fff939d0000)
# libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x00007fff93930000)
# libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x00007fff93770000)
# /lib/ld-linux-aarch64.so.1 (0x00007fff94ac0000)
# libgmodule-2.0.so.0 => /lib/aarch64-linux-gnu/libgmodule-2.0.so.0 (0x00007fff93740000)
# libunwind.so.8 => /lib/aarch64-linux-gnu/libunwind.so.8 (0x00007fff936f0000)
# libdw.so.1 => /lib/aarch64-linux-gnu/libdw.so.1 (0x00007fff93630000)
# libffi.so.8 => /lib/aarch64-linux-gnu/libffi.so.8 (0x00007fff93600000)
# libpcre2-8.so.0 => /lib/aarch64-linux-gnu/libpcre2-8.so.0 (0x00007fff93550000)
# libgsttag-1.0.so.0 => /lib/aarch64-linux-gnu/libgsttag-1.0.so.0 (0x00007fff934f0000)
# liborc-0.4.so.0 => /lib/aarch64-linux-gnu/liborc-0.4.so.0 (0x00007fff93430000)
# libgstpbutils-1.0.so.0 => /lib/aarch64-linux-gnu/libgstpbutils-1.0.so.0 (0x00007fff933d0000)
# libz.so.1 => /lib/aarch64-linux-gnu/libz.so.1 (0x00007fff93390000)
# libmount.so.1 => /lib/aarch64-linux-gnu/libmount.so.1 (0x00007fff93300000)
# libselinux.so.1 => /lib/aarch64-linux-gnu/libselinux.so.1 (0x00007fff932b0000)
# liblzma.so.5 => /lib/aarch64-linux-gnu/liblzma.so.5 (0x00007fff93260000)
# libelf.so.1 => /lib/aarch64-linux-gnu/libelf.so.1 (0x00007fff93220000)
# libbz2.so.1.0 => /lib/aarch64-linux-gnu/libbz2.so.1.0 (0x00007fff931f0000)
# libblkid.so.1 => /lib/aarch64-linux-gnu/libblkid.so.1 (0x00007fff93170000)

ldd gst-plugins/aarch64-linux-gnu/libgstrsrtp.so

# linux-vdso.so.1 (0x00007ffeccf18000)
# libgstvideo-1.0.so.0 => /lib/aarch64-linux-gnu/libgstvideo-1.0.so.0 (0x00007ffeccb90000)
# libgstbase-1.0.so.0 => /lib/aarch64-linux-gnu/libgstbase-1.0.so.0 (0x00007ffeccaf0000)
# libgstreamer-1.0.so.0 => /lib/aarch64-linux-gnu/libgstreamer-1.0.so.0 (0x00007ffecc970000)
# libgobject-2.0.so.0 => /lib/aarch64-linux-gnu/libgobject-2.0.so.0 (0x00007ffecc8f0000)
# libglib-2.0.so.0 => /lib/aarch64-linux-gnu/libglib-2.0.so.0 (0x00007ffecc790000)
# libgstnet-1.0.so.0 => /lib/aarch64-linux-gnu/libgstnet-1.0.so.0 (0x00007ffecc750000)
# libgstrtp-1.0.so.0 => /lib/aarch64-linux-gnu/libgstrtp-1.0.so.0 (0x00007ffecc700000)
# libgio-2.0.so.0 => /lib/aarch64-linux-gnu/libgio-2.0.so.0 (0x00007ffecc4f0000)
# libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x00007ffecc4b0000)
# libm.so.6 => /lib/aarch64-linux-gnu/libm.so.6 (0x00007ffecc410000)
# libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x00007ffecc250000)
# liborc-0.4.so.0 => /lib/aarch64-linux-gnu/liborc-0.4.so.0 (0x00007ffecc190000)
# /lib/ld-linux-aarch64.so.1 (0x00007ffeccee0000)
# libgmodule-2.0.so.0 => /lib/aarch64-linux-gnu/libgmodule-2.0.so.0 (0x00007ffecc160000)
# libunwind.so.8 => /lib/aarch64-linux-gnu/libunwind.so.8 (0x00007ffecc110000)
# libdw.so.1 => /lib/aarch64-linux-gnu/libdw.so.1 (0x00007ffecc050000)
# libffi.so.8 => /lib/aarch64-linux-gnu/libffi.so.8 (0x00007ffecc020000)
# libpcre2-8.so.0 => /lib/aarch64-linux-gnu/libpcre2-8.so.0 (0x00007ffecbf70000)
# libz.so.1 => /lib/aarch64-linux-gnu/libz.so.1 (0x00007ffecbf30000)
# libmount.so.1 => /lib/aarch64-linux-gnu/libmount.so.1 (0x00007ffecbea0000)
# libselinux.so.1 => /lib/aarch64-linux-gnu/libselinux.so.1 (0x00007ffecbe50000)
# liblzma.so.5 => /lib/aarch64-linux-gnu/liblzma.so.5 (0x00007ffecbe00000)
# libelf.so.1 => /lib/aarch64-linux-gnu/libelf.so.1 (0x00007ffecbdc0000)
# libbz2.so.1.0 => /lib/aarch64-linux-gnu/libbz2.so.1.0 (0x00007ffecbd90000)
# libblkid.so.1 => /lib/aarch64-linux-gnu/libblkid.so.1 (0x00007ffecbd10000)

export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

gst-inspect-1.0 rswebrtc

# Plugin Details:
#   Name                     rswebrtc
#   Description              GStreamer plugin for high level WebRTC elements and a simple signaling server
#   Filename                 /home/cavani/Workspace/smartrobot-rpi/dev-projects/gst-webrtc/gst-plugins/aarch64-linux-gnu/libgstrswebrtc.so
#   Version                  0.13.5-RELEASE
#   License                  MPL-2.0
#   Source module            gst-plugin-webrtc
#   Documentation            https://gstreamer.freedesktop.org/documentation/rswebrtc/
#   Source release date      2025-03-04
#   Binary package           gst-plugin-webrtc
#   Origin URL               https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs
#
#   webrtcsink: WebRTCSink
#   webrtcsrc: WebRTCSrc
#
#   2 features:
#   +-- 2 elements

gst-inspect-1.0 rsrtp

# Plugin Details:
#   Name                     rsrtp
#   Description              GStreamer Rust RTP Plugin
#   Filename                 /home/cavani/Workspace/smartrobot-rpi/dev-projects/gst-webrtc/gst-plugins/aarch64-linux-gnu/libgstrsrtp.so
#   Version                  0.13.5-RELEASE
#   License                  MPL-2.0
#   Source module            gst-plugin-rtp
#   Documentation            https://gstreamer.freedesktop.org/documentation/rsrtp/
#   Source release date      2025-03-04
#   Binary package           gst-plugin-rtp
#   Origin URL               https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs
#
#   rtpac3depay2: RTP AC-3 Audio Depayloader
#   rtpac3pay2: RTP AC-3 Audio Payloader
#   rtpav1depay: RTP AV1 Depayloader
#   rtpav1pay: RTP AV1 payloader
#   rtpgccbwe: Google Congestion Control bandwidth estimator
#   rtpjpegdepay2: RTP JPEG Depayloader
#   rtpjpegpay2: RTP JPEG payloader
#   rtpklvdepay2: RTP KLV Metadata Depayloader
#   rtpklvpay2: RTP KLV Metadata Payloader
#   rtpmp2tdepay2: RTP MPEG-TS Depayloader
#   rtpmp2tpay2: RTP MPEG-TS Payloader
#   rtpmp4adepay2: RTP MPEG-4 Audio Depayloader
#   rtpmp4apay2: RTP MPEG-4 Audio Payloader
#   rtpmp4gdepay2: RTP MPEG-4 Generic ES Depayloader
#   rtpmp4gpay2: RTP MPEG-4 Generic Payloader
#   rtpopusdepay2: RTP Opus Depayloader
#   rtpopuspay2: RTP Opus Payloader
#   rtppcmadepay2: RTP PCMA Depayloader
#   rtppcmapay2: RTP PCMA Payloader
#   rtppcmudepay2: RTP PCMU Depayloader
#   rtppcmupay2: RTP PCMU Payloader
#   rtprecv: RTP Session receiver
#   rtpsend: RTP Session Sender
#   rtpvp8depay2: RTP VP8 Depayloader
#   rtpvp8pay2: RTP VP8 payloader
#   rtpvp9depay2: RTP VP9 Depayloader
#   rtpvp9pay2: RTP VP9 payloader
#
#   27 features:
#   +-- 27 elements


#
# Ubuntu x86_64
#

cargo build --profile release-lto

mkdir -p gst-plugins/x86_64-linux-gnu

find ../../target/release-lto/ -name 'libgstrs*.so'

# ../../target/release-lto/deps/libgstrsrtp-3655e826170273c0.so
# ../../target/release-lto/deps/libgstrswebrtc-5cd145f8fc449d4f.so

cp ../../target/release-lto/deps/libgstrswebrtc-5cd145f8fc449d4f.so gst-plugins/x86_64-linux-gnu/libgstrswebrtc.so
cp ../../target/release-lto/deps/libgstrsrtp-3655e826170273c0.so gst-plugins/x86_64-linux-gnu/libgstrsrtp.so

ls -alh gst-plugins/x86_64-linux-gnu/libgstrs*.so

# -rwxrwxr-x 1 cavani cavani 2.6M Mar 12 14:36 gst-plugins/x86_64-linux-gnu/libgstrsrtp.so
# -rwxrwxr-x 1 cavani cavani 2.8M Mar 12 14:36 gst-plugins/x86_64-linux-gnu/libgstrswebrtc.so

file -b gst-plugins/x86_64-linux-gnu/libgstrs*.so

# ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, BuildID[sha1]=23ca2b6170fc14ad2e86c5dcd70d6943e8edd876, stripped
# ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, BuildID[sha1]=d80ec0a15b38c1a93afe1eccb6d817b264df5f26, stripped

ldd gst-plugins/x86_64-linux-gnu/libgstrswebrtc.so

# linux-vdso.so.1 (0x0000727bd6c58000)
# libgstnet-1.0.so.0 => /lib/x86_64-linux-gnu/libgstnet-1.0.so.0 (0x0000727bd6c12000)
# libgstreamer-1.0.so.0 => /lib/x86_64-linux-gnu/libgstreamer-1.0.so.0 (0x0000727bd66ab000)
# libgobject-2.0.so.0 => /lib/x86_64-linux-gnu/libgobject-2.0.so.0 (0x0000727bd6bae000)
# libglib-2.0.so.0 => /lib/x86_64-linux-gnu/libglib-2.0.so.0 (0x0000727bd655c000)
# libgstaudio-1.0.so.0 => /lib/x86_64-linux-gnu/libgstaudio-1.0.so.0 (0x0000727bd6b27000)
# libgstbase-1.0.so.0 => /lib/x86_64-linux-gnu/libgstbase-1.0.so.0 (0x0000727bd64d6000)
# libgstwebrtc-1.0.so.0 => /lib/x86_64-linux-gnu/libgstwebrtc-1.0.so.0 (0x0000727bd6b11000)
# libgstsdp-1.0.so.0 => /lib/x86_64-linux-gnu/libgstsdp-1.0.so.0 (0x0000727bd6af5000)
# libgstvideo-1.0.so.0 => /lib/x86_64-linux-gnu/libgstvideo-1.0.so.0 (0x0000727bd63fe000)
# libgstapp-1.0.so.0 => /lib/x86_64-linux-gnu/libgstapp-1.0.so.0 (0x0000727bd6ade000)
# libgstrtp-1.0.so.0 => /lib/x86_64-linux-gnu/libgstrtp-1.0.so.0 (0x0000727bd63cc000)
# libssl.so.3 => /lib/x86_64-linux-gnu/libssl.so.3 (0x0000727bd62ca000)
# libcrypto.so.3 => /lib/x86_64-linux-gnu/libcrypto.so.3 (0x0000727bd5c00000)
# libgio-2.0.so.0 => /lib/x86_64-linux-gnu/libgio-2.0.so.0 (0x0000727bd5a28000)
# libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x0000727bd629c000)
# libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x0000727bd61af000)
# libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x0000727bd5800000)
# /lib64/ld-linux-x86-64.so.2 (0x0000727bd6c5a000)
# libgmodule-2.0.so.0 => /lib/x86_64-linux-gnu/libgmodule-2.0.so.0 (0x0000727bd6ad3000)
# libunwind.so.8 => /lib/x86_64-linux-gnu/libunwind.so.8 (0x0000727bd6194000)
# libdw.so.1 => /lib/x86_64-linux-gnu/libdw.so.1 (0x0000727bd574b000)
# libffi.so.8 => /lib/x86_64-linux-gnu/libffi.so.8 (0x0000727bd6ac7000)
# libatomic.so.1 => /lib/x86_64-linux-gnu/libatomic.so.1 (0x0000727bd6189000)
# libpcre2-8.so.0 => /lib/x86_64-linux-gnu/libpcre2-8.so.0 (0x0000727bd56ae000)
# libgsttag-1.0.so.0 => /lib/x86_64-linux-gnu/libgsttag-1.0.so.0 (0x0000727bd566d000)
# liborc-0.4.so.0 => /lib/x86_64-linux-gnu/liborc-0.4.so.0 (0x0000727bd55bc000)
# libgstpbutils-1.0.so.0 => /lib/x86_64-linux-gnu/libgstpbutils-1.0.so.0 (0x0000727bd5577000)
# libz.so.1 => /lib/x86_64-linux-gnu/libz.so.1 (0x0000727bd616b000)
# libzstd.so.1 => /lib/x86_64-linux-gnu/libzstd.so.1 (0x0000727bd54b9000)
# libmount.so.1 => /lib/x86_64-linux-gnu/libmount.so.1 (0x0000727bd546c000)
# libselinux.so.1 => /lib/x86_64-linux-gnu/libselinux.so.1 (0x0000727bd543e000)
# liblzma.so.5 => /lib/x86_64-linux-gnu/liblzma.so.5 (0x0000727bd540b000)
# libelf.so.1 => /lib/x86_64-linux-gnu/libelf.so.1 (0x0000727bd53ec000)
# libbz2.so.1.0 => /lib/x86_64-linux-gnu/libbz2.so.1.0 (0x0000727bd53d8000)
# libblkid.so.1 => /lib/x86_64-linux-gnu/libblkid.so.1 (0x0000727bd539c000)

ldd gst-plugins/x86_64-linux-gnu/libgstrsrtp.so

# linux-vdso.so.1 (0x0000740826379000)
# libgstvideo-1.0.so.0 => /lib/x86_64-linux-gnu/libgstvideo-1.0.so.0 (0x0000740825f28000)
# libgstbase-1.0.so.0 => /lib/x86_64-linux-gnu/libgstbase-1.0.so.0 (0x00007408262cf000)
# libgstreamer-1.0.so.0 => /lib/x86_64-linux-gnu/libgstreamer-1.0.so.0 (0x0000740825dd3000)
# libgobject-2.0.so.0 => /lib/x86_64-linux-gnu/libgobject-2.0.so.0 (0x0000740825d6f000)
# libglib-2.0.so.0 => /lib/x86_64-linux-gnu/libglib-2.0.so.0 (0x0000740825c20000)
# libgstnet-1.0.so.0 => /lib/x86_64-linux-gnu/libgstnet-1.0.so.0 (0x00007408262ab000)
# libgstrtp-1.0.so.0 => /lib/x86_64-linux-gnu/libgstrtp-1.0.so.0 (0x0000740825bee000)
# libgio-2.0.so.0 => /lib/x86_64-linux-gnu/libgio-2.0.so.0 (0x0000740825a16000)
# libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007408259e8000)
# libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007408258fb000)
# libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x0000740825600000)
# /lib64/ld-linux-x86-64.so.2 (0x000074082637b000)
# liborc-0.4.so.0 => /lib/x86_64-linux-gnu/liborc-0.4.so.0 (0x000074082584a000)
# libgmodule-2.0.so.0 => /lib/x86_64-linux-gnu/libgmodule-2.0.so.0 (0x00007408262a2000)
# libunwind.so.8 => /lib/x86_64-linux-gnu/libunwind.so.8 (0x0000740826287000)
# libdw.so.1 => /lib/x86_64-linux-gnu/libdw.so.1 (0x000074082554b000)
# libffi.so.8 => /lib/x86_64-linux-gnu/libffi.so.8 (0x000074082583e000)
# libatomic.so.1 => /lib/x86_64-linux-gnu/libatomic.so.1 (0x0000740825833000)
# libpcre2-8.so.0 => /lib/x86_64-linux-gnu/libpcre2-8.so.0 (0x00007408254ae000)
# libz.so.1 => /lib/x86_64-linux-gnu/libz.so.1 (0x0000740825490000)
# libmount.so.1 => /lib/x86_64-linux-gnu/libmount.so.1 (0x0000740825443000)
# libselinux.so.1 => /lib/x86_64-linux-gnu/libselinux.so.1 (0x0000740825415000)
# liblzma.so.5 => /lib/x86_64-linux-gnu/liblzma.so.5 (0x00007408253e2000)
# libelf.so.1 => /lib/x86_64-linux-gnu/libelf.so.1 (0x00007408253c3000)
# libzstd.so.1 => /lib/x86_64-linux-gnu/libzstd.so.1 (0x0000740825305000)
# libbz2.so.1.0 => /lib/x86_64-linux-gnu/libbz2.so.1.0 (0x00007408252f1000)
# libblkid.so.1 => /lib/x86_64-linux-gnu/libblkid.so.1 (0x00007408252b5000)

export GST_PLUGIN_PATH=$PWD/gst-plugins/x86_64-linux-gnu/

gst-inspect-1.0 rswebrtc

# [plugin info]

gst-inspect-1.0 rsrtp

# [plugin info]
```
