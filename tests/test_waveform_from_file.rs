mod fixtures;

mod test_waveform_from_file {
    use babycat::Error;
    use babycat::Signal;
    use babycat::Waveform;
    use babycat::WaveformArgs;

    use crate::fixtures::*;

    fn decode_cof_mp3(waveform_args: WaveformArgs) -> Result<Waveform, Error> {
        Waveform::from_file(COF_FILENAME, waveform_args)
    }

    fn decode_lct_mp3(waveform_args: WaveformArgs) -> Result<Waveform, Error> {
        Waveform::from_file(LCT_FILENAME, waveform_args)
    }

    fn assert_error(result: Result<Waveform, Error>, error_type: &str) {
        assert_eq!(error_type, result.unwrap_err().error_type());
    }

    #[track_caller]
    #[inline]
    fn assert_waveform(
        waveform: &Waveform,
        num_channels: u16,
        num_frames: usize,
        frame_rate_hz: u32,
    ) {
        assert_eq!(num_channels, waveform.num_channels());
        assert_eq!(num_frames, waveform.num_frames());
        assert_eq!(frame_rate_hz, waveform.frame_rate_hz());
        assert_eq!(
            (num_frames * num_channels as usize) as usize,
            waveform.to_interleaved_samples().len()
        );
    }

    #[track_caller]
    #[inline]
    fn assert_zeros_after_frame(waveform: &Waveform, start_frame_idx: usize) {
        let num_frames = waveform.num_frames();
        let num_channels = waveform.num_channels() as usize;
        for frame_idx in start_frame_idx..num_frames {
            for channel_idx in 0..num_channels {
                assert_eq!(
                    waveform.get_sample(frame_idx, channel_idx as u16).unwrap(),
                    0.0_f32
                );
            }
        }
    }

    #[test]
    fn test_circus_of_freaks_default_1() {
        let waveform_args = WaveformArgs {
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(
            &waveform,
            COF_NUM_CHANNELS,
            COF_NUM_FRAMES,
            COF_FRAME_RATE_HZ,
        );
    }

    #[test]
    fn test_circus_of_freaks_wrong_time_offset_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 1000,
            end_time_milliseconds: 999,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongTimeOffset(1000,999)");
    }

