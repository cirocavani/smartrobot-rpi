// gst-launch-1.0 alsasrc device=sysdefault:CARD=Device ! queue ! audioconvert ! wavenc ! filesink location=out.wav
// https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/tutorials/src/bin/basic-tutorial-3.rs
// https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/audio_multichannel_interleave.rs
// https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/blob/main/examples/src/bin/appsink.rs

use anyhow::Result;
use clap::Parser;

use gst::prelude::*;
use gstreamer as gst;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable tracing (generates a trace-timestamp.json file).
    #[arg(long)]
    tracing: bool,

    /// ALSA device, as defined in an asound configuration file.
    #[arg(long)]
    device: String,

    /// The output file using the wav format.
    #[arg(long, default_value = "out.wav")]
    out_file: String,
}

fn create_pipeline(device: &str, out_file: &str) -> Result<gst::Pipeline> {
    gst::init()?;

    let source = gst::ElementFactory::make("alsasrc")
        .property("device", device)
        .build()?;
    let queue = gst::ElementFactory::make("queue").build()?;
    let convert = gst::ElementFactory::make("audioconvert").build()?;
    let enc = gst::ElementFactory::make("wavenc").build()?;
    let sink = gst::ElementFactory::make("filesink")
        .property("location", out_file)
        .build()?;

    let pipeline = gst::Pipeline::new();
    pipeline.add_many([&source, &queue, &convert, &enc, &sink])?;
    gst::Element::link_many([&source, &queue, &convert, &enc, &sink])?;

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<()> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    let args = Args::parse();
    let _guard = if args.tracing {
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };

    create_pipeline(args.device.as_str(), args.out_file.as_str()).and_then(main_loop)?;

    Ok(())
}
