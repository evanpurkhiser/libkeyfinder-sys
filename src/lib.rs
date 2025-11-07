//! Rust bindings to libkeyfinder - musical key detection for audio.
//!
//! This crate provides safe Rust bindings to the [libkeyfinder](https://github.com/mixxxdj/libkeyfinder)
//! C++ library for detecting the musical key of audio files.
//!
//! # Overview
//!
//! libkeyfinder analyzes **decoded PCM audio samples** to detect musical keys.
//! You must decode your audio files (MP3, WAV, FLAC, etc.) into raw PCM samples
//! before using this library. Use a library like [ffmpeg](https://docs.rs/ffmpeg-next/)
//! or [symphonia](https://docs.rs/symphonia/) for audio decoding.
//!
//! # Quick Start
//!
//! ```
//! use libkeyfinder_sys::{KeyFinder, AudioData};
//!
//! // Create audio data container
//! let mut audio = AudioData::new();
//! audio.set_frame_rate(44100);
//! audio.set_channels(2);
//!
//! // Add decoded PCM samples (normalized to range [-1.0, 1.0])
//! // Your decoded PCM audio here
//! let samples: Vec<f32> = vec![0.0, 0.5, -0.3, 0.8];
//! audio.extend(samples);
//!
//! // Detect the key
//! let mut kf = KeyFinder::new();
//! let key = kf.key_of_audio(&audio);
//! println!("Detected key: {:?}", key);
//! ```
//!
//! # Working with iterators
//!
//! `AudioData` implements `Extend` and `FromIterator`, making it easy to work with
//! audio sample streams:
//!
//! ```
//! use libkeyfinder_sys::AudioData;
//!
//! // Collect from an iterator
//! let samples = vec![0.1, 0.2, 0.3];
//! let audio: AudioData = samples.into_iter().collect();
//!
//! // Or extend an existing container
//! let mut audio = AudioData::new();
//! audio.set_frame_rate(48000);
//! audio.set_channels(1);
//! let more_samples = vec![0.4, 0.5, 0.6];
//! audio.extend(more_samples.iter().copied());
//! ```
//!
//! # Audio preprocessing
//!
//! ```
//! use libkeyfinder_sys::AudioData;
//!
//! let mut audio = AudioData::new();
//! audio.set_frame_rate(44100);
//! audio.set_channels(2);
//! // ... add samples ...
//!
//! // Convert to mono for more efficient processing
//! audio.reduce_to_mono();
//!
//! // Downsample by a factor of 2
//! audio.downsample(2);
//!
//! assert_eq!(audio.channels(), 1);
//! assert_eq!(audio.frame_rate(), 22050);
//! ```

#[doc(hidden)]
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("libkeyfinder-sys/src/bridge.h");

        #[namespace = "keyfinder_bridge"]
        type KeyFinderWrapper;
        #[namespace = "keyfinder_bridge"]
        type AudioDataWrapper;

        #[namespace = "keyfinder_bridge"]
        fn new_keyfinder() -> UniquePtr<KeyFinderWrapper>;
        #[namespace = "keyfinder_bridge"]
        fn new_audiodata() -> UniquePtr<AudioDataWrapper>;

        #[namespace = "keyfinder_bridge"]
        fn key_of_audio(kf: Pin<&mut KeyFinderWrapper>, audio: &AudioDataWrapper) -> u32;

        #[namespace = "keyfinder_bridge"]
        fn set_frame_rate(audio: Pin<&mut AudioDataWrapper>, frame_rate: u32);
        #[namespace = "keyfinder_bridge"]
        fn set_channels(audio: Pin<&mut AudioDataWrapper>, channels: u32);
        #[namespace = "keyfinder_bridge"]
        fn get_channels(audio: &AudioDataWrapper) -> u32;
        #[namespace = "keyfinder_bridge"]
        fn get_frame_rate(audio: &AudioDataWrapper) -> u32;
        #[namespace = "keyfinder_bridge"]
        fn get_sample_count(audio: &AudioDataWrapper) -> u32;
        #[namespace = "keyfinder_bridge"]
        fn get_frame_count(audio: &AudioDataWrapper) -> u32;
        #[namespace = "keyfinder_bridge"]
        fn add_to_sample_count(audio: Pin<&mut AudioDataWrapper>, samples: u32);
        #[namespace = "keyfinder_bridge"]
        fn reset_iterators(audio: Pin<&mut AudioDataWrapper>);
        #[namespace = "keyfinder_bridge"]
        fn advance_write_iterator(audio: Pin<&mut AudioDataWrapper>, by: u32);
        #[namespace = "keyfinder_bridge"]
        fn set_sample_at_write_iterator(audio: Pin<&mut AudioDataWrapper>, sample: f32);
        #[namespace = "keyfinder_bridge"]
        fn reduce_to_mono(audio: Pin<&mut AudioDataWrapper>);
        #[namespace = "keyfinder_bridge"]
        fn downsample(audio: Pin<&mut AudioDataWrapper>, factor: u32);
    }
}

