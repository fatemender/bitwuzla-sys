#[cfg(feature = "vendor-cadical")]
include!("build-vendor.rs");

fn main() {
    #[cfg(feature = "vendor-cadical")]
    {
        if std::env::var("BITWUZLA_NO_VENDOR").map_or(true, |value| value == "0") {
            BitwuzlaBuild::new().prerequisites().build();
        }
    }

    println!("cargo:rustc-link-lib=bitwuzla");
}
