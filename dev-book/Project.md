# Raspberry Pi AI Project

Hardware:

- Raspberry Pi 5
- Storage SSD 1T
- USB Microphone
- USB Speaker
- Raspberry Pi Camera Module 3

Software:

- Raspberry Pi OS Lite 64-bit (Linux 6.6, Debian 12 bookworm)
- Docker Engine 28.0 -> Containers
- K3s v1.31.5+k3s1 -> Orchestration
- Cilium CNI -> Network (multicast)

Application:

- Rust -> programming language
- GStreamer -> Audio/Video pipelines
- Zenoh -> Message broker
- Hugging Face Candle -> Machine Learning framework
- Dioxus or Tauri/Leptos -> App framework (web/mobile/desktop)

Services:

- WebRTC
    - Produce:
        - Camera (video)
        - Microphone (audio)
        - Speaker (audio)
    - Consume:
        - Video
        - Audio
- UI App (web/mobile/desktop)

Intelligence:

- Speech to text (Whisper?)
- Text to speech (MetaVoice?)
- Video understanding (VLM, PaliGemma?)
- Language understanding (LLM, Gemma?)
