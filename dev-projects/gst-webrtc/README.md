# GStreamer Example - WebRTC

WebRTC (Web Real-Time Communication)

<https://en.wikipedia.org/wiki/WebRTC>

Server (on device):

- Send audio from microphone
- Send video from camera
- Receive audio and play on speaker
- Receive video (discard, save to file, send back on overlay)
- Send audio from another source (file, TTS)
- Send video from another source (file, image)

Client (web browser):

- Send audio from microphone
- Send video from camera
- Receive audio and play on speaker
- Receive video and play on screen

<https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/tree/main/net/webrtc>

<https://crates.io/crates/gst-plugin-webrtc>

<https://crates.io/crates/gst-plugin-rtp>

<https://crates.io/crates/gst-plugin-webrtc-signalling>

<https://docs.rs/crate/gst-plugin-webrtc/latest>

<https://docs.rs/crate/gst-plugin-rtp/latest>

<https://gstreamer.freedesktop.org/documentation/rswebrtc/webrtcsink.html>

<https://gstreamer.freedesktop.org/documentation/rswebrtc/webrtcsrc.html>

## Prototyping

### GStreamer setup

```sh
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
gstreamer1.0-libcamera \
gstreamer1.0-alsa \
gstreamer1.0-tools
```

### GStreamer WebRTC Signalling Server

```sh
cargo binstall gst-plugin-webrtc-signalling

gst-webrtc-signalling-server --help

# GStreamer WebRTC sink signalling server
#
# Usage: gst-webrtc-signalling-server [OPTIONS]
#
# Options:
#       --host <HOST>                    Address to listen on [default: 0.0.0.0]
#   -p, --port <PORT>                    Port to listen on [default: 8443]
#   -c, --cert <CERT>                    TLS certificate to use
#       --cert-password <CERT_PASSWORD>  password to TLS certificate
#   -h, --help                           Print help
#   -V, --version                        Print version

nohup gst-webrtc-signalling-server > webrtc.log &

tail -f webrtc.log

# 2025-03-12T16:02:21.606418Z  INFO ThreadId(01) gst_webrtc_signalling_server: Listening on: 0.0.0.0:8443
```

### Video Only Test

[GStreamer WebRTC Plugin build](./gst-plugins/README.md).

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

# GStreamer WebRTC Signalling server

gst-webrtc-signalling-server

# 2025-03-12T16:02:21.606418Z  INFO ThreadId(01) gst_webrtc_signalling_server: Listening on: 0.0.0.0:8443

# Streaming pipeline (server)

export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

ldd $GST_PLUGIN_PATH/libgstrswebrtc.so

ldd $GST_PLUGIN_PATH/libgstrsrtp.so

gst-inspect-1.0 webrtcsink

gst-launch-1.0 -v \
videotestsrc ! \
webrtcsink

# -> [Server output]


#
# Client (reception)
#

# Ubuntu x86_64
# 192.168.72.123 <- Server IP Address

export GST_PLUGIN_PATH=$PWD/gst-plugins/x86_64-linux-gnu/

ldd $GST_PLUGIN_PATH/libgstrswebrtc.so

ldd $GST_PLUGIN_PATH/libgstrsrtp.so

gst-inspect-1.0 webrtcsrc

gst-launch-1.0 -v \
webrtcsrc connect-to-first-producer=true signaller::uri=ws://192.168.72.123:8443 ! \
videoconvert ! \
queue ! \
autovideosink

# -> [Client output]
```

<details>
<summary>Server output.</summary>

```text
Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
/GstPipeline:pipeline0/GstVideoTestSrc:videotestsrc0.GstPad:src: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
New clock: GstSystemClock
/GstPipeline:pipeline0/GstWebRTCSink:webrtcsink0.GstWebRTCSinkPad:video_0.GstProxyPad:proxypad0: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstWebRTCSink:webrtcsink0/GstClockSync:clocksync0.GstPad:src: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstWebRTCSink:webrtcsink0/GstAppSink:appsink0.GstPad:sink: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstWebRTCSink:webrtcsink0/GstClockSync:clocksync0.GstPad:sink: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstWebRTCSink:webrtcsink0.GstWebRTCSinkPad:video_0: caps = video/x-raw, format=(string)ABGR64_LE, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
Redistribute latency...

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:28.532335277
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

<details>
<summary>Client output.</summary>

```text
# Window showing test stream (video only)

Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-percentage = 100
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = false
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = none
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: mlineindex = 0
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: receiver = "\(GstWebRTCRTPReceiver\)\ webrtcrtpreceiver0"
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: sender = "\(GstWebRTCRTPSender\)\ webrtcrtpsender0"
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: name = webrtctransceiver0
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = true
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = ulp-red
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: session-id = 0
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: name = transportstream0
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = have-remote-offer
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPSender:webrtcrtpsender0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPReceiver:webrtcrtpreceiver0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 101=(uint)104, 96=(uint)103;
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0: payload-type-map = application/x-rtp-pt-map, 101=(uint)104, 96=(uint)103;
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 101=(uint)104, 96=(uint)103;
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0: payloads = < (int)101 >
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: dtls-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = stable
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = gathering
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = checking
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = complete
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsDec:dtlsdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstFunnelPad:funnelpad0: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstNiceSink:nicesink0.GstPad:sink: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = connected
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1: leaky = no
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src.GstProxyPad:proxypad2: caps = application/x-dtls
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstCapsFilter:capsfilter0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue1.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink.GstProxyPad:proxypad5: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsSrtpDemux:dtlssrtpdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstSrtpDec:srtpdec0.GstPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0.GstProxyPad:proxypad23: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0.GstProxyPad:proxypad24: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSsrcDemux:rtpssrcdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0.GstProxyPad:proxypad25: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:webrtcsrc0/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0: caps = application/x-rtp

[LINES REMOVED]

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:06.575047116
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

### Audio and Video Test

Codecs:

- Audio: Opus
- Video: VP8

[GStreamer WebRTC Plugin build](./gst-plugins/README.md).

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

# GStreamer WebRTC Signalling server

gst-webrtc-signalling-server

# 2025-03-12T16:02:21.606418Z  INFO ThreadId(01) gst_webrtc_signalling_server: Listening on: 0.0.0.0:8443

# Streaming pipeline (server)

export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

gst-launch-1.0 -v \
webrtcsink name=ws \
audiotestsrc is-live=true wave=red-noise ! \
audioconvert ! \
audioresample ! \
queue ! \
opusenc perfect-timestamp=true ! \
ws. \
videotestsrc is-live=true pattern=ball ! \
videoconvert ! \
queue ! \
vp8enc deadline=1 ! \
ws.

# -> [Server output]


#
# Client (reception)
#

# Ubuntu x86_64
# 192.168.72.123 <- Server IP Address

export GST_PLUGIN_PATH=$PWD/gst-plugins/x86_64-linux-gnu/

gst-launch-1.0 -v \
webrtcsrc name=ws connect-to-first-producer=true signaller::uri=ws://192.168.72.123:8443 \
ws. ! \
queue ! \
videoconvert ! \
autovideosink \
ws. ! \
queue ! \
audioconvert ! \
audioresample ! \
autoaudiosink

# -> [Client output]
```

