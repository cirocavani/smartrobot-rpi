# GStreamer Example - WebRTC

WebRTC (Web Real-Time Communication)

<https://en.wikipedia.org/wiki/WebRTC>

Server (on device):

- Send audio from microphone
- Receive audio and play on speaker
- Send video from camera
- Receive video (discard, save to file, send back on overlay)
- Send audio from another source (file, TTS)
- Send video from another source (file, image)

Client (web browser):

- Send audio from microphone
- Receive audio and play on speaker
- Send video from camera
- Receive video and play on screen

<https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/tree/main/net/webrtc>

<https://crates.io/crates/gst-plugin-webrtc>

<https://crates.io/crates/gst-plugin-rtp>

<https://crates.io/crates/gst-plugin-webrtc-signalling>

<https://docs.rs/crate/gst-plugin-webrtc/latest>

<https://docs.rs/crate/gst-plugin-rtp/latest>

<https://gstreamer.freedesktop.org/documentation/rswebrtc/webrtcsink.html>

<https://gstreamer.freedesktop.org/documentation/rswebrtc/webrtcsrc.html>

## GStreamer setup

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
libcamerasrc camera-name=/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a ! \
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
[1:25:19.178314477] [6377]  INFO RPI pisp.cpp:1179 Registered camera /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a to CFE device /dev/media1 and ISP device /dev/media0 using PiSP variant BCM2712_C0
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
[1:25:19.302522457] [6377]  INFO RPI pisp.cpp:1484 Sensor: /base/axi/pcie@120000/rp1/i2c@88000/imx708@1a - Selected sensor format: 2304x1296-SBGGR10_1X10 - Selected CFE format: 2304x1296-PC1B
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
libcamerasrc camera-name=/base/axi/pcie@120000/rp1/i2c@88000/imx708@1a ! \
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
```

## Project

### Dependencies

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
gstreamer1.0-tools
```

### GStreamer WebRTC Plugin

[README](./gst-plugins/README.md)

### GStreamer WebRTC Javascript API

[README](./web-app/README.md).

### HTTPS, WSS, Custom CA

[README](./pki/README.md)

