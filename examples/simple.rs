use libkeyfinder_sys::{AudioData, KeyFinder, KeyFinderKey};

fn main() {
    // Create a KeyFinder instance
    let mut kf = KeyFinder::new();

    // Create some audio data
    let mut audio = AudioData::new();
    audio.set_frame_rate(44100);
    audio.set_channels(2);

    // Add some dummy audio samples (silence)
    let samples = vec![0.0f32; 44100 * 10]; // 10 seconds of silence
    audio.add_samples(&samples);

    // Detect the key
    let key = kf.key_of_audio(&audio);

    println!("Detected key: {:?}", key);

    match key {
        KeyFinderKey::Silence => println!("The audio is silent"),
        _ => println!("Key detected: {:?}", key),
    }
}
