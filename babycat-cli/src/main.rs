use babycat::DecodeArgs;
use babycat::FloatWaveform;
use std::process;
use structopt::StructOpt;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, StructOpt)]
pub struct CommandArgs {
    #[structopt(long)]
    input_filename: String,

    #[structopt(long)]
    output_filename: String,

    #[structopt(long, default_value = "0")]
    pub start_time_milliseconds: u64,

    #[structopt(long, default_value = "0")]
    pub end_time_milliseconds: u64,

    #[structopt(long, default_value = "0")]
    pub frame_rate_hz: u32,

    #[structopt(long, default_value = "0")]
    pub num_channels: u32,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub convert_to_mono: bool,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub zero_pad_ending: bool,
}

fn main() {
    let command_args = CommandArgs::from_args();

    let decode_args = DecodeArgs {
        start_time_milliseconds: command_args.start_time_milliseconds,
        end_time_milliseconds: command_args.end_time_milliseconds,
        frame_rate_hz: command_args.frame_rate_hz,
        num_channels: command_args.num_channels,
        convert_to_mono: command_args.convert_to_mono,
        zero_pad_ending: command_args.zero_pad_ending,
    };

    match FloatWaveform::from_file(&command_args.input_filename, decode_args) {
        Ok(waveform) => {
            let write_result = waveform.to_wav_file(&command_args.output_filename);
            if let Err(err) = write_result {
                println!("Error: {}", err.to_string());
                process::exit(1);
            }
        }
        Err(err) => {
            println!("Error: {}", err.to_string());
            process::exit(1);
        }
    }
}
