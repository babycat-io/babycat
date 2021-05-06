const assert = require("assert");
const fs = require("fs");
const path = require("path");
const { describe, it } = require("mocha");
const babycat = require("../target/wasm/nodejs/babycat.js");

const AUDIO_FOR_TESTS_DIR = path.join(__dirname, "/../audio-for-tests/");
const COF_FILENAME = path.join(
  AUDIO_FOR_TESTS_DIR,
  "circus-of-freaks/track.mp3"
);

const COF = fs.readFileSync(COF_FILENAME);
const COF_NUM_FRAMES = 2492928;
const COF_NUM_CHANNELS = 2;
const COF_FRAME_RATE_HZ = 44100;

function assertWaveform(waveform, numChannels, numFrames, frameRateHz) {
  assert.strictEqual(waveform.numChannels(), numChannels);
  assert.strictEqual(waveform.numFrames(), numFrames);
  assert.strictEqual(waveform.frameRateHz(), frameRateHz);
}

describe("FloatWaveform.fromFramesOfSilence", function () {
  it("should work", function () {
    const waveform = babycat.FloatWaveform.fromFramesOfSilence(44100, 2, 1000);
    assertWaveform(waveform, 2, 1000, 44100);
  });
});

describe("FloatWaveform.fromMillisecondsOfSilence", function () {
  it("should work", function () {
    const waveform = babycat.FloatWaveform.fromMillisecondsOfSilence(
      44100,
      2,
      10000
    );
    assertWaveform(waveform, 2, 44100 * 10, 44100);
  });
});

describe("FloatWaveform.fromEncodedArray", function () {
  this.timeout(0);

  it("test_circus_of_freaks_default_1", function () {
    const decodeArgs = {};
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(
      waveform,
      COF_NUM_CHANNELS,
      COF_NUM_FRAMES,
      COF_FRAME_RATE_HZ
    );
  });

  it("test_circus_of_freaks_wrong_time_offset_1", function () {
    const decodeArgs = {
      start_time_milliseconds: 1000,
      end_time_milliseconds: 999,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_wrong_time_offset_2", function () {
    const decodeArgs = {
      start_time_milliseconds: 1000,
      end_time_milliseconds: 1000,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1", function () {
    const decodeArgs = {
      start_time_milliseconds: 5,
      end_time_milliseconds: 0,
      zero_pad_ending: true,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_get_channels_1", function () {
    const decodeArgs = {
      num_channels: 1,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_get_channels_2", function () {
    const decodeArgs = {
      num_channels: 2,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, 2, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_get_channels_too_many_1", function () {
    const decodeArgs = {
      num_channels: 3,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_convert_to_mono_1", function () {
    const decodeArgs = {
      num_channels: 2,
      convert_to_mono: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_convert_to_mono_2", function () {
    const decodeArgs = {
      convert_to_mono: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, 1, COF_NUM_FRAMES, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_convert_to_mono_invalid_1", function () {
    const decodeArgs = {
      num_channels: 1,
      convert_to_mono: true,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_start_end_milliseconds_1", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 1,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_2", function () {
    const decodeArgs = {
      start_time_milliseconds: 10,
      end_time_milliseconds: 11,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_3", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 30000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_4", function () {
    const decodeArgs = {
      start_time_milliseconds: 15000,
      end_time_milliseconds: 45000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_5", function () {
    const decodeArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 60000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1169928, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 1,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 44, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2", function () {
    const decodeArgs = {
      start_time_milliseconds: 10,
      end_time_milliseconds: 11,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assert.strictEqual(waveform.numChannels(), COF_NUM_CHANNELS);
    assert.strictEqual(waveform.numFrames(), 44);
    assert.strictEqual(waveform.frameRateHz(), COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 30000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4", function () {
    const decodeArgs = {
      start_time_milliseconds: 15000,
      end_time_milliseconds: 45000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5", function () {
    const decodeArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 60000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7", function () {
    const decodeArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8", function () {
    const decodeArgs = {
      start_time_milliseconds: 30000,
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2646000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_end_milliseconds_zero_pad_ending_1", function () {
    const decodeArgs = {
      end_time_milliseconds: 90000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 3969000, COF_FRAME_RATE_HZ);
  });

  it("test_circus_of_freaks_invalid_resample_1", function () {
    const decodeArgs = {
      frame_rate_hz: 1,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_invalid_resample_2", function () {
    const decodeArgs = {
      frame_rate_hz: 20,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_invalid_resample_3", function () {
    const decodeArgs = {
      frame_rate_hz: 172,
    };
    assert.throws(() =>
      babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs)
    );
  });

  it("test_circus_of_freaks_resample_no_change_1", function () {
    const decodeArgs = {
      frame_rate_hz: COF_FRAME_RATE_HZ,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(
      waveform,
      COF_NUM_CHANNELS,
      COF_NUM_FRAMES,
      COF_FRAME_RATE_HZ
    );
  });

  it("test_circus_of_freaks_resample_1", function () {
    const decodeArgs = {
      frame_rate_hz: 22050,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1246464, 22050);
  });

  it("test_circus_of_freaks_resample_2", function () {
    const decodeArgs = {
      frame_rate_hz: 11025,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 623232, 11025);
  });

  it("test_circus_of_freaks_resample_3", function () {
    const decodeArgs = {
      frame_rate_hz: 88200,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 4985856, 88200);
  });

  it("test_circus_of_freaks_resample_4", function () {
    const decodeArgs = {
      frame_rate_hz: 4410,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 249293, 4410);
  });

  it("test_circus_of_freaks_resample_5", function () {
    const decodeArgs = {
      frame_rate_hz: 44099,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2492872, 44099);
  });

  it("test_circus_of_freaks_resample_6", function () {
    const decodeArgs = {
      frame_rate_hz: 48000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2713392, 48000);
  });

  it("test_circus_of_freaks_resample_7", function () {
    const decodeArgs = {
      frame_rate_hz: 60000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 3391739, 60000);
  });

  it("test_circus_of_freaks_resample_8", function () {
    const decodeArgs = {
      frame_rate_hz: 88200,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 4985856, 88200);
  });

  it("test_circus_of_freaks_resample_9", function () {
    const decodeArgs = {
      frame_rate_hz: 96000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 5426783, 96000);
  });

  it("test_circus_of_freaks_resample_10", function () {
    const decodeArgs = {
      frame_rate_hz: 200,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 11306, 200);
  });

  it("test_circus_of_freaks_resample_11", function () {
    const decodeArgs = {
      frame_rate_hz: 2000,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 113058, 2000);
  });

  it("test_circus_of_freaks_resample_12", function () {
    const decodeArgs = {
      frame_rate_hz: 173,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 9780, 173);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 48000,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2880000, 48000);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 44099,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 2645940, 44099);
  });

  it("test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3", function () {
    const decodeArgs = {
      start_time_milliseconds: 0,
      end_time_milliseconds: 60000,
      frame_rate_hz: 22050,
      zero_pad_ending: true,
    };
    const waveform = babycat.FloatWaveform.fromEncodedArray(COF, decodeArgs);
    assertWaveform(waveform, COF_NUM_CHANNELS, 1323000, 22050);
  });
});
