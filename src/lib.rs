//! Bindings to libxxhash, an xxHash implementation.
//!
//! This library contains safe Rust bindings to the libxxhash library which
//! contains an implementation of the xxHash algorithm. There are Rust
//! implementations available of this hash algorithm, but it is intended that
//! this library is a binding to the official C source.
//!
//! The xxHash algorith comes in 32 and 64 bit variants. reflected in the
//! exported types and functions of this crate.
//!
//! # Examples
//!
//! ```
//! assert_eq!(xxhash2::hash32(&[1, 2, 3, 4], 0), 4271296924);
//! assert_eq!(xxhash2::hash64(&[1, 2, 3, 4], 0), 6063570110359613137);
//!
//! let mut s = xxhash2::State32::new();
//! s.reset(0);
//! s.update(&[1, 2, 3, 4]);
//! assert_eq!(s.finish(), 4271296924);
//!
//! let mut s = xxhash2::State64::new();
//! s.reset(0);
//! s.update(&[1, 2, 3, 4]);
//! assert_eq!(s.finish(), 6063570110359613137);
//! ```

extern crate libc;
extern crate xxhash_sys;

use std::fmt;
use std::hash::Hasher;
use std::mem;

use libc::{size_t, c_void};

/// Representation of the intermediate state of a 32-bit xxHash instance.
///
/// This structure can be used to generate a 32-bit hash of a block of bytes.
pub struct State32 {
    inner: xxhash_sys::XXH32_stateBody_t,
}

/// Representation of the intermediate state of a 64-bit xxHash instance.
///
/// This structure can be used to generate a 64-bit hash of a block of bytes.
pub struct State64 {
    inner: xxhash_sys::XXH64_stateBody_t,
}

/// Canonical representation of a 32-bit hash.
///
/// This structure provides a conversion from a 32-bit hash value to a list of
/// bytes in a canonical format. Note that the bytes returned are not
/// necessarily hex.
pub struct Hash32 {
    inner: xxhash_sys::XXH32_canonical_t,
}

/// Canonical representation of a 64-bit hash.
///
/// This structure provides a conversion from a 64-bit hash value to a list of
/// bytes in a canonical format. Note that the bytes returned are not
/// necessarily hex.
pub struct Hash64 {
    inner: xxhash_sys::XXH64_canonical_t,
}

/// Hash a block of bytes with a given seed, returning the 32-bit hash.
///
/// This function, optimized for speed, will hash an entire block all at once
/// and return the hash value.
///
/// # Examples
///
/// ```
/// assert_eq!(xxhash2::hash32(&[1, 2, 3, 4], 0), 4271296924);
/// ```
pub fn hash32(data: &[u8], seed: u32) -> u32 {
    unsafe {
        xxhash_sys::XXH32(data.as_ptr() as *const c_void,
                          data.len() as size_t,
                          seed) as u32
    }
}

/// Hash a block of bytes with a given seed, returning the 64-bit hash.
///
/// This function, optimized for speed, will hash an entire block all at once
/// and return the hash value.
///
/// # Examples
///
/// ```
/// assert_eq!(xxhash2::hash64(&[1, 2, 3, 4], 0), 6063570110359613137);
/// ```
pub fn hash64(data: &[u8], seed: u64) -> u64 {
    unsafe {
        xxhash_sys::XXH64(data.as_ptr() as *const c_void,
                          data.len() as size_t,
                          seed) as u64
    }
}

impl State32 {
    /// Creates a new blank instance of the 32-bit xxHash state.
    ///
    /// This state can then be used to hash a list of bytes and acquire the
    /// result via the `finish` method.
    ///
    /// # Examples
    ///
    /// ```
    /// let state = xxhash2::State32::new();
    /// ```
    pub fn new() -> State32 {
        unsafe {
            State32 { inner: mem::zeroed() }
        }
    }

    /// Input a block of bytes into this hasher.
    ///
    /// This function will update the internal state of this hasher with a new
    /// list of bytes to feed in. To get the hash value of the block (and all
    /// previous bytes), call the `finish` function.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State32::new();
    /// state.update(&[1, 2, 3, 4]);
    /// ```
    pub fn update(&mut self, data: &[u8]) {
        let r = unsafe {
            xxhash_sys::XXH32_update(self.inner(),
                                     data.as_ptr() as *const c_void,
                                     data.len() as size_t)
        };
        assert_eq!(r, xxhash_sys::XXH_OK);
    }

