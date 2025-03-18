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

### WebRTC Signalling Server

```sh
cargo install gst-plugin-webrtc-signalling

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

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

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

Server output.

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

Client output.

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

### Audio and Video Test

Codecs:

- Audio: Opus
- Video: VP8

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

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

Server output.

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

Client output.

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

### Camera and Microphone

Codecs:

- Audio: Opus
- Video: VP8

```sh
#
# Server (transmission)
#

# Raspberry Pi OS Lite (November 19th 2024)

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

Server output.

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

Client output.

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

### Web App

<https://crates.io/crates/simple-http-server>

```sh
# Start Signalling server


gst-webrtc-signalling-server --cert pki/olivia-v1_server.p12


# Start Server streaming


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


# Start Server web app


cargo binstall simple-http-server

simple-http-server -i web-app/ --cert pki/olivia-v1_server.p12


# Client Browser


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
#   Filename                 /home/cavani/Workspace/programmable-matter-rpi/dev-projects/gst-webrtc/gst-plugins/aarch64-linux-gnu/libgstrswebrtc.so
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
#   Filename                 /home/cavani/Workspace/programmable-matter-rpi/dev-projects/gst-webrtc/gst-plugins/aarch64-linux-gnu/libgstrsrtp.so
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

### Gst WebRTC API JS

<https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/tree/0.13.5/net/webrtc/gstwebrtc-api>

```sh
sudo apt install --no-install-recommends nodejs npm

git clone --depth 1 --branch 0.13.5 https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs.git
# rm -rf gst-plugins-rs/.git

cd gst-plugins-rs/net/webrtc/gstwebrtc-api

npm install

# npm WARN deprecated acorn-import-assertions@1.9.0: package has been renamed to acorn-import-attributes
# npm WARN deprecated inflight@1.0.6: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
# npm WARN deprecated glob@7.2.0: Glob versions prior to v9 are no longer supported
# npm WARN deprecated @humanwhocodes/config-array@0.11.14: Use @eslint/config-array instead
# npm WARN deprecated @humanwhocodes/object-schema@2.0.3: Use @eslint/object-schema instead
# npm WARN deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated rimraf@2.7.1: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
# npm WARN deprecated eslint@8.49.0: This version is no longer supported. Please see https://eslint.org/version-support for other options.
# 
# > gstwebrtc-api@2.0.0 postinstall
# > patch-package
# 
# patch-package 8.0.0
# Applying patches...
# webrtc-adapter@8.2.3 ✔
# 
# added 533 packages, and audited 534 packages in 41s
# 
# 4 moderate severity vulnerabilities
# 
# To address all issues, run:
#   npm audit fix --force
# 
# Run `npm audit` for details.

npm run make

# > gstwebrtc-api@2.0.0 make
# > npm run check && npm run build && npm run docs
#
#
# > gstwebrtc-api@2.0.0 check
# > eslint src
#
#
# > gstwebrtc-api@2.0.0 build
# > rimraf dist && webpack
#
# asset gstwebrtc-api-2.0.0.min.js 93.9 KiB [emitted] [minimized] (name: gstwebrtc-api) 1 related asset
# asset index.html 12.7 KiB [emitted]
# orphan modules 181 KiB [orphan] 20 modules
# runtime modules 937 bytes 4 modules
# cacheable modules 207 KiB
#   ./src/index.js + 20 modules 182 KiB [built] [code generated]
#   ./node_modules/sdp/sdp.js 24.7 KiB [built] [code generated]
# webpack 5.88.2 compiled successfully in 2420 ms
#
#> gstwebrtc-api@2.0.0 docs
#> rimraf docs && jsdoc src/*.js -d docs/ -p package.json -R README.md

ls -alh dist/

# total 436K
# drwxr-xr-x 2 cavani cavani 4.0K Mar 15 11:09 ./
# drwxr-xr-x 8 cavani cavani 4.0K Mar 15 11:09 ../
# -rw-r--r-- 1 cavani cavani  94K Mar 15 11:09 gstwebrtc-api-2.0.0.min.js
# -rw-r--r-- 1 cavani cavani 315K Mar 15 11:09 gstwebrtc-api-2.0.0.min.js.map
# -rw-r--r-- 1 cavani cavani  13K Mar 15 11:09 index.html

