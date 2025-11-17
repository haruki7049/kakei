fn main() {
    // Rerun the build script if any locale files change
    println!("cargo:rerun-if-changed=locales/");
}
