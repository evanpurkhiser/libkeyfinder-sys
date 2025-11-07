# libkeyfinder-sys

Rust bindings for [libkeyfinder](https://github.com/mixxxdj/libkeyfinder) - musical key detection for audio.

```rust
use libkeyfinder_sys::{KeyFinder, AudioData};

let mut audio = AudioData::new();
audio.set_frame_rate(44100);
audio.set_channels(2);
audio.extend(samples);

let mut kf = KeyFinder::new();
let key = kf.key_of_audio(&audio);
```

See [documentation](https://docs.rs/libkeyfinder-sys) for details.

## License

GPL v3 or later.
