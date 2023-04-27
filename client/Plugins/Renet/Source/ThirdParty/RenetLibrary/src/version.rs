/// Generate a new Version from the current crate's version.
#[macro_export]
macro_rules! version(
    () => (
        $crate::version::__str_hash(env!("CARGO_PKG_VERSION"))   
    )
);

#[doc(hidden)]
pub const fn __str_hash(s: &str) -> u64 {
    // Based on Jenkins' one-at-a-time hash.
    let mut result: u64 = 0;
    let s = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        result = result.wrapping_add(s[i] as u64);
        result = result.wrapping_add(result << 10);
        result ^= result >> 6;
        i += 1;
    }

    result = result.wrapping_add(result << 3);
    result ^= result >> 11;
    result = result.wrapping_add(result << 15);

    result
}
