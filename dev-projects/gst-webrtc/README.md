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


gst-webrtc-signalling-server


# Start Server streaming


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


# Start Server web app


cargo install simple-http-server

simple-http-server -i web-app/


# Client Browser

# 192.168.72.123 <- Server IP Address


firefox --private-window http://192.168.72.123:8000/

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
