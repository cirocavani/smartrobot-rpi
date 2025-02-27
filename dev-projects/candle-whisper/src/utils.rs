use candle_core::utils::{cuda_is_available, metal_is_available};
use candle_core::{Device, Result};

pub fn device(cpu: bool) -> Result<Device> {
    if cpu {
        Ok(Device::Cpu)
    } else if cuda_is_available() {
        Ok(Device::new_cuda(0)?)
    } else if metal_is_available() {
        Ok(Device::new_metal(0)?)
    } else {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            println!(
                "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
            );
        }
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        {
            println!("Running on CPU, to run on GPU, build this example with `--features cuda`");
        }
        Ok(Device::Cpu)
    }
}

pub mod multilingual {
    use candle_core::{D, IndexOp, Result, Tensor};
    use tokenizers::Tokenizer;

    const LANGUAGES: [(&str, &str); 99] = [
        ("en", "english"),
        ("zh", "chinese"),
        ("de", "german"),
        ("es", "spanish"),
        ("ru", "russian"),
        ("ko", "korean"),
        ("fr", "french"),
        ("ja", "japanese"),
        ("pt", "portuguese"),
        ("tr", "turkish"),
        ("pl", "polish"),
        ("ca", "catalan"),
        ("nl", "dutch"),
        ("ar", "arabic"),
        ("sv", "swedish"),
        ("it", "italian"),
        ("id", "indonesian"),
        ("hi", "hindi"),
        ("fi", "finnish"),
        ("vi", "vietnamese"),
        ("he", "hebrew"),
        ("uk", "ukrainian"),
        ("el", "greek"),
        ("ms", "malay"),
        ("cs", "czech"),
        ("ro", "romanian"),
        ("da", "danish"),
        ("hu", "hungarian"),
        ("ta", "tamil"),
        ("no", "norwegian"),
        ("th", "thai"),
        ("ur", "urdu"),
        ("hr", "croatian"),
        ("bg", "bulgarian"),
        ("lt", "lithuanian"),
        ("la", "latin"),
        ("mi", "maori"),
        ("ml", "malayalam"),
        ("cy", "welsh"),
        ("sk", "slovak"),
        ("te", "telugu"),
        ("fa", "persian"),
        ("lv", "latvian"),
        ("bn", "bengali"),
        ("sr", "serbian"),
        ("az", "azerbaijani"),
        ("sl", "slovenian"),
        ("kn", "kannada"),
        ("et", "estonian"),
        ("mk", "macedonian"),
        ("br", "breton"),
        ("eu", "basque"),
        ("is", "icelandic"),
        ("hy", "armenian"),
        ("ne", "nepali"),
        ("mn", "mongolian"),
        ("bs", "bosnian"),
        ("kk", "kazakh"),
        ("sq", "albanian"),
        ("sw", "swahili"),
        ("gl", "galician"),
        ("mr", "marathi"),
        ("pa", "punjabi"),
        ("si", "sinhala"),
        ("km", "khmer"),
        ("sn", "shona"),
        ("yo", "yoruba"),
        ("so", "somali"),
        ("af", "afrikaans"),
        ("oc", "occitan"),
        ("ka", "georgian"),
        ("be", "belarusian"),
        ("tg", "tajik"),
        ("sd", "sindhi"),
        ("gu", "gujarati"),
        ("am", "amharic"),
        ("yi", "yiddish"),
        ("lo", "lao"),
        ("uz", "uzbek"),
        ("fo", "faroese"),
        ("ht", "haitian creole"),
        ("ps", "pashto"),
        ("tk", "turkmen"),
        ("nn", "nynorsk"),
        ("mt", "maltese"),
        ("sa", "sanskrit"),
        ("lb", "luxembourgish"),
        ("my", "myanmar"),
        ("bo", "tibetan"),
        ("tl", "tagalog"),
        ("mg", "malagasy"),
        ("as", "assamese"),
        ("tt", "tatar"),
        ("haw", "hawaiian"),
        ("ln", "lingala"),
        ("ha", "hausa"),
        ("ba", "bashkir"),
        ("jw", "javanese"),
        ("su", "sundanese"),
    ];

