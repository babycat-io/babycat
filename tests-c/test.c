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

#define DECODE_COF(waveform_args)                                              \
  babycat_waveform_from_file("./audio-for-tests/circus-of-freaks/track.flac",  \
                             waveform_args)

static void test_waveform_from_file__test_circus_of_freaks_default_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_wrong_time_offset_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 1000;
  waveform_args.end_time_milliseconds = 999;

  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_TIME_OFFSET);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_wrong_time_offset_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 1000;
  waveform_args.end_time_milliseconds = 1000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_TIME_OFFSET);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_repeat_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 1000;
  waveform_args.zero_pad_ending = true;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num ==
         babycat_ERROR_CANNOT_ZERO_PAD_AND_REPEAT_PAD);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 5;
  waveform_args.end_time_milliseconds = 0;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_CANNOT_ZERO_PAD);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_repeat_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 5;
  waveform_args.end_time_milliseconds = 0;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_CANNOT_REPEAT_PAD);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_get_channels_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.num_channels = 1;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 1);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_get_channels_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.num_channels = 2;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_get_channels_too_many_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.num_channels = 3;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_NUM_CHANNELS);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_convert_to_mono_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.num_channels = 2;
  waveform_args.convert_to_mono = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 1);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_convert_to_mono_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.convert_to_mono = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 1);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_convert_to_mono_invalid_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.num_channels = 1;
  waveform_args.convert_to_mono = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num ==
         babycat_ERROR_WRONG_NUM_CHANNELS_AND_MONO);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 1;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 10;
  waveform_args.end_time_milliseconds = 11;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 30000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_4() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 15000;
  waveform_args.end_time_milliseconds = 45000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_5() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 30000;
  waveform_args.end_time_milliseconds = 60000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1168247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 1;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 10;
  waveform_args.end_time_milliseconds = 11;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 30000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 15000;
  waveform_args.end_time_milliseconds = 45000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 30000;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 30000;
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_end_milliseconds_zero_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 1;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 10;
  waveform_args.end_time_milliseconds = 11;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 44);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 30000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_4() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 15000;
  waveform_args.end_time_milliseconds = 45000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_5() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 30000;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_6() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_7() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_8() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 30000;
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2646000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_end_milliseconds_repeat_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.end_time_milliseconds = 90000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 3969000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_resample_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 1;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_resample_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 20;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_invalid_resample_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 172;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_ERROR_WRONG_FRAME_RATE_RATIO);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_resample_no_change() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 44100;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2491247);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44100);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 22050;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1245624);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 22050);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 11025;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 622812);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 11025);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 88200;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 4982494);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 88200);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_4() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 4410;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 249125);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 4410);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_5() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 44099;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2491191);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44099);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_6() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 48000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2711562);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 48000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_7() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 60000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 3389452);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 60000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_8() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 88200;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 4982494);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 88200);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_9() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 96000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 5423123);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 96000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_10() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 200;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 11299);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 200);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_11() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 2000;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 112982);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 2000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_from_file__test_circus_of_freaks_resample_12() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.frame_rate_hz = 173;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 9773);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 173);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 48000;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2880000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 48000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 44099;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2645940);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44099);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 22050;
  waveform_args.zero_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 22050);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_1() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 48000;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2880000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 48000);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_2() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 44099;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 2645940);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 44099);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void