// Higher-level Rust API

/// Musical key detection result.
///
/// Represents the 24 possible musical keys (12 major + 12 minor) plus silence.
/// These values match the KeyFinder::key_t enum from libkeyfinder.
///
/// # Example
///
/// ```
/// use libkeyfinder_sys::{KeyFinder, AudioData, KeyFinderKey};
///
/// let mut audio = AudioData::new();
/// audio.set_frame_rate(44100);
/// audio.set_channels(1);
/// // ... add actual audio samples ...
///
/// let mut kf = KeyFinder::new();
/// let key = kf.key_of_audio(&audio);
///
/// match key {
///     KeyFinderKey::AMajor => println!("Key: A Major"),
///     KeyFinderKey::AMinor => println!("Key: A Minor"),
///     KeyFinderKey::Silence => println!("No key detected"),
///     _ => println!("Key: {:?}", key),
/// }
/// ```
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyFinderKey {
    AMajor = 0,
    AMinor = 1,
    BFlatMajor = 2,
    BFlatMinor = 3,
    BMajor = 4,
    BMinor = 5,
    CMajor = 6,
    CMinor = 7,
    DFlatMajor = 8,
    DFlatMinor = 9,
    DMajor = 10,
    DMinor = 11,
    EFlatMajor = 12,
    EFlatMinor = 13,
    EMajor = 14,
    EMinor = 15,
    FMajor = 16,
    FMinor = 17,
    GFlatMajor = 18,
    GFlatMinor = 19,
    GMajor = 20,
    GMinor = 21,
    AFlatMajor = 22,
    AFlatMinor = 23,
    Silence = 24,
}

impl KeyFinderKey {
    /// Convert from the C++ enum value (represented as u32).
    fn from_u32(value: u32) -> Self {
        // Safe because we have explicit discriminants matching the C++ enum
        if value <= 24 {
            unsafe { std::mem::transmute(value) }
        } else {
            KeyFinderKey::Silence
        }
    }
}

/// The main key detection engine.
///
/// This wraps the C++ KeyFinder::KeyFinder class and provides the core
/// functionality for analyzing audio and detecting musical keys.
///
/// # Example
///
/// ```
/// use libkeyfinder_sys::{KeyFinder, AudioData, KeyFinderKey};
///
/// let mut audio = AudioData::new();
/// audio.set_frame_rate(44100);
/// audio.set_channels(2);
/// // 1 second of silence
/// audio.extend(vec![0.0; 44100]);
///
/// let mut kf = KeyFinder::new();
/// let key = kf.key_of_audio(&audio);
/// assert_eq!(key, KeyFinderKey::Silence);
/// ```
pub struct KeyFinder {
    inner: cxx::UniquePtr<ffi::KeyFinderWrapper>,
}

impl KeyFinder {
    /// Create a new KeyFinder instance.
    pub fn new() -> Self {
        Self {
            inner: ffi::new_keyfinder(),
        }
    }