    /// Reset the internal state of this hasher with a given seed.
    ///
    /// This is useful to reuse a hasher or to persist the state across
    /// multiple runs of a hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State32::new();
    /// state.update(&[1, 2, 3, 4]);
    /// ```
    pub fn reset(&mut self, seed: u32) {
        let r = unsafe {
            xxhash_sys::XXH32_reset(self.inner(), seed)
        };
        assert_eq!(r, xxhash_sys::XXH_OK);
    }

    /// Computes the final hash result of all bytes that have been input to
    /// this hasher.
    ///
    /// Returns the 32-bit checksum of the result.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State32::new();
    /// state.update(&[1, 2, 3, 4]);
    /// assert_eq!(state.finish(), 4271296924);
    /// ```
    pub fn finish(&self) -> u32 {
        unsafe {
            xxhash_sys::XXH32_digest(&self.inner as *const _ as *const _)
        }
    }

    fn inner(&mut self) -> *mut xxhash_sys::XXH32_state_t {
        &mut self.inner as *mut _ as *mut _
    }
}

impl Hasher for State32 {
    fn write(&mut self, data: &[u8]) {
        self.update(data)
    }

    fn finish(&self) -> u64 {
        self.finish() as u64
    }
}

impl Clone for State32 {
    fn clone(&self) -> State32 {
        State32 { inner: self.inner }
    }
}

impl Default for State32 {
    fn default() -> State32 {
        State32::new()
    }
}

impl State64 {
    /// Creates a new blank instance of the 64-bit xxHash state.
    ///
    /// This state can then be used to hash a list of bytes and acquire the
    /// result via the `finish` method.
    ///
    /// # Examples
    ///
    /// ```
    /// let state = xxhash2::State64::new();
    /// ```
    pub fn new() -> State64 {
        unsafe {
            State64 { inner: mem::zeroed() }
        }
    }

    /// Input a block of bytes into this hasher.
    ///
    /// This function will update the internal state of this hasher with a new
    /// list of bytes to feed in. To get the hash value of the block (and all
    /// previous bytes), call the `finish` function.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State64::new();
    /// state.update(&[1, 2, 3, 4]);
    /// ```
    pub fn update(&mut self, data: &[u8]) {
        let r = unsafe {
            xxhash_sys::XXH64_update(self.inner(),
                                     data.as_ptr() as *const c_void,
                                     data.len() as size_t)
        };
        assert_eq!(r, xxhash_sys::XXH_OK);
    }

    /// Reset the internal state of this hasher with a given seed.
    ///
    /// This is useful to reuse a hasher or to persist the state across
    /// multiple runs of a hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State64::new();
    /// state.update(&[1, 2, 3, 4]);
    /// ```
    pub fn reset(&mut self, seed: u64) {
        let r = unsafe {
            xxhash_sys::XXH64_reset(self.inner(), seed)
        };
        assert_eq!(r, xxhash_sys::XXH_OK);
    }

    /// Computes the final hash result of all bytes that have been input to
    /// this hasher.
    ///
    /// Returns the 64-bit checksum of the result.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = xxhash2::State64::new();
    /// state.update(&[1, 2, 3, 4]);
    /// assert_eq!(state.finish(), 6063570110359613137);
    /// ```
    pub fn finish(&self) -> u64 {
        unsafe {
            xxhash_sys::XXH64_digest(&self.inner as *const _ as *const _)
        }
    }

    fn inner(&mut self) -> *mut xxhash_sys::XXH64_state_t {
        &mut self.inner as *mut _ as *mut _
    }
}

impl Hasher for State64 {
    fn write(&mut self, data: &[u8]) {
        self.update(data)
    }

    fn finish(&self) -> u64 {
        self.finish()
    }
}

impl Clone for State64 {
    fn clone(&self) -> State64 {
        State64 { inner: self.inner }
    }
}

impl Default for State64 {
    fn default() -> State64 {
        State64::new()
    }
}

impl Hash32 {
    /// Returns the underlying hash as a list of bytes in a canonical
    /// representation.
    pub fn bytes(&self) -> &[u8; 4] {
        &self.inner.digest
    }

