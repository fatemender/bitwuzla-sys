use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use copy_dir::copy_dir;

pub struct BitwuzlaBuild {
    src_dir: PathBuf,
    out_dir: PathBuf,
}

impl BitwuzlaBuild {
    pub fn new() -> Self {
        Self {
            src_dir: Path::new(env!("CARGO_MANIFEST_DIR")).join("bitwuzla"),
            out_dir: Path::new(&env::var_os("OUT_DIR").expect("`OUT_DIR` not set")).join("vendor-build"),
        }
    }

    pub fn prerequisites(self) -> Self {
        if !self.out_dir.exists() {
            copy_dir(&self.src_dir, &self.out_dir).expect("failed to copy Bitwuzla sources to `OUT_DIR`");
        }

        self
    }

    pub fn build(self) -> Self {
        const GMP_VERSION: &str = "6.1";
        const GMP_LIB: &str = "gmp";
        // Ensure GMP is available on the system, _before_ configure.py. Otherwise we'll
        // get a confusing error message. However, don't emit the link commands here,
        // because we want to link GMP later.
        pkg_config::Config::new()
            .cargo_metadata(false)
            .atleast_version(GMP_VERSION)
            .probe(GMP_LIB).unwrap();

        self.run_command(
            "Configure Bitwuzla",
            Command::new("/usr/bin/env")
                .arg("python3")
                .arg(self.out_dir.join("configure.py"))
                .current_dir(&self.out_dir),
        );

        self.run_command(
            "Build Bitwuzla",
            Command::new("meson")
                .arg("compile")
                .current_dir(self.out_dir.join("build")),
        );

        println!("cargo:rustc-link-search=native={}", self.out_dir.join("build/src").display());
        println!("cargo:rustc-link-search=native={}", self.out_dir.join("build/src/lib").display());

        println!("cargo:rustc-link-lib=static=bitwuzla");
        println!("cargo:rustc-link-lib=static=bitwuzlabb");
        println!("cargo:rustc-link-lib=static=bitwuzlals");
        println!("cargo:rustc-link-lib=static=bitwuzlabv");
        // Link to pkg-config GMP
        pkg_config::Config::new()
            .atleast_version(GMP_VERSION)
            .probe(GMP_LIB).unwrap();


        // Handle C++ standard library selection, following policy from `cc` crate.
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        match &target_os[..] {
            "macos" | "freebsd" | "openbsd" | "netbsd" => {
                println!("cargo:rustc-link-lib=c++");
            }
            "android" => {
                println!("cargo:rustc-link-lib=c++_shared");
            }
            _ => {
                println!("cargo:rustc-link-lib=stdc++");
            }
        };

        self
    }

    fn run_command(&self, description: &str, command: &mut Command) {
        println!("*** {}", description);

        let status = command.status().unwrap();

        if !status.success() {
            panic!(
                "*** ERROR in action `{}`, exit status {}\n*** Command: {:?}",
                description,
                status,
                command,
            );
        }
    }
}
