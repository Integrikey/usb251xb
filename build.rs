/// Linker arguments for on-target integration tests (RP2350, embedded-test).
/// These only apply to test targets, not the library itself.
fn main() {
    println!("cargo::rustc-link-arg-tests=-Tlink.x");
    println!("cargo::rustc-link-arg-tests=-Tdefmt.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
