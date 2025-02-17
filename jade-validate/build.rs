fn main() {
    println!("cargo:rerun-if-changed=perfect6502/perfect6502.c");
    println!("cargo:rerun-if-changed=perfect6502/perfect6502.h");

    cc::Build::new()
        .files(["perfect6502/perfect6502.c", "perfect6502/netlist_sim.c"])
        .include("perfect6502")
        .compile("perfect6502");

    bindgen::Builder::default()
        .rust_target("1.84.1".parse().expect("version is valid"))
        .header("perfect6502/perfect6502.h")
        .generate()
        .expect("bindings should be able to get generated")
        .write_to_file("src/emulators/perfect6502/bindings.rs")
        .expect("bindings should be able to get written to a file");
}