    /// Returns this canonical hash value as a 32-bit integer.
    ///
    /// Converts from the underlying list of bytes to an integer.
    pub fn value(&self) -> u32 {
        unsafe {
            xxhash_sys::XXH32_hashFromCanonical(&self.inner)
        }
    }
}

impl From<u32> for Hash32 {
    /// Creates a new canonical representation of a 32-bit hash value.
    ///
    /// The returned value can be viewed as a list of bytes.
    fn from(val: u32) -> Hash32 {
        unsafe {
            let mut ret = Hash32 { inner: mem::zeroed() };
            xxhash_sys::XXH32_canonicalFromHash(&mut ret.inner, val);
            return ret
        }
    }
}

impl From<[u8; 4]> for Hash32 {
    /// Creates a new canonical representation of a hash from the 4-byte
    /// canonical representation.
    ///
    /// The returned value can be viewed as a `u32`.
    fn from(val: [u8; 4]) -> Hash32 {
        Hash32 { inner: xxhash_sys::XXH32_canonical_t { digest: val } }
    }
}

impl fmt::Debug for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.digest.fmt(f)
    }
}

impl Hash64 {
    /// Returns the underlying hash as a list of bytes in a canonical
    /// representation.
    pub fn bytes(&self) -> &[u8; 8] {
        &self.inner.digest
    }

    /// Returns this canonical hash value as a 64-bit integer.
    ///
    /// Converts from the underlying list of bytes to an integer.
    pub fn value(&self) -> u64 {
        unsafe {
            xxhash_sys::XXH64_hashFromCanonical(&self.inner)
        }
    }
}

impl From<u64> for Hash64 {
    /// Creates a new canonical representation of a 64-bit hash value.
    ///
    /// The returned value can be viewed as a list of bytes.
    fn from(val: u64) -> Hash64 {
        unsafe {
            let mut ret = Hash64 { inner: mem::zeroed() };
            xxhash_sys::XXH64_canonicalFromHash(&mut ret.inner, val);
            return ret
        }
    }
}

impl From<[u8; 8]> for Hash64 {
    /// Creates a new canonical representation of a hash from the 8-byte
    /// canonical representation.
    ///
    /// The returned value can be viewed as a `u64`.
    fn from(val: [u8; 8]) -> Hash64 {
        Hash64 { inner: xxhash_sys::XXH64_canonical_t { digest: val } }
    }
}

impl fmt::Debug for Hash64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.digest.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test32(data: &[u8], seed: u32, expected: u32) {
        assert_eq!(hash32(data, seed), expected);
        let mut state = State32::new();
        state.reset(seed);
        state.update(data);
        assert_eq!(state.finish(), expected);
        assert_eq!(state.finish(), expected);

        state.reset(seed);
        for i in 0..data.len() {
            state.update(&data[i..i+1]);
        }
        assert_eq!(state.finish(), expected);
    }

    fn test64(data: &[u8], seed: u64, expected: u64) {
        assert_eq!(hash64(data, seed), expected);
        let mut state = State64::new();
        state.reset(seed);
        state.update(data);
        assert_eq!(state.finish(), expected);
        assert_eq!(state.finish(), expected);

        state.reset(seed);
        for i in 0..data.len() {
            state.update(&data[i..i+1]);
        }
        assert_eq!(state.finish(), expected);
    }

    #[test]
    fn smoke() {
        test32(&[], 0, 46947589);
        test64(&[], 0, 17241709254077376921);
        test32(&[1], 0, 949155633);
        test64(&[1], 0, 9962287286179718960);
        test32(&[1, 2, 3, 4], 0, 4271296924);
        test64(&[1, 2, 3, 4], 0, 6063570110359613137);
    }

    #[test]
    fn hash() {
        assert_eq!(Hash32::from(0).bytes(), &[0, 0, 0, 0]);
        assert_eq!(Hash32::from(0).value(), 0);
        assert_eq!(Hash32::from(0x12345678).bytes(), &[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(Hash32::from(0x12345678).value(), 0x12345678);
        assert_eq!(Hash64::from(0).bytes(), &[0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(Hash64::from(0).value(), 0);
        assert_eq!(Hash64::from(0x12345678).bytes(),
                   &[0, 0, 0, 0, 0x12, 0x34, 0x56, 0x78]);
        assert_eq!(Hash64::from(0x12345678).value(), 0x12345678);
    }
}
