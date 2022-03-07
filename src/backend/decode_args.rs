use crate::backend::errors::Error;
use crate::backend::waveform_args::WaveformArgs;
use crate::backend::waveform_args::DEFAULT_END_TIME_MILLISECONDS;
use crate::backend::waveform_args::DEFAULT_NUM_CHANNELS;
use crate::backend::waveform_args::DEFAULT_START_TIME_MILLISECONDS;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodeArgs {
    pub start_frame_idx: usize,
    pub end_frame_idx: usize,
    pub num_channels: u16,
    pub convert_to_mono: bool,
}

impl DecodeArgs {
    pub fn new(
        args: WaveformArgs,
        original_frame_rate_hz: u32,
        original_num_channels: u16,
    ) -> Result<Self, Error> {
        // If the user has provided an end timestamp that is BEFORE
        // our start timestamp, then we raise an error.
        if args.start_time_milliseconds != DEFAULT_START_TIME_MILLISECONDS
            && args.end_time_milliseconds != DEFAULT_END_TIME_MILLISECONDS
            && args.start_time_milliseconds >= args.end_time_milliseconds
        {
            return Err(Error::WrongTimeOffset(
                args.start_time_milliseconds,
                args.end_time_milliseconds,
            ));
        }

        // If the user has not specified how long the output audio should be,
        // then we would not know how to zero-pad after it.
        if args.zero_pad_ending && args.end_time_milliseconds == DEFAULT_END_TIME_MILLISECONDS {
            return Err(Error::CannotZeroPadWithoutSpecifiedLength);
        }

        // We do not allow the user to specify that they want to extract
        // one channels AND to convert the waveform to mono.
        // Converting the waveform to mono only makes sense when
        // we are working with more than one channel.
        if args.num_channels == 1 && args.convert_to_mono {
            return Err(Error::WrongNumChannelsAndMono);
        }

        // This is the first n channels that we want to read from.
        // If the user wants to convert the output to mono, we do that after
        // reading from the first n channels.
        // If args.num_channels was unspecified, then we read from
        // all of the channels.
        let num_channels = {
            if args.num_channels == DEFAULT_NUM_CHANNELS {
                original_num_channels
            } else if args.num_channels < 1 {
                return Err(Error::WrongNumChannels(
                    args.num_channels,
                    original_num_channels,
                ));
            } else if original_num_channels >= args.num_channels {
                args.num_channels
            } else {
                return Err(Error::WrongNumChannels(
                    args.num_channels,
                    original_num_channels,
                ));
            }
        };

        let start_frame_idx: usize =
            args.start_time_milliseconds * (original_frame_rate_hz as usize) / 1000;
        let end_frame_idx: usize =
            args.end_time_milliseconds * (original_frame_rate_hz as usize) / 1000;

        Ok(Self {
            start_frame_idx,
            end_frame_idx,
            num_channels,
            convert_to_mono: args.convert_to_mono,
        })
    }
}
