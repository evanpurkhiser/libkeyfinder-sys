use libkeyfinder_sys::{AudioData, KeyFinder, KeyFinderKey};
use std::fs::File;
use std::io::Read;

#[test]
fn test_real_audio_sample() {
    // Load 1 second of "Angel City - 24-7 (Chaos Remix)" at 22050 Hz mono
    let mut file = File::open("tests/sample.pcm").expect("Failed to open sample.pcm");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read sample.pcm");

    // Convert bytes to f32 samples (little-endian)
    let samples: Vec<f32> = buffer
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    // Create audio data
    let mut audio = AudioData::new();
    audio.set_frame_rate(22050);
    audio.set_channels(1);
    audio.extend(samples);

    // Verify we loaded the expected amount of data
    // 1 second * 22050 Hz = 22,050 samples
    assert_eq!(audio.sample_count(), 22050);
    assert_eq!(audio.frame_count(), 22050);

    // Detect the key
    let mut kf = KeyFinder::new();
    let key = kf.key_of_audio(&audio);

    // For this particular sample from the middle of the song, it detects D Minor
    assert_eq!(key, KeyFinderKey::DMinor);
}
