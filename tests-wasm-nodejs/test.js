const assert = require("assert");
const fs = require("fs");
const path = require("path");
const { describe, it } = require("mocha");
const babycat = require("../target/frontend-wasm/release/nodejs/babycat.js");

const AUDIO_FOR_TESTS_DIR = path.join(__dirname, "/../audio-for-tests/");
const COF_FILENAME = path.join(
  AUDIO_FOR_TESTS_DIR,
  "circus-of-freaks/track.mp3"
);

const COF = fs.readFileSync(COF_FILENAME);
const COF_NUM_FRAMES = 2491776;
const COF_NUM_CHANNELS = 2;
const COF_FRAME_RATE_HZ = 44100;

function assertWaveform(waveform, numChannels, numFrames, frameRateHz) {
  assert.strictEqual(waveform.numChannels(), numChannels);
  assert.strictEqual(waveform.numFrames(), numFrames);
  assert.strictEqual(waveform.frameRateHz(), frameRateHz);
}

describe("Waveform.fromFramesOfSilence", function () {
  it("should work", function () {
    const waveform = babycat.Waveform.fromFramesOfSilence(44100, 2, 1000);
    assertWaveform(waveform, 2, 1000, 44100);
  });
});

describe("Waveform.fromMillisecondsOfSilence", function () {
  it("should work", function () {
    const waveform = babycat.Waveform.fromMillisecondsOfSilence(
      44100,
      2,
      10000
    );
    assertWaveform(waveform, 2, 44100 * 10, 44100);
  });
});

