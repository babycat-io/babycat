#include "oboe/OboeExt.h"

// !!!!!!!!!!!!!! DIRTY HACK !!!!!!!!!!!!!
/*extern "C" {
  void *__dso_handle __attribute__((weak));
}*/

namespace oboe {
  void AudioStream_delete(AudioStream *oboeStream) {
    delete oboeStream;
  }

  Result AudioStream_open(AudioStream *oboeStream) {
    return oboeStream->open();
  }

  Result AudioStream_requestStart(AudioStream *oboeStream) {
    return oboeStream->requestStart();
  }

  Result AudioStream_requestPause(AudioStream *oboeStream) {
    return oboeStream->requestPause();
  }

  Result AudioStream_requestFlush(AudioStream *oboeStream) {
    return oboeStream->requestFlush();
  }

  Result AudioStream_requestStop(AudioStream *oboeStream) {
    return oboeStream->requestStop();
  }

  StreamState AudioStream_getState(const AudioStream *oboeStream) {
    return oboeStream->getState();
  }

  Result AudioStream_waitForStateChange(AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds) {
    return oboeStream->waitForStateChange(inputState,
                                          nextState,
                                          timeoutNanoseconds);
  }

  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames) {
    return oboeStream->setBufferSizeInFrames(requestedFrames);
  }

  ResultWithValue<int32_t>
  AudioStream_getXRunCount(const AudioStream *oboeStream) {
    return oboeStream->getXRunCount();
  }

  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream) {
    return oboeStream->isXRunCountSupported();
  }

  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream) {
    return oboeStream->getFramesPerBurst();
  }

  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream) {
    return oboeStream->calculateLatencyMillis();
  }

  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream) {
    return oboeStream->getAudioApi();
  }

  ResultWithValue<int32_t> AudioStream_read(AudioStream *oboeStream,
                                            void* buffer,
                                            int32_t numFrames,
                                            int64_t timeoutNanoseconds) {
    return oboeStream->read(buffer, numFrames, timeoutNanoseconds);
  }

  ResultWithValue<int32_t> AudioStream_write(AudioStream *oboeStream,
                                             const void* buffer,
                                             int32_t numFrames,
                                             int64_t timeoutNanoseconds) {
    return oboeStream->write(buffer, numFrames, timeoutNanoseconds);
  }

  AudioStreamBase* AudioStream_getBase(AudioStream *oboeStream) {
    return static_cast<AudioStreamBase*>(oboeStream);
  }
}
