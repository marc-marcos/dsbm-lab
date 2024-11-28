fn main() {
    cc::Build::new().file("src/driver.c").compile("driver.o");

    println!("cargo:rustc-link-lib=wiringPi");
    println!("cargo:rerun-if-changed=src");
}
