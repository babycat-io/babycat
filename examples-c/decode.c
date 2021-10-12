#include "../babycat.h"
#include <stdio.h>

int main() {
  babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
  babycat_WaveformResult waveform_result = babycat_waveform_from_file(
      "audio-for-tests/circus-of-freaks/track.mp3", waveform_args);
  if (waveform_result.error_num != 0) {
    printf("Decoding error: %u", waveform_result.error_num);
    return 1;
  }
  struct babycat_Waveform *waveform = waveform_result.result;
  uint32_t num_frames = babycat_waveform_get_num_frames(waveform);
  uint32_t num_channels = babycat_waveform_get_num_channels(waveform);
  uint32_t frame_rate_hz = babycat_waveform_get_frame_rate_hz(waveform);
  printf("Decoded %u frames with %u channels at %u hz\n", num_frames,
         num_channels, frame_rate_hz);

  return 0;
}