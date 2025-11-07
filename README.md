# libkeyfinder-sys

Rust bindings for [libkeyfinder](https://github.com/mixxxdj/libkeyfinder), a C++ library for estimating the musical key of digital audio.

## Requirements

- libkeyfinder >= 2.2
- C++11 compiler
- Rust 2024 edition

### Installing libkeyfinder

On macOS with Homebrew:
```bash
brew install libkeyfinder
```

On Ubuntu/Debian:
```bash
sudo apt-get install libkeyfinder-dev
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libkeyfinder-sys = { path = "." }
```

Example:

```rust
use libkeyfinder_sys::{AudioData, KeyFinder, KeyFinderKey};

fn main() {
    // Create a KeyFinder instance
    let mut kf = KeyFinder::new();

    // Create audio data
    let mut audio = AudioData::new();
    audio.set_frame_rate(44100);
    audio.set_channels(2);

    // Add audio samples
    let samples: Vec<f32> = vec![/* your audio samples */];
    audio.add_samples(&samples);

    // Detect the key
    let key = kf.key_of_audio(&audio);

    match key {
        KeyFinderKey::Silence => println!("Silent audio"),
        KeyFinderKey::AMajor => println!("Key: A Major"),
        KeyFinderKey::AMinor => println!("Key: A Minor"),
        // ... etc
        _ => println!("Key: {:?}", key),
    }
}
```

## API

### `KeyFinder`

The main key detection engine.

- `KeyFinder::new()` - Create a new KeyFinder instance
- `key_of_audio(&mut self, audio: &AudioData) -> KeyFinderKey` - Analyze audio and return the detected key

### `AudioData`

Container for audio samples.

- `AudioData::new()` - Create a new AudioData instance
- `set_frame_rate(&mut self, frame_rate: u32)` - Set the sample rate (e.g., 44100)
- `set_channels(&mut self, channels: u32)` - Set number of channels (1 for mono, 2 for stereo)
- `add_samples(&mut self, samples: &[f32])` - Add audio samples (convenience method)
- `get_sample_count(&self) -> u32` - Get total number of samples

### `KeyFinderKey`

Enum representing musical keys:

- Major keys: `AMajor`, `BFlatMajor`, `BMajor`, `CMajor`, `DFlatMajor`, `DMajor`, `EFlatMajor`, `EMajor`, `FMajor`, `GFlatMajor`, `GMajor`, `AFlatMajor`
- Minor keys: `AMinor`, `BFlatMinor`, `BMinor`, `CMinor`, `DFlatMinor`, `DMinor`, `EFlatMinor`, `EMinor`, `FMinor`, `GFlatMinor`, `GMinor`, `AFlatMinor`
- `Silence` - No key detected (silent audio)

## Architecture

This crate uses [cxx](https://cxx.rs/) to create safe Rust bindings to the C++ libkeyfinder library. The bindings are minimal and only expose the essential API needed for key detection, matching what's used in the original keyfinder-cli tool.

## License

This crate provides bindings to libkeyfinder, which is licensed under GPL v3 or later. Therefore, this crate is also GPL v3 or later.