    /// Analyze audio data and detect its musical key.
    ///
    /// This performs chromagram generation, spectral analysis, and tone profile
    /// matching to determine the most likely key of the audio.
    ///
    /// Returns `KeyFinderKey::Silence` if no key can be detected.
    pub fn key_of_audio(&mut self, audio: &AudioData) -> KeyFinderKey {
        let value = ffi::key_of_audio(self.inner.pin_mut(), &audio.inner);
        KeyFinderKey::from_u32(value)
    }
}

impl Default for KeyFinder {
    fn default() -> Self {
        Self::new()
    }
}

/// Container for decoded PCM audio samples.
///
/// This wraps the C++ KeyFinder::AudioData class and manages the audio data
/// that will be analyzed for key detection.
///
/// **Important**: You must provide decoded PCM audio samples normalized to the
/// range [-1.0, 1.0]. This library does not decode audio files - use a separate
/// library like ffmpeg or symphonia to decode MP3, WAV, FLAC, etc. into raw PCM.
///
/// # Example
///
/// ```
/// use libkeyfinder_sys::AudioData;
///
/// // Create and configure
/// let mut audio = AudioData::new();
/// audio.set_frame_rate(44100);
/// audio.set_channels(2);
///
/// // Add decoded PCM samples (normalized to [-1.0, 1.0])
/// let samples: Vec<f32> = vec![0.1, -0.2, 0.5, -0.8];
/// audio.extend(samples);
///
/// // Query properties
/// assert_eq!(audio.channels(), 2);
/// assert_eq!(audio.frame_rate(), 44100);
/// assert_eq!(audio.sample_count(), 4);
/// // 4 samples / 2 channels = 2 frames
/// assert_eq!(audio.frame_count(), 2);
/// ```
pub struct AudioData {
    inner: cxx::UniquePtr<ffi::AudioDataWrapper>,
}

impl AudioData {
    /// Create a new empty AudioData container.
    pub fn new() -> Self {
        Self {
            inner: ffi::new_audiodata(),
        }
    }

    /// Set the sample rate of the audio (e.g., 44100, 48000).
    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        ffi::set_frame_rate(self.inner.pin_mut(), frame_rate);
    }

    /// Get the sample rate of the audio.
    pub fn frame_rate(&self) -> u32 {
        ffi::get_frame_rate(&self.inner)
    }

    /// Set the number of audio channels (1 for mono, 2 for stereo, etc.).
    pub fn set_channels(&mut self, channels: u32) {
        ffi::set_channels(self.inner.pin_mut(), channels);
    }

    /// Get the number of audio channels.
    pub fn channels(&self) -> u32 {
        ffi::get_channels(&self.inner)
    }

    /// Get the total number of samples currently in the container.
    pub fn sample_count(&self) -> u32 {
        ffi::get_sample_count(&self.inner)
    }

    /// Get the total number of frames (samples per channel).
    pub fn frame_count(&self) -> u32 {
        ffi::get_frame_count(&self.inner)
    }

    /// Convert stereo (or multi-channel) audio to mono by averaging channels.
    pub fn reduce_to_mono(&mut self) {
        ffi::reduce_to_mono(self.inner.pin_mut());
    }

    /// Downsample the audio by the given factor.
    ///
    /// For example, `downsample(2)` will halve the sample rate.
    pub fn downsample(&mut self, factor: u32) {
        ffi::downsample(self.inner.pin_mut(), factor);
    }
}

impl Default for AudioData {
    fn default() -> Self {
        Self::new()
    }
}

