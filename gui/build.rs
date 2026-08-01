fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib64");
    println!("cargo:rustc-link-lib=acercontrol");
}