<details>
<summary>Server output.</summary>

```text
Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
/GstPipeline:pipeline0/GstVideoTestSrc:videotestsrc0.GstPad:src: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:src: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:sink: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:src: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:src: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
Redistribute latency...
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0.GstProxyPad:proxypad1: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink1.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100014000f00000010000010000001e00000001 >, width=(int)320, height=(int)240, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt601, chroma-site=(string)jpeg, multiview-mode=(string)mono, multiview-flags=(GstVideoMultiviewFlagsSet)0:ffffffff:/right-view-first/left-flipped/left-flopped/right-flipped/right-flopped/half-aspect/mixed-mono
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:sink: caps = video/x-raw, width=(int)320, height=(int)240, framerate=(fraction)30/1, multiview-mode=(string)mono, format=(string)I420, pixel-aspect-ratio=(fraction)1/1, interlace-mode=(string)progressive
/GstPipeline:pipeline0/GstAudioTestSrc:audiotestsrc0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
Redistribute latency...
/GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
Redistribute latency...
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0.GstProxyPad:proxypad0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink0.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e63010000001a0000004445534352495054494f4e3d617564696f74657374207761766501 >
Redistribute latency...

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:18.055821149
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

<details>
<summary>Client output.</summary>

```text
# Window showing test stream and playing noise.

Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-percentage = 100
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = false
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = none
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: mlineindex = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: receiver = "\(GstWebRTCRTPReceiver\)\ webrtcrtpreceiver0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: sender = "\(GstWebRTCRTPSender\)\ webrtcrtpsender0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: name = webrtctransceiver0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = ulp-red
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-percentage = 100
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: do-nack = false
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-type = none
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: mlineindex = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: receiver = "\(GstWebRTCRTPReceiver\)\ webrtcrtpreceiver1"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: sender = "\(GstWebRTCRTPSender\)\ webrtcrtpsender1"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: name = webrtctransceiver1
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: do-nack = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-type = ulp-red
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: session-id = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: name = transportstream0
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = have-remote-offer
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPSender:webrtcrtpsender0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPReceiver:webrtcrtpreceiver0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1/GstWebRTCRTPSender:webrtcrtpsender1: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1/GstWebRTCRTPReceiver:webrtcrtpreceiver1: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0: payloads = < (int)97 >
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: dtls-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = stable
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = gathering
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = checking
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = complete
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsDec:dtlsdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstFunnelPad:funnelpad0: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstNiceSink:nicesink0.GstPad:sink: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src.GstProxyPad:proxypad4: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = no
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = connected
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstCapsFilter:capsfilter0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink.GstProxyPad:proxypad7: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsSrtpDemux:dtlssrtpdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstSrtpDec:srtpdec0.GstPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0.GstProxyPad:proxypad25: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0.GstProxyPad:proxypad26: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSsrcDemux:rtpssrcdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0.GstProxyPad:proxypad27: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0: caps = application/x-rtp

[LINES REMOVED]

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:06.649914437
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

### Camera and Microphone

Codecs:

- Audio: Opus
- Video: VP8

[GStreamer WebRTC Plugin build](./gst-plugins/README.md).

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

# GStreamer WebRTC Signalling server

gst-webrtc-signalling-server

# 2025-03-12T16:02:21.606418Z  INFO ThreadId(01) gst_webrtc_signalling_server: Listening on: 0.0.0.0:8443

# Streaming pipeline (server)

export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

gst-launch-1.0 -v \
webrtcsink name=ws \
alsasrc device=sysdefault:CARD=Device ! \
audioconvert ! \
audioresample ! \
queue ! \
opusenc perfect-timestamp=true ! \
ws. \
libcamerasrc camera-name=/base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a ! \
videoconvert ! \
queue ! \
vp8enc deadline=1 ! \
ws.

# -> [Server output]


#
# Client (reception)
#

# Ubuntu x86_64
# 192.168.72.123 <- Server IP Address

export GST_PLUGIN_PATH=$PWD/gst-plugins/x86_64-linux-gnu/

gst-launch-1.0 -v \
webrtcsrc name=ws connect-to-first-producer=true signaller::uri=ws://192.168.72.123:8443 \
ws. ! \
queue ! \
videoconvert ! \
autovideosink \
ws. ! \
queue ! \
audioconvert ! \
audioresample ! \
autoaudiosink

# -> [Client output]
```

<details>
<summary>Server output.</summary>

```text
Setting pipeline to PAUSED ...
[1:25:19.081789213] [6374]  INFO Camera camera_manager.cpp:327 libcamera v0.4.0+53-29156679
[1:25:19.089652889] [6377]  INFO RPI pisp.cpp:720 libpisp version v1.1.0 e7974a156008 27-01-2025 (21:50:51)
[1:25:19.178314477] [6377]  INFO RPI pisp.cpp:1179 Registered camera /base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a to CFE device /dev/media1 and ISP device /dev/media0 using PiSP variant BCM2712_C0
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstAudioSrcClock
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-buffer-time = 192000
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-latency-time = 21333
Redistribute latency...
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
Redistribute latency...
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0.GstProxyPad:proxypad0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink1.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
Redistribute latency...
[1:25:19.302366013] [6382]  INFO Camera camera.cpp:1202 configuring streams: (0) 1280x1080-YUV420
[1:25:19.302522457] [6377]  INFO RPI pisp.cpp:1484 Sensor: /base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a - Selected sensor format: 2304x1296-SBGGR10_1X10 - Selected CFE format: 2304x1296-PC1B
/GstPipeline:pipeline0/GstLibcameraSrc:libcamerasrc0.GstLibcameraPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
Redistribute latency...
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0.GstProxyPad:proxypad1: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink0.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
Redistribute latency...

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:28.740036882
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

<details>
<summary>Client output.</summary>

