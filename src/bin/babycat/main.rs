use log::error;
use log::info;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use babycat::FloatWaveform;
use babycat::RESAMPLE_MODE_BABYCAT;
use babycat::RESAMPLE_MODE_LIBSAMPLERATE;
use babycat::{DecodeArgs, Waveform};

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Audio analysis made easy.")]
struct CommandArgs {
    #[structopt(
        long,
        display_order = 1,
        help = "The logging level to print to standard error. Valid values are: OFF, ERROR, WARN, INFO, DEBUG, TRACE",
        default_value = "ERROR"
    )]
    log_level: log::LevelFilter,
    #[structopt(subcommand)]
    cmd: SubcommandArgs,
}

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
#[structopt()]
enum SubcommandArgs {
    Convert {
        #[structopt(
            long,
            display_order = 1,
            help = "The input audio file that you want to convert"
        )]
        input_filename: String,

        #[structopt(long, display_order = 2, help = "The name of the output file to write")]
        output_filename: String,

        #[structopt(
            long,
            display_order = 3,
            help = "The format to encode the output audio into. Valid values are: wav"
        )]
        output_format: String,

        #[structopt(
            long,
            display_order = 4,
            default_value = "0",
            help = "Trim all audio before this millisecond timestamp"
        )]
        start_time_milliseconds: u64,

        #[structopt(
            long,
            display_order = 5,
            default_value = "0",
            help = "Trim all audio after this millisecond timestamp"
        )]
        end_time_milliseconds: u64,

        #[structopt(
            long,
            display_order = 6,
            default_value = "0",
            help = "Resample to this sample rate in hertz"
        )]
        frame_rate_hz: u32,

        #[structopt(
            long,
            display_order = 7,
            default_value = "0",
            help = "Include only the first n channels. By default, we inclue all channels in the output"
        )]
        num_channels: u32,

        #[structopt(
            long,
            display_order = 8,
            help = "Average all input audio channels into a single monophonic output channel"
        )]
        convert_to_mono: bool,

        #[structopt(
            long,
            display_order = 9,
            help = "Add silence to the end of the output audio file to make it exactly `--end-time-milliseconds` in length in case the input audio file is shorter"
        )]
        zero_pad_ending: bool,

        #[structopt(
            long,
            display_order = 10,
            default_value = "libsamplerate",
            help = "Select the backend to use for resampling. Valid values are 'libsamplerate' and 'babycat'"
        )]
        resample_mode: String,
    },
}

fn init_logging(level: log::LevelFilter) {
    let mut builder = env_logger::Builder::new();
    // We silence log messages from all non-babycat crates
    // unless the logging level is DEBUG or TRACE.
    if level == log::LevelFilter::Debug || level == log::LevelFilter::Trace {
        builder.filter_level(level);
    } else {
        builder.filter_module("babycat", level);
    }
    builder.init();
}

fn exit_with_msg(msg: &str) -> ! {
    error!("{}", msg);
    std::process::exit(1);
}

pub trait UnwrapOrExit<T, E> {
    fn unwrap_or_exit(self) -> T;
}

impl<T, E: std::error::Error> UnwrapOrExit<T, E> for std::result::Result<T, E> {
    fn unwrap_or_exit(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => exit_with_msg(&e.to_string()),
        }
    }
}

fn main() {
    // Initialization.
    let args = CommandArgs::from_args();
    init_logging(args.log_level);
    // Select which subcommand to run.
    match args.cmd {
        SubcommandArgs::Convert {
            input_filename,
            output_filename,
            output_format,
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            resample_mode,
        } => {
            //
            // Input validation.
            if output_format != "wav" {
                exit_with_msg(&format!(
                    "Unsupported output file format: {}",
                    output_format
                ));
            }
            let resample_mode_int = {
                if resample_mode == "libsamplerate" {
                    RESAMPLE_MODE_LIBSAMPLERATE
                } else if resample_mode == "babycat" {
                    RESAMPLE_MODE_BABYCAT
                } else {
                    panic!("NO");
                }
            };
            //
            // Set up decoding.
            let decode_args = DecodeArgs {
                start_time_milliseconds,
                end_time_milliseconds,
                frame_rate_hz,
                num_channels,
                convert_to_mono,
                zero_pad_ending,
                resample_mode: resample_mode_int,
            };
            //
            // Decode from filesystem.
            let decoding_start_time = std::time::Instant::now();
            let waveform = FloatWaveform::from_file(&input_filename, decode_args).unwrap_or_exit();
            let decoding_elapsed = std::time::Instant::now() - decoding_start_time;
            info!(
                "Decoded {} frames of {} channels at {} hz in {} seconds from {}",
                waveform.num_frames(),
                waveform.num_channels(),
                waveform.frame_rate_hz(),
                decoding_elapsed.as_secs_f64(),
                input_filename,
            );
            //
            // Waveform is now in memory. Time to encode.
            let encoding_start_time = std::time::Instant::now();
            waveform.to_wav_file(&output_filename).unwrap_or_exit();
            let encoding_elapsed = std::time::Instant::now() - encoding_start_time;
            info!(
                "Encoded as {} and saved in {} seconds to {}",
                output_format,
                encoding_elapsed.as_secs_f64(),
                output_filename,
            );
        }
    }
}
