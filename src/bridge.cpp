#include "libkeyfinder-sys/src/bridge.h"

namespace keyfinder_bridge {

std::unique_ptr<KeyFinderWrapper> new_keyfinder() {
    return std::unique_ptr<KeyFinderWrapper>(new KeyFinderWrapper());
}

std::unique_ptr<AudioDataWrapper> new_audiodata() {
    return std::unique_ptr<AudioDataWrapper>(new AudioDataWrapper());
}

uint32_t key_of_audio(KeyFinderWrapper& kf, const AudioDataWrapper& audio) {
    KeyFinder::key_t result = kf.kf.keyOfAudio(audio.audio);
    return static_cast<uint32_t>(result);
}

void set_frame_rate(AudioDataWrapper& audio, uint32_t frame_rate) {
    audio.audio.setFrameRate(frame_rate);
}

void set_channels(AudioDataWrapper& audio, uint32_t channels) {
    audio.audio.setChannels(channels);
}

uint32_t get_channels(const AudioDataWrapper& audio) {
    return audio.audio.getChannels();
}

uint32_t get_frame_rate(const AudioDataWrapper& audio) {
    return audio.audio.getFrameRate();
}

uint32_t get_sample_count(const AudioDataWrapper& audio) {
    return audio.audio.getSampleCount();
}

uint32_t get_frame_count(const AudioDataWrapper& audio) {
    return audio.audio.getFrameCount();
}

void add_to_sample_count(AudioDataWrapper& audio, uint32_t samples) {
    audio.audio.addToSampleCount(samples);
}

void reset_iterators(AudioDataWrapper& audio) {
    audio.audio.resetIterators();
}

void advance_write_iterator(AudioDataWrapper& audio, uint32_t by) {
    audio.audio.advanceWriteIterator(by);
}

void set_sample_at_write_iterator(AudioDataWrapper& audio, float sample) {
    audio.audio.setSampleAtWriteIterator(sample);
}

void reduce_to_mono(AudioDataWrapper& audio) {
    audio.audio.reduceToMono();
}

void downsample(AudioDataWrapper& audio, uint32_t factor) {
    audio.audio.downsample(factor);
}

} // namespace keyfinder_bridge
