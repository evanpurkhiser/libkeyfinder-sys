#[cxx::bridge]
pub mod ffi {
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
        fn key_of_audio(kf: Pin<&mut KeyFinderWrapper>, audio: &AudioDataWrapper) -> KeyFinderKey;

        #[namespace = "keyfinder_bridge"]
        fn set_frame_rate(audio: Pin<&mut AudioDataWrapper>, frame_rate: u32);
        #[namespace = "keyfinder_bridge"]
        fn set_channels(audio: Pin<&mut AudioDataWrapper>, channels: u32);
        #[namespace = "keyfinder_bridge"]
        fn get_sample_count(audio: &AudioDataWrapper) -> u32;
        #[namespace = "keyfinder_bridge"]
        fn add_to_sample_count(audio: Pin<&mut AudioDataWrapper>, samples: u32);
        #[namespace = "keyfinder_bridge"]
        fn reset_iterators(audio: Pin<&mut AudioDataWrapper>);
        #[namespace = "keyfinder_bridge"]
        fn advance_write_iterator(audio: Pin<&mut AudioDataWrapper>);
        #[namespace = "keyfinder_bridge"]
        fn set_sample_at_write_iterator(audio: Pin<&mut AudioDataWrapper>, sample: f32);
    }
}

// Higher-level Rust API
pub use ffi::KeyFinderKey;

pub struct KeyFinder {
    inner: cxx::UniquePtr<ffi::KeyFinderWrapper>,
}

impl KeyFinder {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_keyfinder(),
        }
    }

    pub fn key_of_audio(&mut self, audio: &AudioData) -> KeyFinderKey {
        ffi::key_of_audio(self.inner.pin_mut(), &audio.inner)
    }
}

impl Default for KeyFinder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AudioData {
    inner: cxx::UniquePtr<ffi::AudioDataWrapper>,
}

impl AudioData {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_audiodata(),
        }
    }

    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        ffi::set_frame_rate(self.inner.pin_mut(), frame_rate);
    }

    pub fn set_channels(&mut self, channels: u32) {
        ffi::set_channels(self.inner.pin_mut(), channels);
    }

    pub fn get_sample_count(&self) -> u32 {
        ffi::get_sample_count(&self.inner)
    }

    pub fn add_to_sample_count(&mut self, samples: u32) {
        ffi::add_to_sample_count(self.inner.pin_mut(), samples);
    }

    pub fn reset_iterators(&mut self) {
        ffi::reset_iterators(self.inner.pin_mut());
    }

    pub fn advance_write_iterator(&mut self) {
        ffi::advance_write_iterator(self.inner.pin_mut());
    }

    pub fn set_sample_at_write_iterator(&mut self, sample: f32) {
        ffi::set_sample_at_write_iterator(self.inner.pin_mut(), sample);
    }

    /// Add samples to the audio data using the iterator pattern
    /// This mimics the C++ pattern used in keyfinder-cli
    pub fn add_samples(&mut self, samples: &[f32]) {
        let old_count = self.get_sample_count();
        self.add_to_sample_count(samples.len() as u32);
        self.reset_iterators();
        self.advance_write_iterator_n(old_count);

        for &sample in samples {
            self.set_sample_at_write_iterator(sample);
            self.advance_write_iterator();
        }
    }

    fn advance_write_iterator_n(&mut self, n: u32) {
        for _ in 0..n {
            self.advance_write_iterator();
        }
    }
}

impl Default for AudioData {
    fn default() -> Self {
        Self::new()
    }
}