```text
# Window showing camera video and playing microphone audio.

Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-percentage = 100
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = false
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = none
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: mlineindex = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: receiver = "\(GstWebRTCRTPReceiver\)\ webrtcrtpreceiver0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: sender = "\(GstWebRTCRTPSender\)\ webrtcrtpsender0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: name = webrtctransceiver0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: do-nack = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0: fec-type = ulp-red
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-percentage = 100
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: do-nack = false
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-type = none
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: mlineindex = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: receiver = "\(GstWebRTCRTPReceiver\)\ webrtcrtpreceiver1"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: sender = "\(GstWebRTCRTPSender\)\ webrtcrtpsender1"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: name = webrtctransceiver1
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: do-nack = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1: fec-type = ulp-red
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: session-id = 0
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: name = transportstream0
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = have-remote-offer
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPSender:webrtcrtpsender0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver0/GstWebRTCRTPReceiver:webrtcrtpreceiver0: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1/GstWebRTCRTPSender:webrtcrtpsender1: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/WebRTCTransceiver:webrtctransceiver1/GstWebRTCRTPReceiver:webrtcrtpreceiver1: transport = "\(GstWebRTCDTLSTransport\)\ webrtcdtlstransport0"
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin1/GstRtpRtxSend:rtprtxsend0: payload-type-map = application/x-rtp-pt-map, 97=(uint)100, 96=(uint)99;
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0: payloads = < (int)97 >
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: is-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportStream:transportstream0: dtls-client = true
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = downstream
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: signaling-state = stable
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = gathering
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = checking
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-gathering-state = complete
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = new
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsDec:dtlsdec0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0: connection-state = connecting
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstDtlsEnc:dtlsenc0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstFunnelPad:funnelpad0: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0/GstFunnel:funnel0.GstPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2: leaky = no
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstNiceSink:nicesink0.GstPad:sink: caps = application/x-dtls
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0: ice-connection-state = connected
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportSendBin:transportsendbin0/GstDtlsSrtpEnc:dtlssrtpenc0.GstGhostPad:src.GstProxyPad:proxypad4: caps = application/x-dtls
Redistribute latency...
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstCapsFilter:capsfilter0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstQueue:queue2.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink.GstProxyPad:proxypad7: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstDtlsSrtpDemux:dtlssrtpdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0/GstSrtpDec:srtpdec0.GstPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0/GstDtlsSrtpDec:dtlssrtpdec0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/TransportReceiveBin:transportreceivebin0.GstGhostPad:rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0.GstProxyPad:proxypad25: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0.GstProxyPad:proxypad26: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:src: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSsrcDemux:rtpssrcdemux0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpStorage:rtpstorage0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:src_0.GstProxyPad:proxypad27: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRedDec:rtpreddec0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2/GstRtpRtxReceive:rtprtxreceive0.GstPad:sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstBin:bin2.GstGhostPad:sink_0: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin/GstRtpSession:rtpsession0.GstPad:recv_rtp_sink: caps = application/x-rtp
/GstPipeline:pipeline0/GstWebRTCSrc:ws/GstBin:bin0/GstWebRTCBin:webrtcbin0/GstRtpBin:rtpbin.GstGhostPad:recv_rtp_sink_0: caps = application/x-rtp

[LINES REMOVED]

^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:00:08.487553952
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

### Web App

<https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/tree/0.13.5/net/webrtc/gstwebrtc-api>

<https://crates.io/crates/gst-plugin-webrtc-signalling>

<https://crates.io/crates/simple-http-server>

[HTTPS, WSS, Custom CA configuration](./pki/README.md).

[GStreamer WebRTC Javascript API build](./web-app/README.md).

[GStreamer WebRTC Plugin build](./gst-plugins/README.md).

```sh
# GStreamer WebRTC Signalling server


gst-webrtc-signalling-server --cert pki/olivia-v1_server.p12


# Streaming pipeline (server)


export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

gst-launch-1.0 -v \
webrtcsink name=ws signaller::uri='wss://webrtc.olivia-v1.machine-domain:8443' \
alsasrc device=sysdefault:CARD=Device ! \
audioconvert ! \
audioresample ! \
queue ! \
opusenc perfect-timestamp=true ! \
ws. \
libcamerasrc camera-name=/base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a ! \
videoconvert ! \
queue ! \
vp8enc deadline=1 ! \
ws.


# Web App HTTP server


cargo binstall simple-http-server

simple-http-server -i web-app/ --cert pki/olivia-v1_server.p12


# Client Web Browser


firefox --private-window https://webrtc.olivia-v1.machine-domain:8000/

# (section Remote Streams, click on peer id)


# Server comsumer audio
# dc998a98-fb0f-4b14-a039-dbb153dcc2ed <- Client ID on the Web App page

export GST_PLUGIN_PATH=$PWD/gst-plugins/aarch64-linux-gnu/

gst-launch-1.0 -v \
webrtcsrc \
 signaller::producer-peer-id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed \
 signaller::uri=wss://webrtc.olivia-v1.machine-domain:8443 ! \
