use cc;
use glob::glob;

const PICO_SRC_DIR: &str = "pico/lib";
const PICO_LIB_NAME: &str = "svoxpico";

fn main() {
    let glob_pat = format!("{}/*.c", PICO_SRC_DIR);
    let src_files = glob(glob_pat.as_str())
        .expect("Failed to list *.c files")
        .map(|p| p.unwrap());

    cc::Build::new()
        .include(PICO_SRC_DIR)
        .files(src_files)
        .compile(PICO_LIB_NAME); // (static library)

    println!("cargo:rustc-link-lib={}", PICO_LIB_NAME);
}