    #[test]
    fn test_circus_of_freaks_wrong_time_offset_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 1000,
            end_time_milliseconds: 1000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongTimeOffset(1000,1000)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_repeat_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1000,
            zero_pad_ending: true,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "CannotSetZeroPadEndingAndRepeatPadEnding");
    }

    #[test]
    fn test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 5,
            end_time_milliseconds: 0,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "CannotZeroPadWithoutSpecifiedLength");
    }

    #[test]
    fn test_circus_of_freaks_invalid_end_time_milliseconds_repeat_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 5,
            end_time_milliseconds: 0,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "CannotRepeatPadWithoutSpecifiedLength");
    }

    #[test]
    fn test_circus_of_freaks_get_channels_1() {
        let waveform_args = WaveformArgs {
            num_channels: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_get_channels_2() {
        let waveform_args = WaveformArgs {
            num_channels: 2,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, 2, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_get_channels_too_many_1() {
        let waveform_args = WaveformArgs {
            num_channels: 3,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongNumChannels(3,2)");
    }

    #[test]
    fn test_circus_of_freaks_convert_to_mono_1() {
        let waveform_args = WaveformArgs {
            num_channels: 2,
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }
    #[test]
    fn test_circus_of_freaks_convert_to_mono_2() {
        let waveform_args = WaveformArgs {
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_left_channel_tone_convert_to_mono_1() {
        // In this test, we do mono and stereo decoding of an audio file
        // that only has audio in one of its two channels.
        // First, let's do the mono decoding.
        let mono_waveform_args = WaveformArgs {
            convert_to_mono: true,
            ..Default::default()
        };
        let mono_result = decode_lct_mp3(mono_waveform_args);
        let mono_waveform = mono_result.unwrap();
        assert_eq!(1, mono_waveform.num_channels());
        assert_eq!(LCT_NUM_FRAMES, mono_waveform.num_frames());
        assert_eq!(LCT_FRAME_RATE_HZ, mono_waveform.frame_rate_hz());
        let mono_sum_waveform: f32 = mono_waveform.to_interleaved_samples().iter().sum();
        // Now, let's do the stereo decoding.
        let stereo_waveform_args = WaveformArgs {
            ..Default::default()
        };
        let stereo_result = decode_lct_mp3(stereo_waveform_args);
        let stereo_waveform = stereo_result.unwrap();
        assert_eq!(LCT_NUM_CHANNELS, stereo_waveform.num_channels());
        assert_eq!(LCT_NUM_FRAMES, stereo_waveform.num_frames());
        assert_eq!(LCT_FRAME_RATE_HZ, stereo_waveform.frame_rate_hz());
        let stereo_sum_waveform: f32 = stereo_waveform.to_interleaved_samples().iter().sum();
        // Check that the mono waveform is quieter because we made it
        // by averaging in the other silent channel.
        assert!(float_cmp::approx_eq!(
            f32,
            mono_sum_waveform * 2.0_f32,
            stereo_sum_waveform,
            ulps = 3
        ));
    }

    #[test]
    fn test_circus_of_freaks_convert_to_mono_invalid_1() {
        let waveform_args = WaveformArgs {
            num_channels: 1,
            convert_to_mono: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongNumChannelsAndMono");
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 10,
            end_time_milliseconds: 11,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_3() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 30000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_4() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 15000,
            end_time_milliseconds: 45000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_5() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 60000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1168247, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_6() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 45000,
            end_time_milliseconds: 60000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 506747, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 10,
            end_time_milliseconds: 11,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 30000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 15000,
            end_time_milliseconds: 45000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 60000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_end_milliseconds_zero_pad_ending_1() {
        let waveform_args = WaveformArgs {
            end_time_milliseconds: 90000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
        assert_zeros_after_frame(&waveform, COF_NUM_FRAMES);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 1,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 10,
            end_time_milliseconds: 11,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_3() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 30000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_4() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 15000,
            end_time_milliseconds: 45000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_5() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 60000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_6() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_7() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 90000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_8() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 30000,
            end_time_milliseconds: 90000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_end_milliseconds_repeat_pad_ending_1() {
        let waveform_args = WaveformArgs {
            end_time_milliseconds: 90000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
    }

    #[test]
    fn test_circus_of_freaks_end_milliseconds_repeat_pad_ending_2() {
        let end_time_milliseconds: usize = 190000;
        let waveform_args = WaveformArgs {
            end_time_milliseconds: end_time_milliseconds,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        let final_frame: usize = end_time_milliseconds * COF_FRAME_RATE_HZ as usize / 1000;
        assert_waveform(&waveform, COF_NUM_CHANNELS, final_frame, COF_FRAME_RATE_HZ);
        for frame_idx in COF_NUM_FRAMES..final_frame {
            for channel_idx in 0..COF_NUM_CHANNELS {
                let original_frame_idx = frame_idx % COF_NUM_FRAMES;
                assert_eq!(
                    waveform.get_sample(original_frame_idx, channel_idx),
                    waveform.get_sample(frame_idx, channel_idx)
                );
            }
        }
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_1() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 1,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongFrameRateRatio(44100,1)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_2() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 20,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongFrameRateRatio(44100,20)");
    }

    #[test]
    fn test_circus_of_freaks_invalid_resample_3() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 172,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        assert_error(result, "WrongFrameRateRatio(44100,172)");
    }

    #[test]
    fn test_circus_of_freaks_resample_no_change_1() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: COF_FRAME_RATE_HZ,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(
            &waveform,
            COF_NUM_CHANNELS,
            COF_NUM_FRAMES,
            COF_FRAME_RATE_HZ,
        );
    }

    #[test]
    fn test_circus_of_freaks_resample_1() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 22050,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1245624, 22050);
    }

    #[test]
    fn test_circus_of_freaks_resample_2() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 11025,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 622812, 11025);
    }

    #[test]
    fn test_circus_of_freaks_resample_3() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 88200,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 4982494, 88200);
    }

    #[test]
    fn test_circus_of_freaks_resample_4() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 4410,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 249125, 4410);
    }

    #[test]
    fn test_circus_of_freaks_resample_5() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 44099,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2491191, 44099);
    }

    #[test]
    fn test_circus_of_freaks_resample_6() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 48000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2711562, 48000);
    }

    #[test]
    fn test_circus_of_freaks_resample_7() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 60000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 3389452, 60000);
    }

    #[test]
    fn test_circus_of_freaks_resample_8() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 88200,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 4982494, 88200);
    }

    #[test]
    fn test_circus_of_freaks_resample_9() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 96000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 5423123, 96000);
    }

    #[test]
    fn test_circus_of_freaks_resample_10() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 200,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 11299, 200);
    }

    #[test]
    fn test_circus_of_freaks_resample_11() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 2000,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 112982, 2000);
    }

    #[test]
    fn test_circus_of_freaks_resample_12() {
        let waveform_args = WaveformArgs {
            frame_rate_hz: 173,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 9773, 173);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 48000,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2880000, 48000);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 44099,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2645940, 44099);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 22050,
            zero_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, 22050);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_1() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 48000,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2880000, 48000);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_2() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 44099,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 2645940, 44099);
    }

    #[test]
    fn test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_3() {
        let waveform_args = WaveformArgs {
            start_time_milliseconds: 0,
            end_time_milliseconds: 60000,
            frame_rate_hz: 22050,
            repeat_pad_ending: true,
            ..Default::default()
        };
        let result = decode_cof_mp3(waveform_args);
        let waveform = result.unwrap();
        assert_waveform(&waveform, COF_NUM_CHANNELS, 1323000, 22050);
    }
}
