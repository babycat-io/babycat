mod fixtures;

mod test_waveform_from_many_files {
    use crate::fixtures::*;
    use babycat::{BatchArgs, Waveform, WaveformArgs};

    #[test]
    fn test_all_same_file_1() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let waveform_args = Default::default();
        let batch_args = Default::default();
        let batch = Waveform::from_many_files(filenames, waveform_args, batch_args);
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(COF_NUM_FRAMES, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (COF_NUM_FRAMES * COF_NUM_CHANNELS as u64) as usize,
                waveform.to_interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_all_same_file_2() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let waveform_args = WaveformArgs {
            end_time_milliseconds: 15000,
            ..Default::default()
        };
        let batch_args = Default::default();
        let batch = Waveform::from_many_files(filenames, waveform_args, batch_args);
        let num_frames = 661500;
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(num_frames, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (num_frames * COF_NUM_CHANNELS as u64) as usize,
                waveform.to_interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_all_same_file_single_threaded_1() {
        let filenames = &[COF_FILENAME, COF_FILENAME, COF_FILENAME];
        let waveform_args = Default::default();
        let batch_args = BatchArgs {
            num_workers: 1,
            ..Default::default()
        };
        let batch = Waveform::from_many_files(filenames, waveform_args, batch_args);
        for named_result in batch {
            let waveform = named_result.result.unwrap();
            assert_eq!(COF_NUM_CHANNELS, waveform.num_channels());
            assert_eq!(COF_NUM_FRAMES, waveform.num_frames());
            assert_eq!(COF_FRAME_RATE_HZ, waveform.frame_rate_hz());
            assert_eq!(
                (COF_NUM_FRAMES * COF_NUM_CHANNELS as u64) as usize,
                waveform.to_interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_different_filenames_1() {
        let waveform_args = Default::default();
        let batch_args = Default::default();
        let batch = Waveform::from_many_files(ALL_FILENAMES, waveform_args, batch_args);
        for (i, named_result) in batch.into_iter().enumerate() {
            let waveform = named_result.result.unwrap();
            assert_eq!(ALL_NUM_CHANNELS[i], waveform.num_channels());
            assert_eq!(ALL_NUM_FRAMES[i], waveform.num_frames());
            assert_eq!(ALL_FRAME_RATE_HZ[i], waveform.frame_rate_hz());
            assert_eq!(
                (ALL_NUM_FRAMES[i] * ALL_NUM_CHANNELS[i] as u64) as usize,
                waveform.to_interleaved_samples().len()
            );
        }
    }

    #[test]
    fn test_file_not_found_error_1() {
        let filenames = &[COF_FILENAME, "asdfasdf"];
        let waveform_args = Default::default();
        let batch_args = Default::default();
        let batch = Waveform::from_many_files(filenames, waveform_args, batch_args);
        assert_eq!(batch.len(), 2);
        let first_result = batch[0].result.as_ref().unwrap();
        assert_eq!(COF_NUM_CHANNELS, first_result.num_channels());
        assert_eq!(COF_NUM_FRAMES, first_result.num_frames());
        assert_eq!(COF_FRAME_RATE_HZ, first_result.frame_rate_hz());
        let second_result = batch[1].result.as_ref().unwrap_err();
        assert_eq!(second_result.error_type(), "FileNotFound(asdfasdf)");
    }
}
