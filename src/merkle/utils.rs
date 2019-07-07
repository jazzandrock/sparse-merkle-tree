#[allow(dead_code)]
pub fn log2_64(mut value: u64) -> usize {
    static TAB64: [usize; 64] = [
        0 , 58, 1 , 59, 47, 53, 2 , 60, 
        39, 48, 27, 54, 33, 42, 3 , 61,
        51, 37, 40, 49, 18, 28, 20, 55, 
        30, 34, 11, 43, 14, 22, 4 , 62,
        57, 46, 52, 38, 26, 32, 41, 50, 
        36, 17, 19, 29, 10, 13, 21, 56,
        45, 25, 31, 35, 16, 9 , 12, 44, 
        24, 15, 8 , 23, 7 , 6 , 5 , 63 
    ];

    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value |= value >> 32;
    // the most magic of all the constants
    value = value.wrapping_mul(0x03f6eaf2cd271461u64);
    value >>= 58;
    return TAB64[value as usize];
}