queue ! \
volume volume=0.5 ! \
audioconvert ! \
audioresample ! \
alsasink device=sysdefault:CARD=UACDemoV10
```

<details>
<summary>Signalling Server output.</summary>

```text
2025-03-20T14:01:56.443398Z  INFO ThreadId(01) gst_webrtc_signalling_server: Listening on: 0.0.0.0:8443
2025-03-20T14:02:02.343823Z  INFO ThreadId(01) gst_webrtc_signalling_server: Accepting connection from 127.0.0.1:50598
2025-03-20T14:02:02.378254Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: New WebSocket connection this_id=0a3f49ff-862e-4c76-b24b-521fd0eba60e
2025-03-20T14:02:02.421177Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"list\"}")))
2025-03-20T14:02:02.421488Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"setPeerStatus\",\"roles\":[\"producer\"],\"meta\":null,\"peerId\":\"0a3f49ff-862e-4c76-b24b-521fd0eba60e\"}")))
2025-03-20T14:02:02.421556Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::handlers: registered as a producer peer_id=0a3f49ff-862e-4c76-b24b-521fd0eba60e
2025-03-20T14:02:11.066722Z  INFO ThreadId(01) gst_webrtc_signalling_server: Accepting connection from 192.168.72.152:56148
2025-03-20T14:02:11.178378Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: New WebSocket connection this_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed
2025-03-20T14:02:11.272932Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"setPeerStatus\",\"roles\":[\"listener\"],\"meta\":{\"name\":\"WebClient-1742479331027\"}}")))
2025-03-20T14:02:11.273022Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::handlers: registered as a producer peer_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed
2025-03-20T14:02:11.276406Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"list\"}")))
2025-03-20T14:02:14.965338Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"startSession\",\"peerId\":\"0a3f49ff-862e-4c76-b24b-521fd0eba60e\"}")))
2025-03-20T14:02:14.965565Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::handlers: started a session id=e8d78c32-95a9-4caf-ba05-6e29f0c65890 producer_id=0a3f49ff-862e-4c76-b24b-521fd0eba60e consumer_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed
2025-03-20T14:02:16.121361Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"sdp\":{\"type\":\"offer\",\"sdp\":\"v=0\\r\\no=- 4868623278050899142 0 IN IP4 0.0.0.0\\r\\ns=-\\r\\nt=0 0\\r\\na=ice-options:trickle\\r\\na=group:BUNDLE audio0 video1\\r\\nm=audio 9 UDP/TLS/RTP/SAVPF 101\\r\\nc=IN IP4 0.0.0.0\\r\\na=setup:actpass\\r\\na=ice-ufrag:vUFzktWm1LLErNFcNB3ehdAZCQuN3oqN\\r\\na=ice-pwd:cM5+n8ZXC0k142eHJOePxxKloRrtV57K\\r\\na=rtcp-mux\\r\\na=rtcp-rsize\\r\\na=sendonly\\r\\na=rtpmap:101 OPUS/48000/2\\r\\na=rtcp-fb:101 transport-cc\\r\\na=extmap:1 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\\r\\na=fmtp:101 sprop-stereo=0;sprop-maxcapturerate=48000\\r\\na=ssrc:470392165 msid:user3197027544@host-63f598de webrtctransceiver0\\r\\na=ssrc:470392165 cname:user3197027544@host-63f598de\\r\\na=mid:audio0\\r\\na=fingerprint:sha-256 17:DD:9F:8B:05:69:DF:A4:02:47:CC:2B:E9:A6:AC:11:97:91:EC:E5:06:31:51:0D:48:EA:93:99:C4:A5:94:05\\r\\na=rtcp-mux-only\\r\\nm=video 0 UDP/TLS/RTP/SAVPF 96 97 98 99 100\\r\\nc=IN IP4 0.0.0.0\\r\\na=setup:actpass\\r\\na=ice-ufrag:vUFzktWm1LLErNFcNB3ehdAZCQuN3oqN\\r\\na=ice-pwd:cM5+n8ZXC0k142eHJOePxxKloRrtV57K\\r\\na=bundle-only\\r\\na=rtcp-mux\\r\\na=rtcp-rsize\\r\\na=sendonly\\r\\na=rtpmap:96 VP8/90000\\r\\na=rtcp-fb:96 nack\\r\\na=rtcp-fb:96 nack pli\\r\\na=rtcp-fb:96 ccm fir\\r\\na=rtcp-fb:96 transport-cc\\r\\na=extmap:1 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\\r\\na=rtpmap:97 red/90000\\r\\na=rtpmap:98 ulpfec/90000\\r\\na=rtpmap:99 rtx/90000\\r\\na=fmtp:99 apt=96\\r\\na=rtpmap:100 rtx/90000\\r\\na=fmtp:100 apt=97\\r\\na=ssrc-group:FID 1965760769 681748013\\r\\na=ssrc:1965760769 msid:user3197027544@host-63f598de webrtctransceiver1\\r\\na=ssrc:1965760769 cname:user3197027544@host-63f598de\\r\\na=ssrc:681748013 msid:user3197027544@host-63f598de webrtctransceiver1\\r\\na=ssrc:681748013 cname:user3197027544@host-63f598de\\r\\na=mid:video1\\r\\na=fingerprint:sha-256 17:DD:9F:8B:05:69:DF:A4:02:47:CC:2B:E9:A6:AC:11:97:91:EC:E5:06:31:51:0D:48:EA:93:99:C4:A5:94:05\\r\\na=rtcp-mux-only\\r\\n\"}}")))
2025-03-20T14:02:16.152694Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:1 1 UDP 2015363327 192.168.72.123 50653 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.152871Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:2 1 TCP 1015021823 192.168.72.123 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.152921Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:3 1 TCP 1010827519 192.168.72.123 53315 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.152952Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:4 1 UDP 2015363583 10.0.0.163 45424 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.152995Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:5 1 TCP 1015022079 10.0.0.163 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.153024Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:6 1 TCP 1010827775 10.0.0.163 57413 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.153106Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:7 1 UDP 2015363839 fe80::951c:38c9:2ed6:fd01 40488 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.153149Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:8 1 TCP 1015022335 fe80::951c:38c9:2ed6:fd01 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.153257Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:9 1 TCP 1010828031 fe80::951c:38c9:2ed6:fd01 46113 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.153324Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:10 1 UDP 2015364095 fe80::f0a2:e2ff:fe6e:8c1a 60047 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154187Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:11 1 TCP 1015022591 fe80::f0a2:e2ff:fe6e:8c1a 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154224Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:12 1 TCP 1010828287 fe80::f0a2:e2ff:fe6e:8c1a 40827 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154254Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:13 1 UDP 2015364351 fe80::1494:2ff:fe32:1891 54458 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154282Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:14 1 TCP 1015022847 fe80::1494:2ff:fe32:1891 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154314Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:15 1 TCP 1010828543 fe80::1494:2ff:fe32:1891 47457 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154343Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:16 1 UDP 2015364607 fe80::488a:7eff:fe33:2596 49992 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154374Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:17 1 TCP 1015023103 fe80::488a:7eff:fe33:2596 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154471Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:18 1 TCP 1010828799 fe80::488a:7eff:fe33:2596 46807 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154601Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:19 1 UDP 2015364863 fe80::70c9:80ff:fe7d:1a13 43120 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154671Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:20 1 TCP 1015023359 fe80::70c9:80ff:fe7d:1a13 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154745Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:21 1 TCP 1010829055 fe80::70c9:80ff:fe7d:1a13 50785 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154813Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:22 1 UDP 2015365119 fe80::3c:37ff:fe1c:da11 48334 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154874Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:23 1 TCP 1015023615 fe80::3c:37ff:fe1c:da11 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.154940Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:24 1 TCP 1010829311 fe80::3c:37ff:fe1c:da11 37857 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.155006Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:25 1 UDP 2015365375 fe80::ec8b:20ff:fea5:b90d 39059 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.155084Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:26 1 TCP 1015023871 fe80::ec8b:20ff:fea5:b90d 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.155155Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:27 1 TCP 1010829567 fe80::ec8b:20ff:fea5:b90d 37445 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156839Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:28 1 UDP 2015365631 fe80::e4ea:f7ff:fe66:ebd1 34375 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156876Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:29 1 TCP 1015024127 fe80::e4ea:f7ff:fe66:ebd1 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156898Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:30 1 TCP 1010829823 fe80::e4ea:f7ff:fe66:ebd1 60519 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156919Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:31 1 UDP 2015365887 fe80::7889:8cff:fe4e:b691 39037 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156941Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:32 1 TCP 1015024383 fe80::7889:8cff:fe4e:b691 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156965Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:33 1 TCP 1010830079 fe80::7889:8cff:fe4e:b691 51585 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.156988Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:34 1 UDP 2015366143 fe80::e4ac:c8ff:fe55:2964 41129 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157011Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:35 1 TCP 1015024639 fe80::e4ac:c8ff:fe55:2964 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157034Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:36 1 TCP 1010830335 fe80::e4ac:c8ff:fe55:2964 38733 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157056Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:37 1 UDP 2015366399 fe80::6c03:d2ff:fe60:3290 47359 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157078Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:38 1 TCP 1015024895 fe80::6c03:d2ff:fe60:3290 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157099Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:39 1 TCP 1010830591 fe80::6c03:d2ff:fe60:3290 47281 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157121Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:40 1 UDP 2015366655 fe80::5c26:d0ff:fee9:dbdb 35450 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157144Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:41 1 TCP 1015025151 fe80::5c26:d0ff:fee9:dbdb 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157168Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:42 1 TCP 1010830847 fe80::5c26:d0ff:fee9:dbdb 42341 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157191Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:43 1 UDP 2015366911 fe80::9ce0:b1ff:fea2:b7d9 39653 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157213Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:44 1 TCP 1015025407 fe80::9ce0:b1ff:fea2:b7d9 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.157238Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:45 1 TCP 1010831103 fe80::9ce0:b1ff:fea2:b7d9 48621 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.184300Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:46 1 UDP 1679819007 187.180.171.0 10020 typ srflx raddr 192.168.72.123 rport 50653\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.184675Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:47 1 TCP 847249663 187.180.171.0 9 typ srflx raddr 192.168.72.123 rport 9 tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.185020Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:48 1 TCP 843055359 187.180.171.0 53315 typ srflx raddr 192.168.72.123 rport 53315 tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:16.186697Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"sdp\":{\"type\":\"answer\",\"sdp\":\"v=0\\r\\no=mozilla...THIS_IS_SDPARTA-99.0 1445748174313989637 0 IN IP4 0.0.0.0\\r\\ns=-\\r\\nt=0 0\\r\\na=sendrecv\\r\\na=fingerprint:sha-256 26:4A:34:FE:F8:EE:E9:75:B1:82:70:E3:41:61:97:92:C6:77:8F:7C:F4:BB:9C:0C:52:E5:F6:69:C9:AC:C3:6A\\r\\na=group:BUNDLE audio0 video1\\r\\na=ice-options:trickle\\r\\na=msid-semantic:WMS *\\r\\nm=audio 9 UDP/TLS/RTP/SAVPF 101\\r\\nc=IN IP4 0.0.0.0\\r\\na=recvonly\\r\\na=fmtp:101 maxplaybackrate=48000;stereo=1;useinbandfec=1\\r\\na=ice-pwd:316450192c5fe104c2fb0e9f05a78793\\r\\na=ice-ufrag:cfeff6b5\\r\\na=mid:audio0\\r\\na=rtcp-mux\\r\\na=rtpmap:101 opus/48000/2\\r\\na=setup:active\\r\\na=ssrc:3439847407 cname:{a2ef166e-533e-4692-a834-f1d9ec18a84c}\\r\\nm=video 9 UDP/TLS/RTP/SAVPF 96 99 97 100 98\\r\\nc=IN IP4 0.0.0.0\\r\\na=recvonly\\r\\na=extmap:1 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\\r\\na=fmtp:96 max-fs=12288;max-fr=60\\r\\na=fmtp:99 apt=96\\r\\na=fmtp:100 apt=97\\r\\na=ice-pwd:316450192c5fe104c2fb0e9f05a78793\\r\\na=ice-ufrag:cfeff6b5\\r\\na=mid:video1\\r\\na=rtcp-fb:96 nack\\r\\na=rtcp-fb:96 nack pli\\r\\na=rtcp-fb:96 ccm fir\\r\\na=rtcp-fb:96 transport-cc\\r\\na=rtcp-mux\\r\\na=rtcp-rsize\\r\\na=rtpmap:96 VP8/90000\\r\\na=rtpmap:99 rtx/90000\\r\\na=rtpmap:97 red/90000\\r\\na=rtpmap:100 rtx/90000\\r\\na=rtpmap:98 ulpfec/90000\\r\\na=setup:active\\r\\na=ssrc:573764159 cname:{a2ef166e-533e-4692-a834-f1d9ec18a84c}\\r\\n\"}}")))
2025-03-20T14:02:16.193044Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:0 1 UDP 2122187007 192.168.72.152 59914 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.193155Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:2 1 UDP 2122252543 172.17.0.1 57274 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.193212Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:4 1 TCP 2105458943 192.168.72.152 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.193256Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:5 1 TCP 2105524479 172.17.0.1 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.219102Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:1 1 UDP 1685987327 187.180.171.0 10021 typ srflx raddr 192.168.72.152 rport 59914\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.236373Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"candidate:3 1 UDP 1686052863 187.180.171.0 10022 typ srflx raddr 172.17.0.1 rport 57274\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:16.281606Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\",\"ice\":{\"candidate\":\"\",\"sdpMLineIndex\":0,\"sdpMid\":\"audio0\",\"usernameFragment\":\"cfeff6b5\"}}")))
2025-03-20T14:02:20.244543Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"setPeerStatus\",\"roles\":[\"listener\",\"producer\"],\"meta\":{\"name\":\"WebClient-1742479331027\"}}")))
2025-03-20T14:02:20.244611Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::handlers: registered as a producer peer_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed
2025-03-20T14:02:43.297428Z  INFO ThreadId(01) gst_webrtc_signalling_server: Accepting connection from 127.0.0.1:33152
2025-03-20T14:02:43.328214Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: New WebSocket connection this_id=cedae646-126f-44a2-bdf2-544da3fb3b31
2025-03-20T14:02:43.369139Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"startSession\",\"peerId\":\"dc998a98-fb0f-4b14-a039-dbb153dcc2ed\"}")))
2025-03-20T14:02:43.369226Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::handlers: started a session id=facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1 producer_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed consumer_id=cedae646-126f-44a2-bdf2-544da3fb3b31
2025-03-20T14:02:43.370542Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"setPeerStatus\",\"roles\":[],\"meta\":null,\"peerId\":\"cedae646-126f-44a2-bdf2-544da3fb3b31\"}")))
2025-03-20T14:02:43.370649Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::handlers: registered as a producer peer_id=cedae646-126f-44a2-bdf2-544da3fb3b31
2025-03-20T14:02:43.379173Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"sdp\":{\"type\":\"offer\",\"sdp\":\"v=0\\r\\no=mozilla...THIS_IS_SDPARTA-99.0 6766148208940411470 0 IN IP4 0.0.0.0\\r\\ns=-\\r\\nt=0 0\\r\\na=sendrecv\\r\\na=fingerprint:sha-256 00:54:FB:45:45:3B:9A:3E:02:E6:D7:68:B6:CC:F1:49:1B:CC:0C:26:C2:BA:05:AD:2E:45:6F:7E:58:AD:2B:49\\r\\na=group:BUNDLE 0 1\\r\\na=ice-options:trickle\\r\\na=msid-semantic:WMS *\\r\\nm=audio 9 UDP/TLS/RTP/SAVPF 109 9 0 8 101\\r\\nc=IN IP4 0.0.0.0\\r\\na=sendrecv\\r\\na=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level\\r\\na=extmap:2/recvonly urn:ietf:params:rtp-hdrext:csrc-audio-level\\r\\na=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid\\r\\na=fmtp:109 maxplaybackrate=48000;stereo=1;useinbandfec=1\\r\\na=fmtp:101 0-15\\r\\na=ice-pwd:c6e24bf6849f6fc223df7ba5b49b047a\\r\\na=ice-ufrag:298eb7a7\\r\\na=mid:0\\r\\na=msid:{40b0649a-f410-4201-a128-ba3ea6d1084a} {e4e826c9-952a-494d-a648-1b64036289d7}\\r\\na=rtcp-mux\\r\\na=rtpmap:109 opus/48000/2\\r\\na=rtpmap:9 G722/8000/1\\r\\na=rtpmap:0 PCMU/8000\\r\\na=rtpmap:8 PCMA/8000\\r\\na=rtpmap:101 telephone-event/8000\\r\\na=setup:actpass\\r\\na=ssrc:3432223771 cname:{0f080410-b25d-4b60-a05a-8f0334449a44}\\r\\nm=video 9 UDP/TLS/RTP/SAVPF 120 124 121 125 126 127 97 98 99 100 123 122 119\\r\\nc=IN IP4 0.0.0.0\\r\\na=sendrecv\\r\\na=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid\\r\\na=extmap:4 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time\\r\\na=extmap:5 urn:ietf:params:rtp-hdrext:toffset\\r\\na=extmap:6/recvonly http://www.webrtc.org/experiments/rtp-hdrext/playout-delay\\r\\na=extmap:7 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\\r\\na=fmtp:126 profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1\\r\\na=fmtp:97 profile-level-id=42e01f;level-asymmetry-allowed=1\\r\\na=fmtp:120 max-fs=12288;max-fr=60\\r\\na=fmtp:124 apt=120\\r\\na=fmtp:121 max-fs=12288;max-fr=60\\r\\na=fmtp:125 apt=121\\r\\na=fmtp:127 apt=126\\r\\na=fmtp:98 apt=97\\r\\na=fmtp:100 apt=99\\r\\na=fmtp:119 apt=122\\r\\na=ice-pwd:c6e24bf6849f6fc223df7ba5b49b047a\\r\\na=ice-ufrag:298eb7a7\\r\\na=mid:1\\r\\na=msid:{40b0649a-f410-4201-a128-ba3ea6d1084a} {6618abd1-05f9-41bd-b980-9e95342a3b19}\\r\\na=rtcp-fb:120 nack\\r\\na=rtcp-fb:120 nack pli\\r\\na=rtcp-fb:120 ccm fir\\r\\na=rtcp-fb:120 goog-remb\\r\\na=rtcp-fb:120 transport-cc\\r\\na=rtcp-fb:121 nack\\r\\na=rtcp-fb:121 nack pli\\r\\na=rtcp-fb:121 ccm fir\\r\\na=rtcp-fb:121 goog-remb\\r\\na=rtcp-fb:121 transport-cc\\r\\na=rtcp-fb:126 nack\\r\\na=rtcp-fb:126 nack pli\\r\\na=rtcp-fb:126 ccm fir\\r\\na=rtcp-fb:126 goog-remb\\r\\na=rtcp-fb:126 transport-cc\\r\\na=rtcp-fb:97 nack\\r\\na=rtcp-fb:97 nack pli\\r\\na=rtcp-fb:97 ccm fir\\r\\na=rtcp-fb:97 goog-remb\\r\\na=rtcp-fb:97 transport-cc\\r\\na=rtcp-fb:99 nack\\r\\na=rtcp-fb:99 nack pli\\r\\na=rtcp-fb:99 ccm fir\\r\\na=rtcp-fb:99 goog-remb\\r\\na=rtcp-fb:99 transport-cc\\r\\na=rtcp-fb:123 nack\\r\\na=rtcp-fb:123 nack pli\\r\\na=rtcp-fb:123 ccm fir\\r\\na=rtcp-fb:123 goog-remb\\r\\na=rtcp-fb:123 transport-cc\\r\\na=rtcp-fb:122 nack\\r\\na=rtcp-fb:122 nack pli\\r\\na=rtcp-fb:122 ccm fir\\r\\na=rtcp-fb:122 goog-remb\\r\\na=rtcp-fb:122 transport-cc\\r\\na=rtcp-mux\\r\\na=rtcp-rsize\\r\\na=rtpmap:120 VP8/90000\\r\\na=rtpmap:124 rtx/90000\\r\\na=rtpmap:121 VP9/90000\\r\\na=rtpmap:125 rtx/90000\\r\\na=rtpmap:126 H264/90000\\r\\na=rtpmap:127 rtx/90000\\r\\na=rtpmap:97 H264/90000\\r\\na=rtpmap:98 rtx/90000\\r\\na=rtpmap:99 AV1/90000\\r\\na=rtpmap:100 rtx/90000\\r\\na=rtpmap:123 ulpfec/90000\\r\\na=rtpmap:122 red/90000\\r\\na=rtpmap:119 rtx/90000\\r\\na=setup:actpass\\r\\na=ssrc:3702260413 cname:{0f080410-b25d-4b60-a05a-8f0334449a44}\\r\\na=ssrc:3309647578 cname:{0f080410-b25d-4b60-a05a-8f0334449a44}\\r\\na=ssrc-group:FID 3702260413 3309647578\\r\\n\"}}")))
2025-03-20T14:02:43.380296Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:0 1 UDP 2122187007 192.168.72.152 39139 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.380944Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:2 1 UDP 2122252543 172.17.0.1 36081 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.383507Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:4 1 TCP 2105458943 192.168.72.152 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.383753Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:5 1 TCP 2105524479 172.17.0.1 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.383827Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:0 2 UDP 2122187006 192.168.72.152 60392 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.383959Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:2 2 UDP 2122252542 172.17.0.1 43117 typ host\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.384058Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:4 2 TCP 2105458942 192.168.72.152 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.384107Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:5 2 TCP 2105524478 172.17.0.1 9 typ host tcptype active\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.384207Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:0 1 UDP 2122187007 192.168.72.152 38347 typ host\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386543Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:2 1 UDP 2122252543 172.17.0.1 58206 typ host\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386786Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:4 1 TCP 2105458943 192.168.72.152 9 typ host tcptype active\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386831Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:5 1 TCP 2105524479 172.17.0.1 9 typ host tcptype active\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386868Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:0 2 UDP 2122187006 192.168.72.152 43155 typ host\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386905Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:2 2 UDP 2122252542 172.17.0.1 54408 typ host\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.386962Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:4 2 TCP 2105458942 192.168.72.152 9 typ host tcptype active\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.388474Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:5 2 TCP 2105524478 172.17.0.1 9 typ host tcptype active\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.400232Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:1 1 UDP 1685987327 187.180.171.0 10031 typ srflx raddr 192.168.72.152 rport 39139\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.440077Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:3 1 UDP 1686052863 187.180.171.0 10032 typ srflx raddr 172.17.0.1 rport 36081\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.480032Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:1 2 UDP 1685987326 187.180.171.0 10033 typ srflx raddr 192.168.72.152 rport 60392\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.520884Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:3 2 UDP 1686052862 187.180.171.0 10034 typ srflx raddr 172.17.0.1 rport 43117\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.547631Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"\",\"sdpMLineIndex\":0,\"sdpMid\":\"0\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.559986Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:1 1 UDP 1685987327 187.180.171.0 10035 typ srflx raddr 192.168.72.152 rport 38347\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.600406Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:3 1 UDP 1686052863 187.180.171.0 10036 typ srflx raddr 172.17.0.1 rport 58206\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.640317Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:1 2 UDP 1685987326 187.180.171.0 10037 typ srflx raddr 192.168.72.152 rport 43155\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.680076Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:3 2 UDP 1686052862 187.180.171.0 10039 typ srflx raddr 172.17.0.1 rport 54408\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.698852Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"\",\"sdpMLineIndex\":1,\"sdpMid\":\"1\",\"usernameFragment\":\"298eb7a7\"}}")))
2025-03-20T14:02:43.728506Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"sdp\":{\"type\":\"answer\",\"sdp\":\"v=0\\r\\no=- 6766148208940411470 0 IN IP4 0.0.0.0\\r\\ns=-\\r\\nt=0 0\\r\\na=ice-options:trickle\\r\\na=group:BUNDLE 0 1\\r\\nm=audio 9 UDP/TLS/RTP/SAVPF 109\\r\\nc=IN IP4 0.0.0.0\\r\\na=ice-ufrag:TIXOcM2GFGjZE3fYHxKw9xYXlixATkih\\r\\na=ice-pwd:TDZrT6KDWH+r6fJOvCWFZ4BW04dOGKil\\r\\na=mid:0\\r\\na=rtcp-mux\\r\\na=setup:active\\r\\na=rtpmap:109 OPUS/48000/2\\r\\na=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level\\r\\na=extmap:2/recvonly urn:ietf:params:rtp-hdrext:csrc-audio-level\\r\\na=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid\\r\\na=fmtp:109 maxplaybackrate=48000;stereo=1;useinbandfec=1\\r\\na=recvonly\\r\\na=fingerprint:sha-256 D8:2E:75:B5:E2:FB:05:25:44:11:AB:D3:AD:24:D5:64:05:79:8D:39:C9:E1:A3:3F:FA:F5:5A:44:8F:AE:8E:CA\\r\\nm=video 9 UDP/TLS/RTP/SAVPF 120 123 122 119 124\\r\\nc=IN IP4 0.0.0.0\\r\\na=ice-ufrag:TIXOcM2GFGjZE3fYHxKw9xYXlixATkih\\r\\na=ice-pwd:TDZrT6KDWH+r6fJOvCWFZ4BW04dOGKil\\r\\na=mid:1\\r\\na=rtcp-mux\\r\\na=setup:active\\r\\na=rtpmap:120 VP8/90000\\r\\na=rtcp-fb:120 nack\\r\\na=rtcp-fb:120 nack pli\\r\\na=rtcp-fb:120 ccm fir\\r\\na=rtcp-fb:120 transport-cc\\r\\na=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid\\r\\na=extmap:4 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time\\r\\na=extmap:5 urn:ietf:params:rtp-hdrext:toffset\\r\\na=extmap:6/recvonly http://www.webrtc.org/experiments/rtp-hdrext/playout-delay\\r\\na=extmap:7 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01\\r\\na=fmtp:120 max-fs=12288;max-fr=60\\r\\na=rtpmap:123 ulpfec/90000\\r\\na=rtpmap:122 red/90000\\r\\na=rtpmap:119 rtx/90000\\r\\na=fmtp:119 apt=122\\r\\na=rtpmap:124 rtx/90000\\r\\na=fmtp:124 apt=120\\r\\na=recvonly\\r\\na=fingerprint:sha-256 D8:2E:75:B5:E2:FB:05:25:44:11:AB:D3:AD:24:D5:64:05:79:8D:39:C9:E1:A3:3F:FA:F5:5A:44:8F:AE:8E:CA\\r\\n\"}}")))
2025-03-20T14:02:43.772936Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:1 1 UDP 2015363327 192.168.72.123 48062 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773008Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:2 1 TCP 1015021823 192.168.72.123 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773085Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:3 1 TCP 1010827519 192.168.72.123 49217 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773156Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:4 1 UDP 2015363583 10.0.0.163 55100 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773190Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:5 1 TCP 1015022079 10.0.0.163 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773220Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:6 1 TCP 1010827775 10.0.0.163 34905 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773253Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:7 1 UDP 2015363839 fe80::951c:38c9:2ed6:fd01 48571 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773286Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:8 1 TCP 1015022335 fe80::951c:38c9:2ed6:fd01 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773319Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:9 1 TCP 1010828031 fe80::951c:38c9:2ed6:fd01 35223 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773351Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:10 1 UDP 2015364095 fe80::f0a2:e2ff:fe6e:8c1a 48016 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773381Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:11 1 TCP 1015022591 fe80::f0a2:e2ff:fe6e:8c1a 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773412Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:12 1 TCP 1010828287 fe80::f0a2:e2ff:fe6e:8c1a 34893 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773484Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:13 1 UDP 2015364351 fe80::1494:2ff:fe32:1891 46764 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773540Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:14 1 TCP 1015022847 fe80::1494:2ff:fe32:1891 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773569Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:15 1 TCP 1010828543 fe80::1494:2ff:fe32:1891 38861 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773601Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:16 1 UDP 2015364607 fe80::488a:7eff:fe33:2596 35240 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773654Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:17 1 TCP 1015023103 fe80::488a:7eff:fe33:2596 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773704Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:18 1 TCP 1010828799 fe80::488a:7eff:fe33:2596 48381 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773796Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:19 1 UDP 2015364863 fe80::70c9:80ff:fe7d:1a13 55778 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773836Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:20 1 TCP 1015023359 fe80::70c9:80ff:fe7d:1a13 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773877Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:21 1 TCP 1010829055 fe80::70c9:80ff:fe7d:1a13 48151 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773947Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:22 1 UDP 2015365119 fe80::3c:37ff:fe1c:da11 55048 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.773990Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:23 1 TCP 1015023615 fe80::3c:37ff:fe1c:da11 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774028Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:24 1 TCP 1010829311 fe80::3c:37ff:fe1c:da11 45897 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774068Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:25 1 UDP 2015365375 fe80::ec8b:20ff:fea5:b90d 57576 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774101Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:26 1 TCP 1015023871 fe80::ec8b:20ff:fea5:b90d 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774133Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:27 1 TCP 1010829567 fe80::ec8b:20ff:fea5:b90d 35221 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774167Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:28 1 UDP 2015365631 fe80::e4ea:f7ff:fe66:ebd1 41206 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774201Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:29 1 TCP 1015024127 fe80::e4ea:f7ff:fe66:ebd1 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774233Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:30 1 TCP 1010829823 fe80::e4ea:f7ff:fe66:ebd1 42275 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774268Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:31 1 UDP 2015365887 fe80::7889:8cff:fe4e:b691 46700 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774297Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:32 1 TCP 1015024383 fe80::7889:8cff:fe4e:b691 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774330Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:33 1 TCP 1010830079 fe80::7889:8cff:fe4e:b691 58971 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774362Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:34 1 UDP 2015366143 fe80::e4ac:c8ff:fe55:2964 57658 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.774761Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:35 1 TCP 1015024639 fe80::e4ac:c8ff:fe55:2964 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775283Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:36 1 TCP 1010830335 fe80::e4ac:c8ff:fe55:2964 43147 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775347Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:37 1 UDP 2015366399 fe80::6c03:d2ff:fe60:3290 60805 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775403Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:38 1 TCP 1015024895 fe80::6c03:d2ff:fe60:3290 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775457Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:39 1 TCP 1010830591 fe80::6c03:d2ff:fe60:3290 54041 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775500Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:40 1 UDP 2015366655 fe80::5c26:d0ff:fee9:dbdb 51925 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775560Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:41 1 TCP 1015025151 fe80::5c26:d0ff:fee9:dbdb 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775594Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:42 1 TCP 1010830847 fe80::5c26:d0ff:fee9:dbdb 38557 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775624Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:43 1 UDP 2015366911 fe80::9ce0:b1ff:fea2:b7d9 36740 typ host\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775652Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:44 1 TCP 1015025407 fe80::9ce0:b1ff:fea2:b7d9 9 typ host tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.775717Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:45 1 TCP 1010831103 fe80::9ce0:b1ff:fea2:b7d9 47299 typ host tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.794811Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:46 1 UDP 1679819007 187.180.171.0 10041 typ srflx raddr 192.168.72.123 rport 48062\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.795005Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:47 1 TCP 847249663 187.180.171.0 9 typ srflx raddr 192.168.72.123 rport 9 tcptype active\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:43.795070Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"peer\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\",\"ice\":{\"candidate\":\"candidate:48 1 TCP 843055359 187.180.171.0 49217 typ srflx raddr 192.168.72.123 rport 49217 tcptype passive\",\"sdpMLineIndex\":0}}")))
2025-03-20T14:02:46.284973Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Pong(b""))
2025-03-20T14:03:07.561423Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"setPeerStatus\",\"roles\":[\"listener\"],\"meta\":{\"name\":\"WebClient-1742479331027\"}}")))
2025-03-20T14:03:07.561529Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::handlers: registered as a producer peer_id=dc998a98-fb0f-4b14-a039-dbb153dcc2ed
2025-03-20T14:03:07.562994Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"endSession\",\"sessionId\":\"facd0d74-cbc8-43ab-ac4a-952c9c3b2ba1\"}")))
2025-03-20T14:03:08.019217Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: Received message Ok(Close(None))
2025-03-20T14:03:08.019282Z  INFO ThreadId(05) gst_plugin_webrtc_signalling::server: connection closed: None this_id=cedae646-126f-44a2-bdf2-544da3fb3b31
2025-03-20T14:03:08.019365Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::handlers: removing peer peer_id=cedae646-126f-44a2-bdf2-544da3fb3b31
2025-03-20T14:03:11.928153Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"endSession\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\"}")))
2025-03-20T14:03:11.928232Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: Received message Ok(Close(None))
2025-03-20T14:03:11.928244Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::server: connection closed: None this_id=0a3f49ff-862e-4c76-b24b-521fd0eba60e
2025-03-20T14:03:11.928425Z  INFO ThreadId(02) gst_plugin_webrtc_signalling::handlers: removing peer peer_id=0a3f49ff-862e-4c76-b24b-521fd0eba60e
2025-03-20T14:03:11.931658Z  INFO ThreadId(04) gst_plugin_webrtc_signalling::server: Received message Ok(Text(Utf8Bytes(b"{\"type\":\"endSession\",\"sessionId\":\"e8d78c32-95a9-4caf-ba05-6e29f0c65890\"}")))
^C⏎
```
</details>

<details>
<summary>Streaming Pipeline output.</summary>

```text
Setting pipeline to PAUSED ...
[2:53:20.202263791] [9143]  INFO Camera camera_manager.cpp:327 libcamera v0.4.0+53-29156679
[2:53:20.212525661] [9147]  INFO RPI pisp.cpp:720 libpisp version v1.1.0 e7974a156008 27-01-2025 (21:50:51)
[2:53:20.305737351] [9147]  INFO RPI pisp.cpp:1179 Registered camera /base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a to CFE device /dev/media0 and ISP device /dev/media2 using PiSP variant BCM2712_C0
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstAudioSrcClock
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-buffer-time = 192000
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0: actual-latency-time = 21333
Redistribute latency...
/GstPipeline:pipeline0/GstAlsaSrc:alsasrc0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:src: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstQueue:queue0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioResample:audioresample0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
/GstPipeline:pipeline0/GstAudioConvert:audioconvert0.GstPad:sink: caps = audio/x-raw, rate=(int)48000, format=(string)S16LE, channels=(int)1, layout=(string)interleaved
Redistribute latency...
/GstPipeline:pipeline0/GstOpusEnc:opusenc0.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0.GstProxyPad:proxypad0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:src: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink0.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync0.GstPad:sink: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:audio_0: caps = audio/x-opus, rate=(int)48000, channels=(int)1, channel-mapping-family=(int)0, stream-count=(int)1, coupled-count=(int)0, streamheader=(buffer)< 4f707573486561640101380180bb0000000000, 4f707573546167731e000000456e636f6465642077697468204753747265616d6572206f707573656e630000000001 >
Redistribute latency...
[2:53:20.451461493] [9152]  INFO Camera camera.cpp:1202 configuring streams: (0) 1280x1080-YUV420
[2:53:20.451644862] [9147]  INFO RPI pisp.cpp:1484 Sensor: /base/axi/pcie@1000120000/rp1/i2c@88000/imx708@1a - Selected sensor format: 2304x1296-SBGGR10_1X10 - Selected CFE format: 2304x1296-PC1B
/GstPipeline:pipeline0/GstLibcameraSrc:libcamerasrc0.GstLibcameraPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:src: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstQueue:queue1.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
/GstPipeline:pipeline0/GstVideoConvert:videoconvert0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
Redistribute latency...
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0.GstProxyPad:proxypad1: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:src: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstAppSink:appsink1.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws/GstClockSync:clocksync1.GstPad:sink: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstWebRTCSink:ws.GstWebRTCSinkPad:video_0: caps = video/x-vp8, profile=(string)0, streamheader=(buffer)< 4f56503830010100050004380000010000010000001e00000001 >, width=(int)1280, height=(int)1080, pixel-aspect-ratio=(fraction)1/1, framerate=(fraction)30/1, interlace-mode=(string)progressive, colorimetry=(string)bt709
/GstPipeline:pipeline0/GstVP8Enc:vp8enc0.GstPad:sink: caps = video/x-raw, format=(string)I420, width=(int)1280, height=(int)1080, colorimetry=(string)bt709, framerate=(fraction)30/1
Redistribute latency...
^Chandling interrupt.
Interrupt: Stopping pipeline ...
Execution ended after 0:01:10.305010909
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

