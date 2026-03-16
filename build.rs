fn main() {
    println!("cargo::rustc-link-arg-tests=-Tlink.x");
    println!("cargo::rustc-link-arg-tests=-Tdefmt.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
