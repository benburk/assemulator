/// Masks a signed integer into a field of a given width.
///
/// # Examples
///
/// ```rust
/// use assemulator::mask;
/// assert_eq!(mask(2, 2), Ok(2));
/// assert_eq!(mask(0xff, 8), Ok(0xff));
/// assert_eq!(mask(u64::MAX, 4), Ok(0b1111));
/// ```
///  
/// # Errors
///
/// If `value` can't fit inside `n_bits`.
///
pub fn mask<T>(value: u64, n_bits: usize) -> Result<T, String>
where
    T: TryFrom<u64> + Copy,
{
    // Compute up to 2^64 - 1 without overflow
    let mask: u64 = ((1 << (n_bits - 1)) - 1) * 2 + 1;
    if ((value >> 63_i32) == 1 && ((!value).wrapping_add(1) > mask))
        || ((value >> 63_i32) == 0 && value > mask)
    {
        return Err(format!("Value {value} does not fit in {n_bits} bits"));
    }
    T::try_from(value & mask).map_err(|_err| "Invalid mask".to_owned())
}
