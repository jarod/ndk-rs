use std::env;
use std::env::consts::{ARCH, OS};
use std::path::PathBuf;

pub struct Toolchain {
    ndk: PathBuf,
    toolchain: String,
    api: u8,
    // target: String,
    toolchain_host: String,
    platform_arch: String,
}

impl Toolchain {
    pub fn new(api: u8, target: &str) -> Result<Self, String> {
        let root_dir = env::var("ANDROID_NDK")
            .expect("ANDROID_NDK is not set")
            .into();
        Self::with_ndk(root_dir, api, target)
    }

    pub fn with_ndk(ndk_dir: PathBuf, api: u8, target: &str) -> Result<Self, String> {
        let os = match OS {
            "macos" => "darwin",
            os => os,
        };
        let platform_arch = Self::platform_arch(target, api)?;
        Ok(Toolchain {
            ndk: ndk_dir.into(),
            toolchain: "llvm".to_owned(),
            api: api,
            // target: target.to_owned(),
            toolchain_host: format!("{}-{}", os, ARCH),
            platform_arch: platform_arch.to_owned(),
        })
    }

    pub fn cc(&self) -> String {
        format!(
            "{}/toolchains/{}/prebuilt/{}/bin/clang",
            self.ndk.to_str().unwrap(),
            self.toolchain,
            self.toolchain_host,
        )
    }
    pub fn cxx(&self) -> String {
        format!(
            "{}/toolchains/{}/prebuilt/{}/bin/clang++",
            self.ndk.to_str().unwrap(),
            self.toolchain,
            self.toolchain_host,
        )
    }

    fn platform_arch(target: &str, api: u8) -> Result<&str, String> {
        let target_arch = target
            .split("-")
            .nth(0)
            .expect(&format!("invalid target triple {}", target));
        let arch = match target_arch {
            "arm" | "armv7" => "arm",
            "aarch64" => if api < 21 {
                "arm"
            } else {
                "arm64"
            },
            "i686" => "x86",
            "x86_64" if api < 21 => "x86",
            "mips64" if api < 21 => "mips",
            arch => arch,
        };
        if api < 9 && arch != "arm" {
            return Err(format!("Not support target {} for API {}", target, api));
        }
        Ok(arch)
    }

    pub fn sysroot(&self) -> String {
        format!(
            "{}/platforms/android-{}/arch-{}",
            self.ndk.to_str().unwrap(),
            self.api,
            self.platform_arch,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_arch() {
        let arch = Toolchain::platform_arch("i686-linux-android", 21);
        assert!(arch.is_ok());
        assert_eq!("x86", arch.unwrap());

        let arch = Toolchain::platform_arch("x86_64-linux-android", 21);
        assert!(arch.is_ok());
        assert_eq!("x86_64", arch.unwrap());

        let arch = Toolchain::platform_arch("aarch64-linux-android", 21);
        assert!(arch.is_ok());
        assert_eq!("arm64", arch.unwrap());

        let arch = Toolchain::platform_arch("arm-linux-androideabi", 21);
        assert!(arch.is_ok());
        assert_eq!("arm", arch.unwrap());

        let arch = Toolchain::platform_arch("armv7-linux-androideabi", 21);
        assert!(arch.is_ok());
        assert_eq!("arm", arch.unwrap());
    }

    #[test]
    fn platform_arch_not_support_for_low_api_level() {
        assert!(Toolchain::platform_arch("i686-linux-android", 8).is_err());
        assert!(Toolchain::platform_arch("x86_64-linux-android", 8).is_err());
    }

    #[test]
    fn platform_arch_downgrade_64_to_32bit_for_low_api_level() {
        let arch = Toolchain::platform_arch("x86_64-linux-android", 9);
        assert!(arch.is_ok());
        assert_eq!("x86", arch.unwrap());

        let arch = Toolchain::platform_arch("aarch64-linux-android", 9);
        assert!(arch.is_ok());
        assert_eq!("arm", arch.unwrap());
    }
}
