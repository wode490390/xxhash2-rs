extern crate ctest;

use std::env;

fn main() {
    let out = env::var("DEP_XXHASH_INCLUDE").unwrap();
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("xxhash.h");
    cfg.define("XXH_NAMESPACE", Some("__rust_xxhash_sys_"));
    cfg.include(&out);
    cfg.type_name(|s, _| s.to_string());
    cfg.fn_cname(|s, l| l.unwrap_or(s).to_string());
    cfg.skip_type(|n| n == "__enum_ty");
    cfg.generate("../xxhash-sys/src/lib.rs", "all.rs");
}
