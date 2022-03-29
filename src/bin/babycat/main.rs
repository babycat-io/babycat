use structopt::StructOpt;

mod command_args;
mod commands;
mod common;

use crate::common::UnwrapOrExit;

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

fn main() {
    // Initialization.
    let args = command_args::CommandArgs::from_args();
    init_logging(args.log_level);
    // Select which subcommand to run.
    match args.cmd {
        command_args::SubcommandArgs::Convert {
            input_filename,
            output_filename,
            output_format,
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            repeat_pad_ending,
            resample_mode,
            decoding_backend,
        } => commands::convert::convert(
            &input_filename,
            &output_filename,
            &output_format,
            start_time_milliseconds,
            end_time_milliseconds,
            frame_rate_hz,
            num_channels,
            convert_to_mono,
            zero_pad_ending,
            repeat_pad_ending,
            &resample_mode,
            &decoding_backend,
        ),
        command_args::SubcommandArgs::Play { input_filename } => {
            commands::play::play(input_filename).unwrap_or_exit()
        }
    }
}
