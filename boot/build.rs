pub fn main() {
    println!("cargo:rerun-if-changed=.cargo/linker.ld");
}