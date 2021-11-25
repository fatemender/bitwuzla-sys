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

        if !self.out_dir.join("deps/install/lib/libcadical.a").exists() {
            self.run_command(
                "Download and build CaDiCaL",
                Command::new("/usr/bin/env")
                    .arg("bash")
                    .arg(self.out_dir.join("contrib/setup-cadical.sh"))
                    .current_dir(&self.out_dir),
            );
        }

        if !self.out_dir.join("deps/install/lib/libbtor2parser.a").exists() {
            self.run_command(
                "Download and build BTOR2Tools",
                Command::new("/usr/bin/env")
                    .arg("bash")
                    .arg(self.out_dir.join("contrib/setup-btor2tools.sh"))
                    .current_dir(&self.out_dir),
            );
        }

        if !self.out_dir.join("deps/symfpu").exists() {
            self.run_command(
                "Download and build SymFPU",
                Command::new("/usr/bin/env")
                    .arg("bash")
                    .arg(self.out_dir.join("contrib/setup-symfpu.sh"))
                    .current_dir(&self.out_dir),
            );
        }

        println!("cargo:rustc-link-search=native={}", self.out_dir.join("deps/install/lib").display());
        println!("cargo:rustc-link-lib=static=cadical");
        println!("cargo:rustc-link-lib=static=btor2parser");

        self
    }

    pub fn build(self) -> Self {
        self.run_command(
            "Configure Bitwuzla",
            Command::new("/bin/sh")
                .arg(self.out_dir.join("configure.sh"))
                .current_dir(&self.out_dir),
        );

        self.run_command(
            "Build Bitwuzla",
            Command::new("make")
                .arg("-j")
                .current_dir(self.out_dir.join("build")),
        );

        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-search=native={}", self.out_dir.join("build/lib").display());

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
