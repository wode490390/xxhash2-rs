#![allow(bad_style)]

extern crate xxhash_sys;
extern crate libc;

use libc::*;
use xxhash_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
