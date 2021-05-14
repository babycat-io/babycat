#include "../babycat.h"
#include <stdio.h>

int main() {
  babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
  babycat_FloatWaveformResult waveform_result =
      babycat_float_waveform_from_file(
          "audio-for-tests/circus-of-freaks/track.mp3", decode_args);
  if (waveform_result.error_num != 0) {
    printf("Decoding error: %u", waveform_result.error_num);
    return 1;
  }
  struct babycat_FloatWaveform *waveform = waveform_result.result;
  uint32_t num_frames = babycat_float_waveform_get_num_frames(waveform);
  uint32_t num_channels = babycat_float_waveform_get_num_channels(waveform);
  uint32_t frame_rate_hz = babycat_float_waveform_get_frame_rate_hz(waveform);
  printf("Decoded %u frames with %u channels at %u hz\n", num_frames,
         num_channels, frame_rate_hz);

  return 0;
}