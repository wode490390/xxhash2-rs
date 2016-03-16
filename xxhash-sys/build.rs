extern crate cmake;

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    cmake::Config::new("xxHash/cmake_unofficial")
          .cflag("-DXXH_NAMESPACE=__rust_xxhash_sys_")
          .build();
    println!("cargo:rustc-link-lib=static=xxhash");
    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:include=native={}/include", out_dir);
}