    /// Returns the token id for the selected language.
    pub fn detect_language(
        model: &mut crate::Model,
        tokenizer: &Tokenizer,
        mel: &Tensor,
    ) -> Result<u32> {
        let (_bsize, _, seq_len) = mel.dims3()?;
        let mel = mel.narrow(
            2,
            0,
            usize::min(seq_len, model.config().max_source_positions),
        )?;
        let device = mel.device();
        let language_token_ids = LANGUAGES
            .iter()
            .map(|(t, _)| crate::token_id(tokenizer, &format!("<|{t}|>")))
            .collect::<Result<Vec<_>>>()?;
        let sot_token = crate::token_id(tokenizer, crate::m::SOT_TOKEN)?;
        let audio_features = model.encoder_forward(&mel, true)?;
        let tokens = Tensor::new(&[[sot_token]], device)?;
        let language_token_ids = Tensor::new(language_token_ids.as_slice(), device)?;
        let ys = model.decoder_forward(&tokens, &audio_features, true)?;
        let logits = model.decoder_final_linear(&ys.i(..1)?)?.i(0)?.i(0)?;
        let logits = logits.index_select(&language_token_ids, 0)?;
        let probs = candle_nn::ops::softmax(&logits, D::Minus1)?;
        let probs = probs.to_vec1::<f32>()?;
        let mut probs = LANGUAGES.iter().zip(probs.iter()).collect::<Vec<_>>();
        probs.sort_by(|(_, p1), (_, p2)| p2.total_cmp(p1));
        for ((_, language), p) in probs.iter().take(5) {
            println!("{language}: {p}")
        }
        let language = crate::token_id(tokenizer, &format!("<|{}|>", probs[0].0.0))?;
        Ok(language)
    }
}

pub mod pcm_decode {
    use symphonia::core::audio::{AudioBufferRef, Signal};
    use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
    use symphonia::core::conv::FromSample;

    fn conv<T>(
        samples: &mut Vec<f32>,
        data: std::borrow::Cow<symphonia::core::audio::AudioBuffer<T>>,
    ) where
        T: symphonia::core::sample::Sample,
        f32: symphonia::core::conv::FromSample<T>,
    {
        samples.extend(data.chan(0).iter().map(|v| f32::from_sample(*v)))
    }

    pub(crate) fn pcm_decode<P: AsRef<std::path::Path>>(
        path: P,
    ) -> anyhow::Result<(Vec<f32>, u32)> {
        // Open the media source.
        let src = std::fs::File::open(path)?;

        // Create the media source stream.
        let mss = symphonia::core::io::MediaSourceStream::new(Box::new(src), Default::default());

        // Create a probe hint using the file's extension. [Optional]
        let hint = symphonia::core::probe::Hint::new();

        // Use the default options for metadata and format readers.
        let meta_opts: symphonia::core::meta::MetadataOptions = Default::default();
        let fmt_opts: symphonia::core::formats::FormatOptions = Default::default();

        // Probe the media source.
        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;
        // Get the instantiated format reader.
        let mut format = probed.format;

        // Find the first audio track with a known (decodeable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks");

        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");
        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap_or(0);
        let mut pcm_data = Vec::new();
        // The decode loop.
        while let Ok(packet) = format.next_packet() {
            // Consume any new metadata that has been read since the last packet.
            while !format.metadata().is_latest() {
                format.metadata().pop();
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }
            match decoder.decode(&packet)? {
                AudioBufferRef::F32(buf) => pcm_data.extend(buf.chan(0)),
                AudioBufferRef::U8(data) => conv(&mut pcm_data, data),
                AudioBufferRef::U16(data) => conv(&mut pcm_data, data),
                AudioBufferRef::U24(data) => conv(&mut pcm_data, data),
                AudioBufferRef::U32(data) => conv(&mut pcm_data, data),
                AudioBufferRef::S8(data) => conv(&mut pcm_data, data),
                AudioBufferRef::S16(data) => conv(&mut pcm_data, data),
                AudioBufferRef::S24(data) => conv(&mut pcm_data, data),
                AudioBufferRef::S32(data) => conv(&mut pcm_data, data),
                AudioBufferRef::F64(data) => conv(&mut pcm_data, data),
            }
        }
        Ok((pcm_data, sample_rate))
    }
}