mkdir -p ../../../../web-app/
cp dist/* ../../../../web-app/
```

### HTTPS

Requirements:

- `mediaDevices` (Camera, Microphone) requires HTTPS
- `Websocket` uses WSS when using HTTPS

Implications:

- HTTP Server should use TLS configuration
- Signalling Server should use TLS configuration
- `webrtcsink` should connect to the Signalling Serverusing WSS

<https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices>

> The mediaDevices read-only property of the Navigator interface returns a MediaDevices
> object, which provides access to connected media input devices like cameras and
> microphones, as well as screen sharing.
>
> Secure context: This feature is available only in secure contexts (HTTPS), in some or all
> supporting browsers.
>
> Firefox, Chrome, Safari, WevView requires HTTPS.

<https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_client_applications#security_considerations>

> WebSockets should not be used in a mixed content environment; that is, you shouldn't
> open a non-secure WebSocket connection from a page loaded using HTTPS or vice versa. Most
> browsers now only allow secure WebSocket connections, and no longer support using them in
> insecure contexts.

Solution:

- Create a new CA (Certificate Authority) keys
- Create server keys signed by the new CA key
- Import CA Certificate on Linux (ca-certificates package)
- Import CA Certificate into Firefox

Files:

- `[machine_id]_ca.key`: custom CA private key
- `[machine_id]_ca.crt`: custom CA certificate (public key)
- `[machine_id]_server.p12`: Server Key Store (signed by the custom CA)


```sh
sudo apt install openssl

mkdir -p pki
cd pki/


# Create a new CA keys


# machine_id = olivia-v1

openssl req \
-x509 \
-newkey rsa:4096 \
-sha256 \
-nodes \
-days 3650 \
-keyout olivia-v1_ca.key \
-out olivia-v1_ca.crt \
-subj '/CN=Olivia v1 CA'

openssl x509 -in olivia-v1_ca.crt -noout -text

# Certificate:
#     Data:
#         Version: 3 (0x2)
#         Serial Number:
#             47:97:a6:fc:ad:56:27:07:39:86:f5:85:73:75:06:6e:81:f8:bb:a2
#         Signature Algorithm: sha256WithRSAEncryption
#         Issuer: CN = Olivia v1 CA
#         Validity
#             Not Before: Mar 18 18:02:57 2025 GMT
#             Not After : Mar 16 18:02:57 2035 GMT
#         Subject: CN = Olivia v1 CA
#         Subject Public Key Info:
#             Public Key Algorithm: rsaEncryption
#                 Public-Key: (4096 bit)
#                 Modulus:
#                     00:9c:19:a4:f3:09:8c:1c:42:fd:a5:c0:16:b5:a5:
#                     28:2a:ca:69:cc:98:df:cc:1f:55:f6:48:b5:86:71:
#                     ff:a1:14:01:6a:f3:4f:72:43:2f:8c:cf:d2:1e:e5:
#                     aa:10:5a:b9:57:32:bf:2c:0e:58:5b:ec:8a:84:64:
#                     c5:45:4b:9f:5e:2c:b3:ab:7e:68:a6:be:0f:54:ce:
#                     1e:ad:55:50:33:c1:4b:01:a9:45:ae:fe:e1:cb:2f:
#                     71:db:80:9f:5a:99:ae:23:81:b7:0e:49:4c:91:07:
#                     da:c5:89:b0:1c:aa:47:26:8d:bf:d4:a1:41:14:3c:
#                     4c:06:4b:5b:27:46:37:a7:50:b8:9e:23:61:bf:3a:
#                     3b:e7:a6:83:46:b9:5d:00:e9:f5:f1:de:b1:b8:06:
#                     e1:b2:75:68:3f:4a:df:6a:9b:dc:fa:7c:d1:5b:6a:
#                     20:27:3f:66:f4:88:41:45:77:74:dd:b4:a7:e4:fc:
#                     fb:93:27:6e:5e:73:b6:47:7b:7d:f8:99:ba:2d:a1:
#                     34:0d:dc:e4:f8:a7:0b:bb:0f:f9:16:ef:f4:21:08:
#                     99:0b:ae:3b:9d:e5:9b:08:39:14:c7:d3:74:29:a3:
#                     e0:28:57:d4:b5:10:dc:b5:19:0d:23:7c:0f:e7:a3:
#                     f6:67:40:fc:d4:b8:b9:34:2a:b2:52:9f:83:38:fa:
#                     f0:00:a6:ed:40:6c:c9:eb:a1:d6:a0:31:13:d2:f9:
#                     fa:af:5a:dd:ed:19:69:06:94:4d:9e:e9:38:57:0c:
#                     e3:4c:2b:5d:e2:3a:f7:6c:b6:60:7d:f2:3f:5b:f5:
#                     a8:52:aa:54:6c:bd:da:8b:c6:d7:d9:7f:f9:40:83:
#                     70:04:68:95:49:73:96:ca:ab:5a:29:48:af:b9:66:
#                     df:c5:ad:79:ee:c6:24:6a:3d:6a:02:52:b9:42:b3:
#                     5a:fd:c2:0e:44:27:13:61:50:c5:95:d1:b2:0b:14:
#                     97:1c:ad:cd:e1:fa:c2:c1:e0:a9:8b:c6:60:1c:1e:
#                     1b:e8:da:5f:0c:2f:68:55:53:1f:68:b4:71:7f:31:
#                     ba:4d:59:f2:f2:20:a2:34:93:31:be:bd:07:bb:1b:
#                     de:4a:2f:54:c2:58:5a:e0:74:70:57:23:6c:71:ea:
#                     b7:e5:59:50:00:2b:e3:9e:d0:0e:ca:98:6b:64:d8:
#                     6b:e9:38:96:b4:66:17:3e:8a:03:c6:7d:35:a8:80:
#                     d6:6e:0d:5a:ea:7c:56:56:40:89:f1:8c:16:c0:7a:
#                     21:54:20:7d:e0:60:d5:fe:75:dc:11:37:bf:48:83:
#                     fd:86:7b:e3:4b:7a:cb:f9:5a:34:52:38:e7:74:69:
#                     15:04:01:49:f4:80:ce:77:9d:f0:05:4a:bb:3d:b2:
#                     7d:97:91
#                 Exponent: 65537 (0x10001)
#         X509v3 extensions:
#             X509v3 Subject Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#             X509v3 Authority Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#             X509v3 Basic Constraints: critical
#                 CA:TRUE
#     Signature Algorithm: sha256WithRSAEncryption
#     Signature Value:
#         34:b9:9d:61:07:53:4a:d5:87:05:cb:93:11:72:ad:da:35:21:
#         46:e0:3a:16:aa:4f:c9:d7:74:81:bb:56:01:6c:5e:6d:10:c2:
#         f6:a2:57:12:56:7d:04:80:5c:40:25:ee:6f:85:88:36:31:a1:
#         c8:7d:90:e1:d1:7b:00:d7:00:cf:87:7b:36:f0:bd:76:ce:cf:
#         43:62:6e:30:b9:e1:d2:67:e0:06:df:bc:8c:0b:0a:7d:fd:07:
#         5f:67:b8:4c:39:73:d5:d4:38:f0:92:bb:0e:c3:1e:b3:06:9e:
#         f5:90:ca:c6:8a:4e:2b:30:59:39:4b:00:94:7f:e9:a2:72:80:
#         ea:db:44:69:74:b3:31:f6:f0:59:d7:ba:e5:e0:68:8b:75:f7:
#         e6:e3:d5:56:37:e9:fb:eb:5e:77:16:25:10:15:1f:de:9e:b6:
#         b8:66:6e:4a:1c:8a:97:8c:f0:3c:7c:81:f7:80:c1:d1:03:cd:
#         9f:a2:d5:5a:61:bf:7d:f4:da:a7:a9:89:80:4d:e5:84:ea:a4:
#         ed:b2:90:a4:9f:30:23:ab:25:6b:2a:1a:46:ae:53:11:84:73:
#         24:75:c7:f6:72:54:79:e6:6d:83:84:c5:aa:cb:6f:69:fa:fb:
#         01:be:ff:dd:20:b2:fc:48:f9:e7:4f:1a:ec:be:81:4b:6b:d3:
#         d0:f9:96:53:26:80:80:e8:08:92:66:d4:31:d0:49:1d:6f:ee:
#         7d:5a:50:f9:7b:81:55:7d:bd:90:72:b9:d4:da:2f:58:56:6d:
#         25:0a:1e:40:83:2e:90:85:97:3a:ec:48:6f:6b:12:be:13:6a:
#         78:f4:85:29:46:c8:1d:4b:16:b2:c1:07:ea:37:4c:12:f6:a8:
#         f2:2a:af:f1:a9:a9:43:5c:ea:26:e3:1f:36:3f:80:01:4d:bc:
#         36:93:a7:6d:53:11:af:ab:a9:9c:ce:c0:f8:5f:f7:16:86:74:
#         f2:2a:a6:66:96:ab:11:e9:e2:be:2c:67:64:f8:1a:96:6e:7e:
#         cb:f6:0a:68:4f:53:f3:af:05:33:30:19:e5:8e:b7:c3:6b:24:
#         d6:70:f0:f1:75:1c:85:e5:0c:ce:33:b1:f9:90:82:9a:8e:ab:
#         ac:ca:16:ca:49:0a:fc:34:fa:40:2b:54:e5:5c:5d:e6:95:18:
#         8a:d3:51:80:00:cf:b2:3e:d3:21:de:bd:b0:7b:a7:c5:03:31:
#         32:2d:08:95:5d:50:9a:9c:35:5c:a7:31:59:cd:0c:80:be:1e:
#         5a:36:95:4e:7f:3d:85:32:64:f3:f4:25:73:67:11:98:80:83:
#         a7:e3:40:b9:f7:7a:b3:b7:9e:95:15:4a:11:54:8e:6e:69:53:
#         f3:29:d8:e5:71:2d:fb:e


# Create the server keys


# CSR (Certificate Signing Request)

openssl req \
-newkey rsa:4096 \
-sha256 \
-nodes \
-keyout olivia-v1_server.key \
-out olivia-v1_server.csr \
-subj '/CN=*.olivia-v1.machine-domain'


# Subject Alternate Names
# https://man.openbsd.org/x509v3.cnf.5#Subject_alternative_name

# Firefox
# Error code: SSL_ERROR_BAD_CERT_DOMAIN
# Firefox does not trust this site because it uses a certificate that is not valid for
# webrtc.olivia-v1.machine-domain:8000.

echo 'subjectAltName=DNS:webrtc.olivia-v1.machine-domain' > olivia-v1_server.cnf


# CA Signing

openssl x509 \
-req \
-days 3650 \
-in olivia-v1_server.csr \
-CA olivia-v1_ca.crt \
-CAkey olivia-v1_ca.key \
-CAcreateserial \
-extfile olivia-v1_server.cnf \
-out olivia-v1_server.crt

openssl x509 -in olivia-v1_server.crt -noout -text

# Certificate:
#     Data:
#         Version: 3 (0x2)
#         Serial Number:
#             06:7a:6b:91:85:06:82:50:f6:4b:7b:aa:71:98:d1:da:49:6d:a4:11
#         Signature Algorithm: sha256WithRSAEncryption
#         Issuer: CN = Olivia v1 CA
#         Validity
#             Not Before: Mar 18 18:06:15 2025 GMT
#             Not After : Mar 16 18:06:15 2035 GMT
#         Subject: CN = *.olivia-v1.machine-domain
#         Subject Public Key Info:
#             Public Key Algorithm: rsaEncryption
#                 Public-Key: (4096 bit)
#                 Modulus:
#                     00:a9:9b:58:06:3a:16:a8:d8:81:0e:c3:0b:a1:2d:
#                     0c:3f:2f:56:47:bb:85:c3:46:bb:91:a9:a4:e4:ae:
#                     ab:1b:df:e5:6f:d7:41:6c:61:78:3f:b4:a2:82:81:
#                     fa:f5:81:bc:9d:cd:ba:8e:c4:da:95:56:5b:56:4c:
#                     54:a3:f4:57:a2:49:46:3d:68:d4:1d:b5:a9:35:7b:
#                     c4:35:21:90:a8:f2:c0:4a:e0:9e:f5:ce:1d:97:88:
#                     16:4e:fe:c6:42:82:af:de:84:1f:1c:53:25:0c:2a:
#                     98:a3:df:1c:8c:32:f3:3a:5a:2d:6c:74:c4:ea:69:
#                     8e:61:56:31:c1:8b:1d:8b:0d:2d:dd:bd:23:02:c5:
#                     8d:ba:c1:98:52:d4:5b:57:55:98:2f:b7:08:e7:f7:
#                     6a:4f:d8:b1:2e:e0:f0:78:d7:81:e2:c0:bf:26:52:
#                     0c:f4:09:a5:09:35:ad:40:23:c8:2f:0a:0f:b8:60:
#                     db:ba:4b:f0:ea:f9:ef:6b:77:b7:ef:61:90:ad:ca:
#                     d4:1d:95:62:4b:31:f3:b3:64:ba:31:07:3b:00:ec:
#                     9a:d2:7e:32:96:37:62:77:35:e4:2f:9f:d8:d7:f0:
#                     f3:ad:f8:e6:49:3f:ed:5a:29:04:8a:42:fc:4b:fc:
#                     45:b1:3e:1a:a6:e3:a4:46:37:6c:ec:1f:00:7b:21:
#                     5e:70:46:28:14:a0:7d:a5:14:53:4b:0f:c2:f1:1b:
#                     db:30:db:4e:87:a9:84:ef:72:ec:8c:2e:52:5f:44:
#                     e9:7b:7f:b0:26:f7:2d:65:45:cc:e8:ef:78:58:4e:
#                     c9:1d:19:6b:e4:6e:1e:69:fe:e7:91:f1:5f:8d:c1:
#                     d9:c0:e1:04:b0:62:76:45:66:e2:b6:6b:ca:fd:77:
#                     73:c2:38:66:d4:95:df:fa:ac:cb:8f:11:ee:85:74:
#                     17:f6:33:00:53:84:a1:04:64:36:f3:8e:a8:27:6b:
#                     aa:8c:88:91:c1:17:04:55:07:ac:52:96:a1:54:dd:
#                     31:27:c5:0d:a0:3d:b0:5f:e0:62:01:b2:95:4c:24:
#                     e4:98:5c:47:c2:e5:60:29:73:38:9d:c7:ff:41:2a:
#                     f5:71:8c:c3:27:9e:dd:04:2b:2e:50:89:2f:2a:47:
#                     af:6f:3a:1d:2d:38:b3:90:e2:cb:3a:fa:d2:a6:18:
#                     5c:1e:f7:35:b0:16:18:ec:ce:e6:b1:46:57:6a:9b:
#                     2b:45:a3:b6:7a:bf:27:93:86:41:2c:4e:00:eb:d3:
#                     24:80:09:bb:65:e1:e4:f8:94:16:3f:55:b5:bd:15:
#                     bb:e0:67:f5:a2:ea:01:e7:86:cc:69:17:c6:12:00:
#                     bd:ae:0f:11:cd:a6:5d:a1:74:2d:ab:53:ec:81:71:
#                     0d:d3:8f
#                 Exponent: 65537 (0x10001)
#         X509v3 extensions:
#             X509v3 Subject Alternative Name:
#                 DNS:webrtc.olivia-v1.machine-domain
#             X509v3 Subject Key Identifier:
#                 07:14:B3:C9:33:1B:98:41:46:3E:8E:DB:68:6D:60:18:E8:03:41:AA
#             X509v3 Authority Key Identifier:
#                 72:EF:56:AB:4B:FC:C0:F6:84:D1:DF:19:C4:00:A6:37:CB:00:8A:9B
#     Signature Algorithm: sha256WithRSAEncryption
#     Signature Value:
#         42:bc:5b:55:2f:2a:0b:7e:7c:66:7e:28:78:4c:43:5d:de:0c:
#         b7:1a:94:b0:2d:38:71:e6:10:f9:c8:43:2b:83:d5:0e:dd:8f:
#         8d:35:b2:c7:b7:5b:9c:f1:e6:f8:c8:3e:2d:a9:52:d4:28:8d:
#         97:42:8f:f2:f1:f2:5c:71:b1:df:ef:2e:9a:71:f2:06:74:10:
#         10:25:31:9d:af:b6:83:6c:6f:c4:fb:e4:ce:35:f2:cc:ca:54:
#         26:18:f4:0e:df:94:b1:02:ea:ad:8a:50:9e:54:91:4a:c3:d8:
#         9a:62:ff:eb:ee:18:c5:0f:79:47:ff:bd:a1:e9:87:b8:ca:4c:
#         2b:41:c5:9b:74:12:b8:fb:96:4f:d4:96:09:f0:77:e3:eb:e9:
#         e5:8e:a6:f4:b2:bb:90:9c:d9:7b:80:fa:bc:9f:6e:f1:c3:ff:
#         d7:36:13:b8:ab:07:5b:42:19:62:00:14:73:62:10:26:e5:96:
#         f1:11:ed:a4:1d:bb:d0:53:bc:43:52:b8:e1:ec:38:6e:c9:21:
#         66:14:8d:f3:bb:f3:26:9a:d4:46:87:cd:79:ad:fb:b1:3b:95:
#         02:7a:57:bb:1f:fb:9b:7c:32:78:d1:d4:f7:f7:43:de:d1:42:
#         51:fe:e6:fc:36:78:7b:36:00:ad:c8:5d:f4:78:45:1b:ff:37:
#         2c:47:8b:4a:ef:cc:5c:72:51:1c:4f:de:d8:37:3c:af:6f:10:
#         24:83:64:d7:ce:c4:15:3e:4f:93:93:f8:f1:34:30:ee:a5:24:
#         be:a2:e0:89:6c:6f:0a:30:f9:be:4a:c9:7f:c1:49:3f:7c:5e:
#         ec:f4:51:20:2d:c9:f7:b0:15:93:00:02:24:7c:dc:7a:e9:f4:
#         01:9b:10:67:e4:ad:c5:80:41:0e:4d:da:3d:47:70:4a:df:cf:
#         46:ba:ac:8f:42:36:77:f0:63:af:0f:50:43:24:dd:36:96:f4:
#         8c:bc:b0:67:5b:32:b1:aa:f8:6e:30:3f:38:a2:90:56:63:4b:
#         a1:11:0b:e4:c6:b8:53:b1:72:55:84:43:8d:69:f3:eb:26:4e:
#         03:63:9b:8c:48:e6:c9:39:2d:fa:5b:00:9b:3f:02:54:18:8b:
#         72:2e:65:94:ef:9b:e6:b1:54:06:0f:f1:5b:f4:d1:39:b5:86:
#         d6:e4:29:bc:00:bd:0e:c1:42:ae:7d:64:95:24:24:ca:de:90:
#         fd:8b:1c:d3:f4:12:2f:ec:45:1d:b2:87:59:f6:59:da:8a:89:
#         47:9c:35:de:b9:ae:73:99:73:97:66:dc:85:66:62:7e:83:9f:
#         0e:b9:1f:94:0e:1d:04:d6:45:ce:bf:ff:27:fd:b3:a2:08:71:
#         ea:9f:57:78:51:6f:27:37


# Server key store file (PKCS#12)

openssl pkcs12 \
-export \
-out olivia-v1_server.p12 \
-passout 'pass:' \
-inkey olivia-v1_server.key \
-in olivia-v1_server.crt


# Files

ls -alh olivia-v1_*

# -rw-r--r-- 1 root root 1.8K Mar 18 18:02 olivia-v1_ca.crt
# -rw------- 1 root root 3.2K Mar 18 18:02 olivia-v1_ca.key
# -rw-r--r-- 1 root root   41 Mar 18 18:06 olivia-v1_ca.srl
# -rw-r--r-- 1 root root   51 Mar 18 18:05 olivia-v1_server.cnf
# -rw-r--r-- 1 root root 1.9K Mar 18 18:06 olivia-v1_server.crt
# -rw-r--r-- 1 root root 1.6K Mar 18 18:05 olivia-v1_server.csr
# -rw------- 1 root root 3.2K Mar 18 18:05 olivia-v1_server.key
# -rw------- 1 root root 4.2K Mar 18 18:09 olivia-v1_server.p12


# Debian / Ubuntu - Install Root CA Certificate


sudo mkdir -p /usr/local/share/ca-certificates/olivia-v1

sudo cp olivia-v1_ca.crt /usr/local/share/ca-certificates/olivia-v1/

sudo update-ca-certificates

# Updating certificates in /etc/ssl/certs...
# 1 added, 0 removed; done.
# Running hooks in /etc/ca-certificates/update.d...
# done


# Firefox - Import Root CA Certificate

# 1. Open Settings
# 2. Open Privacy & Security
# 3. Go to Certificates and open View Certificates...
# 4. Go to Authorities and open Import...
# 5. Select olivia-v1_ca.crt
# 6. Check Trust this CA to identify websites.
# 7. Ok


# Server hostname resolution
# (required to validate server certificate)
# Server Certificate: webrtc.olivia-v1.machine-domain

# On Server

sudo sed -i '0,/localhost/s//localhost webrtc.olivia-v1.machine-domain/' /etc/hosts

# On Client
# 192.168.72.123 <- Server IP Address

echo '

# Olivia v1 WebRTC Server (TLS hostname)
192.168.72.123 webrtc.olivia-v1.machine-domain' | \
sudo tee -a /etc/hosts


# Test

# Start HTTP Server with TLS

cargo binstall simple-http-server
simple-http-server --cert olivia-v1_server.p12

#      Index: disabled, Cache: enabled, Cors: disabled, Coop: disabled, Coep: disabled, Range:
#           Upload: disabled, CSRF Token:
#           Auth: disabled, Compression: disabled
#          https: enabled, Cert: olivia-v1_server.p12, Cert-Password:
#           Root: /home/cavani/Workspace/programmable-matter-rpi/dev-projects/gst-webrtc/pki,
#     TryFile404:
#        Address: https://0.0.0.0:8000
#     ======== [2025-03-18 13:50:40] ========
# [2025-03-18 13:50:50] - 192.168.72.152 - 200 - HEAD /


# Run HTTP Client

curl -vI https://webrtc.olivia-v1.machine-domain:8000/

# * Host webrtc.olivia-v1.machine-domain:8000 was resolved.
# * IPv6: (none)
# * IPv4: 192.168.72.123
# *   Trying 192.168.72.123:8000...
# * Connected to webrtc.olivia-v1.machine-domain (192.168.72.123) port 8000
# * ALPN: curl offers h2,http/1.1
# * TLSv1.3 (OUT), TLS handshake, Client hello (1):
# *  CAfile: /etc/ssl/certs/ca-certificates.crt
# *  CApath: /etc/ssl/certs
# * TLSv1.3 (IN), TLS handshake, Server hello (2):
# * TLSv1.2 (IN), TLS handshake, Certificate (11):
# * TLSv1.2 (IN), TLS handshake, Server key exchange (12):
# * TLSv1.2 (IN), TLS handshake, Server finished (14):
# * TLSv1.2 (OUT), TLS handshake, Client key exchange (16):
# * TLSv1.2 (OUT), TLS change cipher, Change cipher spec (1):
# * TLSv1.2 (OUT), TLS handshake, Finished (20):
# * TLSv1.2 (IN), TLS handshake, Finished (20):
# * SSL connection using TLSv1.2 / ECDHE-RSA-CHACHA20-POLY1305 / x25519 / RSASSA-PSS
# * ALPN: server did not agree on a protocol. Uses default.
# * Server certificate:
# *  subject: CN=*.olivia-v1.machine-domain
# *  start date: Mar 18 16:47:47 2025 GMT
# *  expire date: Mar 16 16:47:47 2035 GMT
# *  common name: *.olivia-v1.machine-domain (matched)
# *  issuer: CN=Olivia v1 CA
# *  SSL certificate verify ok.
# *   Certificate level 0: Public key type RSA (4096/152 Bits/secBits), signed using sha256WithRSAEncryption
# *   Certificate level 1: Public key type RSA (4096/152 Bits/secBits), signed using sha256WithRSAEncryption
# * using HTTP/1.x
# > HEAD / HTTP/1.1
# > Host: webrtc.olivia-v1.machine-domain:8000
# > User-Agent: curl/8.9.1
# > Accept: */*
# >
# * Request completely sent off
# < HTTP/1.1 200 OK
# HTTP/1.1 200 OK
# < Content-Length: 1987
# Content-Length: 1987
# < Content-Type: text/html; charset=utf-8
# Content-Type: text/html; charset=utf-8
# < Date: Tue, 18 Mar 2025 16:50:50 GMT
# Date: Tue, 18 Mar 2025 16:50:50 GMT
# <
# 
# * shutting down connection #0
# * TLSv1.2 (OUT), TLS alert, close notify (256):


# Firefox

firefox --private-window https://webrtc.olivia-v1.machine-domain:8000/

# Should open with no security warnings
```
