
extern crate cc;

fn main() {

    let boot_path = "boot.S";

    println!("cargo:rerun-if-changed={}", boot_path);

    cc::Build::new()
        .file(boot_path)
        .compile("boot");

}
