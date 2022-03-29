use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Audio analysis made easy.")]
pub struct CommandArgs {
    #[structopt(
        long,
        display_order = 1,
        help = "The logging level to print to standard error. Valid values are: OFF, ERROR, WARN, INFO, DEBUG, TRACE",
        default_value = "ERROR"
    )]
    pub log_level: log::LevelFilter,
    #[structopt(subcommand)]
    pub cmd: SubcommandArgs,
}

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
#[structopt()]
pub enum SubcommandArgs {
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
        start_time_milliseconds: usize,

        #[structopt(
            long,
            display_order = 5,
            default_value = "0",
            help = "Trim all audio after this millisecond timestamp"
        )]
        end_time_milliseconds: usize,

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
        num_channels: u16,

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
            display_order = 9,
            help = "Repeat the audio waveform to make it exactly `--end-time-milliseconds` in length in case the input audio file is shorter"
        )]
        repeat_pad_ending: bool,

        #[structopt(
            long,
            display_order = 10,
            default_value = "libsamplerate",
            help = "Select the backend to use for resampling. Valid values are: libsamplerate, babycat_lanczos, babycat_sinc"
        )]
        resample_mode: String,

        #[structopt(
            long,
            display_order = 11,
            default_value = "symphonia",
            help = "Select the backend to use for audio decoding. Valid values are: symphonia"
        )]
        decoding_backend: String,
    },
    Play {
        #[structopt(long, display_order = 1, help = "The audio file to play")]
        input_filename: String,
    },
}
