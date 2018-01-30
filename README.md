A helper library for [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html) to work with NDK to build native libraries targeting android.

# Usage
Cargo.toml
```toml
[build-dependencies]
ndk-rs = { git = "https://github.com/jarod/ndk-rs.git" }
```

Autotools configure script:
```rust
extern crate ndk;

use ndk::Toolchain;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut configure = Command::new("sh");
    configure.args(&[
        "configure",
        "--host",
        &target,
    ]);
    // set CC and sysroot for android targets
    if target.contains("android") {
        let toolchain = Toolchain::new(14, &target).unwrap();
        configure
            .env("CC", toolchian.cc())
            .arg("--with-sysroot")
            .arg(&toolchian.sysroot());
    }
    configure.status().expect("configure");
}
```

Set ANDROID_NDK then build
```bash
export ANDROID_NDK=<path to ndk directory>
cargo build
```