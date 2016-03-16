#![allow(bad_style)]

extern crate libc;

use libc::{c_void, size_t, c_uint, c_ulonglong, c_int, c_uchar, c_longlong};

pub type XXH32_hash_t = c_uint;
pub type XXH64_hash_t = c_ulonglong;
pub type XXH_errorcode = c_int;

pub const XXH_OK: XXH_errorcode = 0;
pub const XXH_ERROR: XXH_errorcode = 1;

pub enum XXH32_state_t {}
pub enum XXH64_state_t {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XXH32_stateBody_t {
    pub ll: [c_longlong; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XXH64_stateBody_t {
    pub ll: [c_longlong; 11],
}

#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [c_uchar; 4],
}

#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [c_uchar; 8],
}

extern {
    #[link_name = "__rust_xxhash_sys_XXH32"]
    pub fn XXH32(input: *const c_void,
                 length: size_t,
                 seed: c_uint) -> XXH32_hash_t;
    #[link_name = "__rust_xxhash_sys_XXH64"]
    pub fn XXH64(input: *const c_void,
                 length: size_t,
                 seed: c_ulonglong) -> XXH64_hash_t;

    #[link_name = "__rust_xxhash_sys_XXH32_createState"]
    pub fn XXH32_createState() -> *mut XXH32_state_t;
    #[link_name = "__rust_xxhash_sys_XXH32_freeState"]
    pub fn XXH32_freeState(ptr: *mut XXH32_state_t) -> XXH_errorcode;
    #[link_name = "__rust_xxhash_sys_XXH64_createState"]
    pub fn XXH64_createState() -> *mut XXH64_state_t;
    #[link_name = "__rust_xxhash_sys_XXH64_freeState"]
    pub fn XXH64_freeState(ptr: *mut XXH64_state_t) -> XXH_errorcode;

    #[link_name = "__rust_xxhash_sys_XXH32_reset"]
    pub fn XXH32_reset(statePtr: *mut XXH32_state_t,
                       seed: c_uint) -> XXH_errorcode;
    #[link_name = "__rust_xxhash_sys_XXH32_update"]
    pub fn XXH32_update(statePtr: *mut XXH32_state_t,
                        input: *const c_void,
                        length: size_t) -> XXH_errorcode;
    #[link_name = "__rust_xxhash_sys_XXH32_digest"]
    pub fn XXH32_digest(statePtr: *const XXH32_state_t) -> XXH32_hash_t;

    #[link_name = "__rust_xxhash_sys_XXH64_reset"]
    pub fn XXH64_reset(statePtr: *mut XXH64_state_t,
                       seed: c_ulonglong) -> XXH_errorcode;
    #[link_name = "__rust_xxhash_sys_XXH64_update"]
    pub fn XXH64_update(statePtr: *mut XXH64_state_t,
                        input: *const c_void,
                        length: size_t) -> XXH_errorcode;
    #[link_name = "__rust_xxhash_sys_XXH64_digest"]
    pub fn XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;

    // apis that aren't namespaced?!

    pub fn XXH32_canonicalFromHash(dst: *mut XXH32_canonical_t,
                                   hash: XXH32_hash_t);
    pub fn XXH64_canonicalFromHash(dst: *mut XXH64_canonical_t,
                                   hash: XXH64_hash_t);

    pub fn XXH32_hashFromCanonical(src: *const XXH32_canonical_t)
                                   -> XXH32_hash_t;
    pub fn XXH64_hashFromCanonical(src: *const XXH64_canonical_t)
                                   -> XXH64_hash_t;
}