describe("Waveform.fromEncodedArray", function () {
  this.timeout(0);

  it("test_circus_of_freaks_default_1", function () {
    const WaveformArgs = {};
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(
      waveform,
      COF_NUM_CHANNELS,
      COF_NUM_FRAMES,
      COF_FRAME_RATE_HZ
    );
  });

  it("test_circus_of_freaks_wrong_time_offset_1", function () {
    const WaveformArgs = {
      start_time_milliseconds: 1000,
      end_time_milliseconds: 999,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_wrong_time_offset_2", function () {
    const WaveformArgs = {
      start_time_milliseconds: 1000,
      end_time_milliseconds: 1000,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1", function () {
    const WaveformArgs = {
      start_time_milliseconds: 5,
      end_time_milliseconds: 0,
      zero_pad_ending: true,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_get_channels_1", function () {
    const WaveformArgs = {
      num_channels: 1,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_get_channels_2", function () {
    const WaveformArgs = {
      num_channels: 2,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, 2, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_get_channels_too_many_1", function () {
    const WaveformArgs = {
      num_channels: 3,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_convert_to_mono_1", function () {
    const WaveformArgs = {
      num_channels: 2,
      convert_to_mono: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_convert_to_mono_2", function () {
    const WaveformArgs = {
      convert_to_mono: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_convert_to_mono_invalid_1", function () {
    const WaveformArgs = {
      num_channels: 1,
      convert_to_mono: true,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_start_end_milliseconds_1", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 1,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_2", function () {
    const WaveformArgs = {
      start_time_milliseconds: 10,
      end_time_milliseconds: 11,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_3", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 30000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_4", function () {
    const WaveformArgs = {
      start_time_milliseconds: 15000,
      end_time_milliseconds: 45000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_5", function () {
    const WaveformArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 60000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1168776, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 1,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2", function () {
    const WaveformArgs = {
      start_time_milliseconds: 10,
      end_time_milliseconds: 11,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assert.strictEqual(waveform.numChannels(), COF_NUM_CHANNELS);
    assert.strictEqual(waveform.numFrames(), 44);
    assert.strictEqual(waveform.frameRateHz(), COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 30000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4", function () {
    const WaveformArgs = {
      start_time_milliseconds: 15000,
      end_time_milliseconds: 45000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5", function () {
    const WaveformArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 60000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7", function () {
    const WaveformArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8", function () {
    const WaveformArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_end_milliseconds_zero_pad_ending_1", function () {
    const WaveformArgs = {
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_invalid_resample_1", function () {
    const WaveformArgs = {
      frame_rate_hz: 1,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_invalid_resample_2", function () {
    const WaveformArgs = {
      frame_rate_hz: 20,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_invalid_resample_3", function () {
    const WaveformArgs = {
      frame_rate_hz: 172,
    };
    assert.throws(() => babycat.Waveform.fromEncodedArray(COF, WaveformArgs));
  });

  it("test_circus_of_freaks_resample_no_change_1", function () {
    const WaveformArgs = {
      frame_rate_hz: COF_FRAME_RATE_HZ,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(
      waveform,
      COF_NUM_CHANNELS,
      COF_NUM_FRAMES,
      COF_FRAME_RATE_HZ
    );
  });

  it("test_circus_of_freaks_resample_1", function () {
    const WaveformArgs = {
      frame_rate_hz: 22050,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1245888, 22050);
  });

  it("test_circus_of_freaks_resample_2", function () {
    const WaveformArgs = {
      frame_rate_hz: 11025,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 622944, 11025);
  });

  it("test_circus_of_freaks_resample_3", function () {
    const WaveformArgs = {
      frame_rate_hz: 88200,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 4983552, 88200);
  });

  it("test_circus_of_freaks_resample_4", function () {
    const WaveformArgs = {
      frame_rate_hz: 4410,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 249178, 4410);
  });

  it("test_circus_of_freaks_resample_5", function () {
    const WaveformArgs = {
      frame_rate_hz: 44099,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2491720, 44099);
  });

  it("test_circus_of_freaks_resample_6", function () {
    const WaveformArgs = {
      frame_rate_hz: 48000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2712138, 48000);
  });

  it("test_circus_of_freaks_resample_7", function () {
    const WaveformArgs = {
      frame_rate_hz: 60000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 3390172, 60000);
  });

  it("test_circus_of_freaks_resample_8", function () {
    const WaveformArgs = {
      frame_rate_hz: 88200,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 4983552, 88200);
  });

  it("test_circus_of_freaks_resample_9", function () {
    const WaveformArgs = {
      frame_rate_hz: 96000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 5424275, 96000);
  });

  it("test_circus_of_freaks_resample_10", function () {
    const WaveformArgs = {
      frame_rate_hz: 200,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 11301, 200);
  });

  it("test_circus_of_freaks_resample_11", function () {
    const WaveformArgs = {
      frame_rate_hz: 2000,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 113006, 2000);
  });

  it("test_circus_of_freaks_resample_12", function () {
    const WaveformArgs = {
      frame_rate_hz: 173,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 9775, 173);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 48000,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2880000, 48000);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 44099,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2645940, 44099);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3", function () {
    const WaveformArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 22050,
      zero_pad_ending: true,
    };
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, 22050);
  });
});

describe("Waveform.resampleByMode", function () {
  this.timeout(0);

  function decodeAndAssertCOF(frameRateHz, expectedNumFrames) {
    const WaveformArgs = {};
    const waveform = babycat.Waveform.fromEncodedArray(COF, WaveformArgs);
    const resampled = waveform.resampleByMode(frameRateHz, 2);
    assertWaveform(resampled, COF_NUM_CHANNELS, expectedNumFrames, frameRateHz);
  }

  it("test_circus_of_freaks_no_change_1", function () {
    decodeAndAssertCOF(44100, 2491776);
  });

  it("test_circus_of_freaks_44099", function () {
    decodeAndAssertCOF(44099, 2491720);
  });

  it("test_circus_of_freaks_44101", function () {
    decodeAndAssertCOF(44101, 2491833);
  });

  it("test_circus_of_freaks_22050", function () {
    decodeAndAssertCOF(22050, COF_NUM_FRAMES / 2);
  });

  it("test_circus_of_freaks_11025", function () {
    decodeAndAssertCOF(11025, COF_NUM_FRAMES / 4);
  });

  it("test_circus_of_freaks_88200", function () {
    decodeAndAssertCOF(88200, COF_NUM_FRAMES * 2);
  });
});
