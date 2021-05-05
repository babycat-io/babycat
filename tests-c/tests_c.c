#include "../babycat.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>

// We call this macro at the end of every unit test
// so we know that the test succeeded.
#define SUCCESS() fprintf(stderr, "Success: %s\n", __FUNCTION__);

void test_babycat_from_frames_of_silence_1() {
  CAPI_FloatWaveform* waveform = babycat_float_waveform_from_frames_of_silence(
    44100, 2, 1000
  );
  uint32_t frame_rate_hz = babycat_float_waveform_get_frame_rate_hz(waveform);
  assert(frame_rate_hz == 44100);
  SUCCESS();
}

int main() {
  test_babycat_from_frames_of_silence_1();
  return 0;
}