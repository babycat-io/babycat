#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

impl Default for SRC_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[cfg(test)]
#[macro_use]
extern crate all_asserts;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sample_rate_conversion() {
        unsafe {
            let from = 44100;
            let to = 48000;
            let ratio = to as f64 / from as f64;

            // Setup sample data and storage.
            let freq = ::std::f32::consts::PI * 880f32 / from as f32;
            let mut input: Vec<f32> = (0..from).map(|i| (freq * i as f32).sin()).collect();
            let mut resampled = vec![0f32; to];
            let mut output = vec![0f32; from];

            // Convert from 44100Hz to 48000Hz.
            let mut src_pass_1 = SRC_DATA {
                data_in: input.as_mut_ptr(),
                data_out: resampled.as_mut_ptr(),
                input_frames: (input.len() as i32).into(),
                output_frames: (resampled.len() as i32).into(),
                src_ratio: ratio,
                ..Default::default()
            };
            let err = src_simple(&mut src_pass_1 as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);
            assert_eq!(err, 0);
            assert_eq!(src_pass_1.output_frames_gen, src_pass_1.output_frames);
            assert_eq!(src_pass_1.input_frames_used, src_pass_1.input_frames);

            // Convert from 48000Hz to 44100Hz.
            let mut src_pass_2 = SRC_DATA {
                data_in: resampled.as_mut_ptr(),
                data_out: output.as_mut_ptr(),
                input_frames: (resampled.len() as i32).into(),
                output_frames: (output.len() as i32).into(),
                src_ratio: 1f64/ratio,
                ..Default::default()
            };
            let err = src_simple(&mut src_pass_2 as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);
            assert_eq!(err, 0);
            assert_eq!(src_pass_2.output_frames_gen, src_pass_2.output_frames);
            assert_eq!(src_pass_2.input_frames_used, src_pass_2.input_frames);

            // Expect the difference between all input frames and all output frames to be less than
            // an epsilon.
            let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
            assert_lt!(error, 0.002);
        }
    }

    #[test]
    fn complex_sample_rate_conversion() {
        unsafe {
            let from = 44100;
            let to = 44100;
            let ratio = to as f64 / from as f64;

            // Setup sample data and storage.
            let freq = ::std::f32::consts::PI * 880f32 / from as f32;
            let mut input: Vec<f32> = (0..from).map(|i| (freq * i as f32).sin()).collect();
            let mut resampled = vec![0f32; to];
            let mut output = vec![0f32; from];

            // Create the samplerate converter.
            let mut error = 0i32;
            let converter: *mut SRC_STATE = src_new(SRC_SINC_BEST_QUALITY as i32, 1, &mut error as *mut i32);

            assert_eq!(error, 0);

            // Initial input configuration.
            let slices: usize = 10;

            let mut src = SRC_DATA {
                src_ratio: ratio,
                input_frames: (input.len() as i32 / slices as i32).into(),
                output_frames: (resampled.len() as i32 / slices as i32).into(),
                ..Default::default()
            };

            // Convert the input data in slices.
            let mut out_pos = 0;
            for i in 0..slices+1 {
                if i == (slices - 1) {
                    src.end_of_input = 1;
                }
                if i == slices {
                    src.input_frames = 0;
                } else {
                    src.data_in = input[i * from / slices..].as_mut_ptr();
                }

                src.data_out = resampled[out_pos..].as_mut_ptr();

                let err = src_process(converter, &mut src as *mut SRC_DATA);
                assert_eq!(err, 0);
                assert_eq!(src.input_frames_used, src.input_frames);
                out_pos += src.output_frames_gen as usize;
            }
            assert_eq!(out_pos, resampled.len());

            // Delete the converter.
            src_delete(converter);

            // Convert back from 48000Hz to 44100Hz.
            let mut src_reverse = SRC_DATA {
                data_in: resampled.as_mut_ptr(),
                data_out: output.as_mut_ptr(),
                input_frames: (resampled.len() as i32).into(),
                output_frames: (output.len() as i32).into(),
                src_ratio: 1f64/ratio,
                ..Default::default()
            };
            let err = src_simple(&mut src_reverse as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);

            assert_eq!(err, 0);
            assert_eq!(src_reverse.output_frames_gen, src_reverse.output_frames);
            assert_eq!(src_reverse.input_frames_used, src_reverse.input_frames);

            // Expect the difference between all input frames and all output frames to be less than
            // an epsilon.
            let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
            assert_lt!(error, 0.002);
        }
    }
}