impl Extend<f32> for AudioData {
    /// Extend the audio data with samples from an iterator.
    ///
    /// Samples should be in the range [-1.0, 1.0].
    fn extend<T: IntoIterator<Item = f32>>(&mut self, iter: T) {
        // Collect into a Vec for efficient batch processing
        let samples: Vec<f32> = iter.into_iter().collect();
        if samples.is_empty() {
            return;
        }

        let old_count = self.sample_count();
        ffi::add_to_sample_count(self.inner.pin_mut(), samples.len() as u32);
        ffi::reset_iterators(self.inner.pin_mut());
        ffi::advance_write_iterator(self.inner.pin_mut(), old_count);

        for &sample in &samples {
            ffi::set_sample_at_write_iterator(self.inner.pin_mut(), sample);
            ffi::advance_write_iterator(self.inner.pin_mut(), 1);
        }
    }
}

impl FromIterator<f32> for AudioData {
    /// Create audio data from an iterator of samples.
    ///
    /// Samples should be in the range [-1.0, 1.0].
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut audio = AudioData::new();
        audio.extend(iter);
        audio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_data_new() {
        let audio = AudioData::new();
        assert_eq!(audio.sample_count(), 0);
    }

    #[test]
    fn test_audio_data_set_and_get_properties() {
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(2);

        assert_eq!(audio.frame_rate(), 44100);
        assert_eq!(audio.channels(), 2);
    }

    #[test]
    fn test_audio_data_extend() {
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(2);

        let samples = vec![0.1, -0.2, 0.3, -0.4];
        audio.extend(samples);

        assert_eq!(audio.sample_count(), 4);
        assert_eq!(audio.frame_count(), 2);
    }

    #[test]
    fn test_audio_data_from_iterator() {
        let samples = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
        let mut audio: AudioData = samples.into_iter().collect();
        audio.set_frame_rate(48000);
        audio.set_channels(1);

        assert_eq!(audio.sample_count(), 6);
        assert_eq!(audio.frame_count(), 6);
        assert_eq!(audio.channels(), 1);
    }

    #[test]
    fn test_audio_data_reduce_to_mono() {
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(2);

        // Stereo: left, right, left, right
        audio.extend(vec![0.5, -0.5, 0.3, -0.3]);

        audio.reduce_to_mono();

        assert_eq!(audio.channels(), 1);
        assert_eq!(audio.frame_count(), 2);
    }

    #[test]
    fn test_audio_data_downsample() {
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(1);

        audio.extend(vec![0.1, 0.2, 0.3, 0.4]);

        audio.downsample(2);

        assert_eq!(audio.frame_rate(), 22050);
    }

    #[test]
    fn test_keyfinder_new() {
        let _kf = KeyFinder::new();
    }

    #[test]
    fn test_keyfinder_silence() {
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(1);

        // One second of silence
        audio.extend(vec![0.0; 44100]);

        let mut kf = KeyFinder::new();
        let key = kf.key_of_audio(&audio);

        assert_eq!(key, KeyFinderKey::Silence);
    }

    #[test]
    fn test_keyfinder_key_enum_values() {
        // Test that all key enum values are distinct
        assert_eq!(KeyFinderKey::AMajor as u32, 0);
        assert_eq!(KeyFinderKey::AMinor as u32, 1);
        assert_eq!(KeyFinderKey::Silence as u32, 24);
    }

    #[test]
    fn test_keyfinder_key_from_u32() {
        assert_eq!(KeyFinderKey::from_u32(0), KeyFinderKey::AMajor);
        assert_eq!(KeyFinderKey::from_u32(1), KeyFinderKey::AMinor);
        assert_eq!(KeyFinderKey::from_u32(24), KeyFinderKey::Silence);
        assert_eq!(KeyFinderKey::from_u32(999), KeyFinderKey::Silence);
    }

    #[test]
    fn test_audio_data_default() {
        let audio = AudioData::default();
        assert_eq!(audio.sample_count(), 0);
    }

    #[test]
    fn test_keyfinder_default() {
        let mut kf = KeyFinder::default();
        let mut audio = AudioData::new();
        audio.set_frame_rate(44100);
        audio.set_channels(1);
        audio.extend(vec![0.0; 1000]);

        let key = kf.key_of_audio(&audio);
        assert_eq!(key, KeyFinderKey::Silence);
    }
}
