extern crate cmake;

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut cfg = cmake::Config::new("xxHash/cmake_unofficial");
    cfg.cflag("-DXXH_NAMESPACE=__rust_xxhash_sys_");
    cfg.define("BUILD_SHARED_LIBS", "OFF");
    cfg.build();
    println!("cargo:rustc-link-lib=static=xxhash");
    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:include={}/include", out_dir);
}
