#include "../babycat.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>

// We call this macro at the end of every unit test
// so we know that the test succeeded.
#define SUCCESS() fprintf(stderr, "Success: %s\n", __FUNCTION__)

#define ISCLOSE_EPSILON 0.0001

#define ISCLOSE(a, b) fabs(a - b) < ISCLOSE_EPSILON

void test_babycat_from_frames_of_silence_1() {
  babycat_FloatWaveform *waveform =
      babycat_float_waveform_from_frames_of_silence(44100, 2, 10);
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 10);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);

  u_int64_t num_samples = babycat_float_waveform_get_num_samples(waveform);
  const float *samples =
      babycat_float_waveform_get_interleaved_samples(waveform);
  for (u_int64_t i = 0; i < num_samples; ++i) {
    assert(ISCLOSE(samples[i], 0.0));
  }
  babycat_float_waveform_free(waveform);
  SUCCESS();
}

void test_babycat_from_filename_1() {
  babycat_DecodeArgs decode_args = {};
  babycat_FloatWaveformResult waveform_result =
      babycat_float_waveform_from_file(
          "./audio-for-tests/circus-of-freaks/track.mp3", decode_args);
  assert(waveform_result.error_num == babycat_NO_ERROR);
  babycat_FloatWaveform *waveform = waveform_result.result;
  assert(babycat_float_waveform_get_num_channels(waveform) == 2);
  assert(babycat_float_waveform_get_num_frames(waveform) == 2492928);
  assert(babycat_float_waveform_get_frame_rate_hz(waveform) == 44100);

  babycat_float_waveform_free(waveform);
  SUCCESS();
}

int main() {
  test_babycat_from_frames_of_silence_1();
  test_babycat_from_filename_1();
  return 0;
}