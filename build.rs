fn main() {
    println!("cargo:rustc-link-lib=dylib=vl");
    if let Ok(var) = std::env::var("VLFEAT_BIN") {
        println!("cargo:rustc-link-search={}", var);
    }
}
