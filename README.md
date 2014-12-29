# Rust HackRF

Rust wrapper for [libhackrf](https://github.com/mossmann/hackrf/tree/master/host)

# Installation
Add it to your `Cargo.toml`:

```toml
[dependencies.hackrf]
git = "git://github.com/code0100fun/rust-hackrf"
```

# Usage

```rust
extern crate hackrf;
use hackrf::HackRF;

HackRF::open(|mut hackrf| {

    hackrf.set_freq(900000000);
    hackrf.set_sample_rate(8000000);
    hackrf.set_baseband_filter_bandwidth(28000000);
    hackrf.set_vga_gain(20);
    hackrf.set_lna_gain(8);

    let (tx, rx) = hackrf.start();
    Thread::spawn(move || {
        loop {
            let mut bytes = rx.recv();
            // ...
        }
    });
});
```
