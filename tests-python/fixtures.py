"""
Test fixtures.

These fxitures mirror the ones in ``../tests/fixtures.rs``
"""
AT_FILENAME = "./audio-for-tests/andreas-theme/track.flac"
AT_NUM_CHANNELS = 2
AT_NUM_FRAMES = 9586415
AT_FRAME_RATE_HZ = 44100

BT_FILENAME = "./audio-for-tests/blippy-trance/track.wav"
BT_NUM_CHANNELS = 2
BT_NUM_FRAMES = 5292911
BT_FRAME_RATE_HZ = 44100

COF_FILENAME = "./audio-for-tests/circus-of-freaks/track.flac"
COF_NUM_CHANNELS = 2
COF_NUM_FRAMES = 2491247
COF_FRAME_RATE_HZ = 44100

LCT_FILENAME = "./audio-for-tests/left-channel-tone/track.flac"
LCT_NUM_CHANNELS = 2
LCT_NUM_FRAMES = 1323000
LCT_FRAME_RATE_HZ = 44100

MONO_DTMF_FILENAME = "./audio-for-tests/mono-dtmf-tones/track.flac"
MONO_DTMF_NUM_CHANNELS = 1
MONO_DTMF_NUM_FRAMES = 441000
MONO_DTMF_FRAME_RATE_HZ = 44100

OHFY_FILENAME = "./audio-for-tests/on-hold-for-you/track.flac"
OHFY_NUM_CHANNELS = 2
OHFY_NUM_FRAMES = 9619823
OHFY_FRAME_RATE_HZ = 44100

TMS_FILENAME = "./audio-for-tests/tone-missing-sounds/track.flac"
TMS_NUM_CHANNELS = 1
TMS_NUM_FRAMES = 1323000
TMS_FRAME_RATE_HZ = 44100

VR_FILENAME = "./audio-for-tests/voxel-revolution/track.flac"
VR_NUM_CHANNELS = 2
VR_NUM_FRAMES = 5728367
VR_FRAME_RATE_HZ = 44100

ALL_FILENAMES = [
    AT_FILENAME,
    BT_FILENAME,
    COF_FILENAME,
    LCT_FILENAME,
    MONO_DTMF_FILENAME,
    OHFY_FILENAME,
    TMS_FILENAME,
    VR_FILENAME,
]

ALL_NUM_CHANNELS = [
    AT_NUM_CHANNELS,
    BT_NUM_CHANNELS,
    COF_NUM_CHANNELS,
    LCT_NUM_CHANNELS,
    MONO_DTMF_NUM_CHANNELS,
    OHFY_NUM_CHANNELS,
    TMS_NUM_CHANNELS,
    VR_NUM_CHANNELS,
]

ALL_NUM_FRAMES = [
    AT_NUM_FRAMES,
    BT_NUM_FRAMES,
    COF_NUM_FRAMES,
    LCT_NUM_FRAMES,
    MONO_DTMF_NUM_FRAMES,
    OHFY_NUM_FRAMES,
    TMS_NUM_FRAMES,
    VR_NUM_FRAMES,
]

ALL_FRAME_RATE_HZ = [
    AT_FRAME_RATE_HZ,
    BT_FRAME_RATE_HZ,
    COF_FRAME_RATE_HZ,
    LCT_FRAME_RATE_HZ,
    MONO_DTMF_FRAME_RATE_HZ,
    OHFY_FRAME_RATE_HZ,
    TMS_FRAME_RATE_HZ,
    VR_FRAME_RATE_HZ,
]