<details>
<summary>HTTP Server output.</summary>

```text
     Index: enabled, Cache: enabled, Cors: disabled, Coop: disabled, Coep: disabled, Range: enabled, Sort: enabled, Threads: 3
          Upload: disabled, CSRF Token:
          Auth: disabled, Compression: disabled
         https: enabled, Cert: pki/olivia-v1_server.p12, Cert-Password:
          Root: /home/cavani/Workspace/programmable-matter-rpi/dev-projects/gst-webrtc/web-app,
    TryFile404:
       Address: https://0.0.0.0:8000
    ======== [2025-03-20 11:02:05] ========
^C⏎
```
</details>

<details>
<summary>Server Audio Consumer output.</summary>

```text
Setting pipeline to PAUSED ...
Pipeline is live and does not need PREROLL ...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
New clock: GstSystemClock
Redistribute latency...
Redistribute latency...
Redistribute latency...
Redistribute latency...
Got EOS from element "pipeline0".
Execution ended after 0:00:24.805394462
Setting pipeline to NULL ...
Freeing pipeline ...
```
</details>

## Project

### GStreamer WebRTC Plugin

[README](./gst-plugins/README.md)

### GStreamer WebRTC Javascript API

[README](./web-app/README.md)

### HTTPS, WSS, Custom CA

[README](./pki/README.md)

