#include "../babycat.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>

#define NUM_RESAMPLING_MODES 3

const uint32_t RESAMPLING_MODES[NUM_RESAMPLING_MODES] = {
    babycat_RESAMPLE_MODE_LIBSAMPLERATE, babycat_RESAMPLE_MODE_BABYCAT_LANCZOS,
    babycat_RESAMPLE_MODE_BABYCAT_SINC};

// We call this macro at the end of every unit test
// so we know that the test succeeded.
#define SUCCESS() fprintf(stderr, "Success: %s\n", __FUNCTION__)

#define DECODE_COF(decode_args)                                                \
  babycat_float_waveform_from_file(                                            \
      "./audio-for-tests/circus-of-freaks/track.mp3", decode_args)

static void test_float_waveform_from_file__test_circus_of_freaks_default_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_wrong_time_offset_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 1000;
  decode_args.end_time_milliseconds = 999;

  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_TIME_OFFSET);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_wrong_time_offset_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 1000;
  decode_args.end_time_milliseconds = 1000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_TIME_OFFSET);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 5;
  decode_args.end_time_milliseconds = 0;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_CANNOT_ZERO_PAD);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_get_channels_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.num_channels = 1;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 1);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_get_channels_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.num_channels = 2;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_get_channels_too_many_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.num_channels = 3;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_NUM_CHANNELS);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.num_channels = 2;
  decode_args.convert_to_mono = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 1);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.convert_to_mono = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 1);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_invalid_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.num_channels = 1;
  decode_args.convert_to_mono = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num ==
         babycat_ERROR_WRONG_NUM_CHANNELS_AND_MONO);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 1;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 44);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 10;
  decode_args.end_time_milliseconds = 11;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 44);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_3() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 30000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_4() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 15000;
  decode_args.end_time_milliseconds = 45000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_5() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 30000;
  decode_args.end_time_milliseconds = 60000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1169928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 1;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 44);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 10;
  decode_args.end_time_milliseconds = 11;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 44);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 30000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 15000;
  decode_args.end_time_milliseconds = 45000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 30000;
  decode_args.end_time_milliseconds = 60000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 60000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 90000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 30000;
  decode_args.end_time_milliseconds = 90000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_end_milliseconds_zero_pad_ending_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.end_time_milliseconds = 90000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 1;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 20;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_3() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 172;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_resample_no_change() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 44100;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 22050;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1246464);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 22050);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 11025;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 623232);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 11025);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_3() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 88200;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 4985856);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 88200);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_4() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 4410;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 249293);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 4410);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_5() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 44099;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492872);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44099);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_6() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 48000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2713392);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 48000);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_7() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 60000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 3391739);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 60000);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_8() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 88200;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 4985856);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 88200);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_9() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 96000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 5426783);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 96000);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_10() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 200;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 11306);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 200);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_11() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 2000;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 113058);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 2000);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_from_file__test_circus_of_freaks_resample_12() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.frame_rate_hz = 173;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 9780);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 173);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 60000;
  decode_args.frame_rate_hz = 48000;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2880000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 48000);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 60000;
  decode_args.frame_rate_hz = 44099;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2645940);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44099);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  decode_args.start_time_milliseconds = 0;
  decode_args.end_time_milliseconds = 60000;
  decode_args.frame_rate_hz = 22050;
  decode_args.zero_pad_ending = true;
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 22050);
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void
test_float_waveform_resample_method__test_circus_of_freaks_no_change_1() {
  uint32_t new_frame_rate_hz = 44100;
  uint32_t expected_num_frames = 2492928;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_resample_method__test_circus_of_freaks_44099() {
  uint32_t new_frame_rate_hz = 44099;
  uint32_t expected_num_frames = 2492872;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_resample_method__test_circus_of_freaks_44101() {
  uint32_t new_frame_rate_hz = 44101;
  uint32_t expected_num_frames = 2492985;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_resample_method__test_circus_of_freaks_22050() {
  uint32_t new_frame_rate_hz = 22050;
  uint32_t expected_num_frames = 1246464;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_resample_method__test_circus_of_freaks_11025() {
  uint32_t new_frame_rate_hz = 11025;
  uint32_t expected_num_frames = 623232;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

static void test_float_waveform_resample_method__test_circus_of_freaks_88200() {
  uint32_t new_frame_rate_hz = 88200;
  uint32_t expected_num_frames = 4985856;
  //
  // Decode the waveform the first time.
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result = DECODE_COF(decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_FloatWaveformResult resampled_result =
        babycat_float_waveform_resample_by_mode(waveform, new_frame_rate_hz,
                                                resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_FloatWaveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_float_waveform_get_num_channels(resampled) == 2);
    assert(babycat_float_waveform_get_num_frames(resampled) ==
           expected_num_frames);
    assert(babycat_float_waveform_get_frame_rate_hz(resampled) ==
           new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_float_waveform_free(resampled);
  }

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

int main() {
  printf("\n\n == Begin testing ==\n\n");
  test_float_waveform_from_file__test_circus_of_freaks_default_1();
  test_float_waveform_from_file__test_circus_of_freaks_wrong_time_offset_1();
  test_float_waveform_from_file__test_circus_of_freaks_wrong_time_offset_2();
  test_float_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1();
  test_float_waveform_from_file__test_circus_of_freaks_get_channels_1();
  test_float_waveform_from_file__test_circus_of_freaks_get_channels_2();
  test_float_waveform_from_file__test_circus_of_freaks_get_channels_too_many_1();
  test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_1();
  test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_2();
  test_float_waveform_from_file__test_circus_of_freaks_convert_to_mono_invalid_1();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_1();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_2();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_3();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_4();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_5();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8();
  test_float_waveform_from_file__test_circus_of_freaks_end_milliseconds_zero_pad_ending_1();
  test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_1();
  test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_2();
  test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_2();
  test_float_waveform_from_file__test_circus_of_freaks_invalid_resample_3();
  test_float_waveform_from_file__test_circus_of_freaks_resample_no_change();
  test_float_waveform_from_file__test_circus_of_freaks_resample_1();
  test_float_waveform_from_file__test_circus_of_freaks_resample_2();
  test_float_waveform_from_file__test_circus_of_freaks_resample_3();
  test_float_waveform_from_file__test_circus_of_freaks_resample_4();
  test_float_waveform_from_file__test_circus_of_freaks_resample_5();
  test_float_waveform_from_file__test_circus_of_freaks_resample_6();
  test_float_waveform_from_file__test_circus_of_freaks_resample_7();
  test_float_waveform_from_file__test_circus_of_freaks_resample_8();
  test_float_waveform_from_file__test_circus_of_freaks_resample_9();
  test_float_waveform_from_file__test_circus_of_freaks_resample_10();
  test_float_waveform_from_file__test_circus_of_freaks_resample_11();
  test_float_waveform_from_file__test_circus_of_freaks_resample_12();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2();
  test_float_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3();
  test_float_waveform_resample_method__test_circus_of_freaks_no_change_1();
  test_float_waveform_resample_method__test_circus_of_freaks_44099();
  test_float_waveform_resample_method__test_circus_of_freaks_44101();
  test_float_waveform_resample_method__test_circus_of_freaks_22050();
  test_float_waveform_resample_method__test_circus_of_freaks_11025();
  test_float_waveform_resample_method__test_circus_of_freaks_88200();
  printf("\n\n == ALL TESTS HAVE PASSED! == \n\n");
  return 0;
}
