#pragma once
#include <memory>
#include <keyfinder/keyfinder.h>
#include <keyfinder/audiodata.h>

// Forward declare our wrapper types so cxx can reference them
namespace keyfinder_bridge {
    struct KeyFinderWrapper;
    struct AudioDataWrapper;
}

// Include the cxx-generated header so KeyFinderKey enum is defined
#include "libkeyfinder-sys/src/lib.rs.h"

namespace keyfinder_bridge {

// Now provide full definitions of wrapper types
struct KeyFinderWrapper {
    KeyFinder::KeyFinder kf;
};

struct AudioDataWrapper {
    KeyFinder::AudioData audio;
};

// Create new instances
std::unique_ptr<KeyFinderWrapper> new_keyfinder();
std::unique_ptr<AudioDataWrapper> new_audiodata();

// KeyFinder methods
KeyFinderKey key_of_audio(KeyFinderWrapper& kf, const AudioDataWrapper& audio);

// AudioData methods
void set_frame_rate(AudioDataWrapper& audio, uint32_t frame_rate);
void set_channels(AudioDataWrapper& audio, uint32_t channels);
uint32_t get_sample_count(const AudioDataWrapper& audio);
void add_to_sample_count(AudioDataWrapper& audio, uint32_t samples);
void reset_iterators(AudioDataWrapper& audio);
void advance_write_iterator(AudioDataWrapper& audio);
void set_sample_at_write_iterator(AudioDataWrapper& audio, float sample);

} // namespace keyfinder_bridge