test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_3() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  waveform_args.start_time_milliseconds = 0;
  waveform_args.end_time_milliseconds = 60000;
  waveform_args.frame_rate_hz = 22050;
  waveform_args.repeat_pad_ending = true;
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  assert(babycat_waveform_get_num_channels(waveform) == 2);
  assert(babycat_waveform_get_num_frames(waveform) == 1323000);
  assert(babycat_waveform_get_frame_rate_hz(waveform) == 22050);
  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_no_change_1() {
  uint32_t new_frame_rate_hz = 44100;
  uint32_t expected_num_frames = 2491247;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_44099() {
  uint32_t new_frame_rate_hz = 44099;
  uint32_t expected_num_frames = 2491191;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_44101() {
  uint32_t new_frame_rate_hz = 44101;
  uint32_t expected_num_frames = 2491304;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_22050() {
  uint32_t new_frame_rate_hz = 22050;
  uint32_t expected_num_frames = 1245624;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_11025() {
  uint32_t new_frame_rate_hz = 11025;
  uint32_t expected_num_frames = 622812;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

static void test_waveform_resample_method__test_circus_of_freaks_88200() {
  uint32_t new_frame_rate_hz = 88200;
  uint32_t expected_num_frames = 4982494;
  //
  // Decode the waveform the first time.
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = DECODE_COF(waveform_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_Waveform *waveform = waveform_result.result;
  //
  // Run through the resampling modes.
  for (size_t i = 0; i < NUM_RESAMPLING_MODES; ++i) {
    uint32_t resample_mode = RESAMPLING_MODES[i];
    //
    // Resample the waveform.
    babycat_WaveformResult resampled_result = babycat_waveform_resample_by_mode(
        waveform, new_frame_rate_hz, resample_mode);
    assert(resampled_result.error_num == babycat_NO_ERROR);
    babycat_Waveform *resampled = resampled_result.result;
    //
    // Make assertions.
    assert(babycat_waveform_get_num_channels(resampled) == 2);
    assert(babycat_waveform_get_num_frames(resampled) == expected_num_frames);
    assert(babycat_waveform_get_frame_rate_hz(resampled) == new_frame_rate_hz);
    //
    // Cleanup time.
    babycat_waveform_free(resampled);
  }

  babycat_waveform_free(waveform);
  SUCCESS();
}

int main() {
  printf("\n\n == Begin testing ==\n\n");
  test_waveform_from_file__test_circus_of_freaks_default_1();
  test_waveform_from_file__test_circus_of_freaks_wrong_time_offset_1();
  test_waveform_from_file__test_circus_of_freaks_wrong_time_offset_2();
  test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_repeat_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_invalid_end_time_milliseconds_repeat_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_get_channels_1();
  test_waveform_from_file__test_circus_of_freaks_get_channels_2();
  test_waveform_from_file__test_circus_of_freaks_get_channels_too_many_1();
  test_waveform_from_file__test_circus_of_freaks_convert_to_mono_1();
  test_waveform_from_file__test_circus_of_freaks_convert_to_mono_2();
  test_waveform_from_file__test_circus_of_freaks_convert_to_mono_invalid_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_2();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_3();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_4();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_5();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8();
  test_waveform_from_file__test_circus_of_freaks_end_milliseconds_zero_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_2();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_3();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_4();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_5();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_6();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_7();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_repeat_pad_ending_8();
  test_waveform_from_file__test_circus_of_freaks_end_milliseconds_repeat_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_invalid_resample_1();
  test_waveform_from_file__test_circus_of_freaks_invalid_resample_2();
  test_waveform_from_file__test_circus_of_freaks_invalid_resample_2();
  test_waveform_from_file__test_circus_of_freaks_invalid_resample_3();
  test_waveform_from_file__test_circus_of_freaks_resample_no_change();
  test_waveform_from_file__test_circus_of_freaks_resample_1();
  test_waveform_from_file__test_circus_of_freaks_resample_2();
  test_waveform_from_file__test_circus_of_freaks_resample_3();
  test_waveform_from_file__test_circus_of_freaks_resample_4();
  test_waveform_from_file__test_circus_of_freaks_resample_5();
  test_waveform_from_file__test_circus_of_freaks_resample_6();
  test_waveform_from_file__test_circus_of_freaks_resample_7();
  test_waveform_from_file__test_circus_of_freaks_resample_8();
  test_waveform_from_file__test_circus_of_freaks_resample_9();
  test_waveform_from_file__test_circus_of_freaks_resample_10();
  test_waveform_from_file__test_circus_of_freaks_resample_11();
  test_waveform_from_file__test_circus_of_freaks_resample_12();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_1();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_2();
  test_waveform_from_file__test_circus_of_freaks_start_end_milliseconds_resample_repeat_pad_ending_3();
  test_waveform_resample_method__test_circus_of_freaks_no_change_1();
  test_waveform_resample_method__test_circus_of_freaks_44099();
  test_waveform_resample_method__test_circus_of_freaks_44101();
  test_waveform_resample_method__test_circus_of_freaks_22050();
  test_waveform_resample_method__test_circus_of_freaks_11025();
  test_waveform_resample_method__test_circus_of_freaks_88200();
  printf("\n\n == ALL TESTS HAVE PASSED! == \n\n");
  return 0;
}
