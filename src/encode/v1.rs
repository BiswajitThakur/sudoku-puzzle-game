use base64::prelude::*;

use crate::utils::GemeType;

// v1_<0:easy|1:medium|2:difficult><0:show|1:hide ans>_<u8:game len><game_data:base64><ans:base64>
pub fn encode_game_keep_answer(data: Vec<Vec<u8>>) -> String {
    todo!()
}
pub fn encode_game_without_answer() -> String {
    todo!()
}

pub fn generate_remove_indices(n: u8, level: GemeType) -> Vec<u8> {
    assert!(n < 16 && n > 0);
    let n = n as usize;
    let total_cells = n * n;

    let size = (total_cells + 7) / 8;
    let mut array: Vec<u8> = vec![0xFF; size];

    let remove_percent = match level {
        GemeType::Easy => 35,
        GemeType::Medium => 50,
        GemeType::Difficult => 80,
    };
    let max_remove = (total_cells * remove_percent) / 100;
    let mut total_remove_count = 0;
    while total_remove_count <= max_remove {
        let row = rand::random_range(0..n);
        let col = rand::random_range(0..n);
        let bit_num = (row * n) + col;
        let index = (bit_num + 7) / 8;
        let bit = bit_num % 8;
        let mask = 1 << bit;
        if (array[index] & mask) != 0 {
            array[index] &= !mask;
            total_remove_count += 1;
        }
    }
    array
}
pub fn bitmask_to_bool_grid(n: u8, data: &[u8]) -> Vec<Vec<bool>> {
    let n = n as usize;
    let mut grid = vec![vec![false; n]; n];

    for i in 0..(n * n) {
        let byte_index = i / 8;
        let bit = i % 8;

        let mask = 1 << bit;

        if (data[byte_index] & mask) != 0 {
            grid[i / n][i % n] = true;
        }
    }

    grid
}

/// Makesure inner len and outer len is equal and less 16
/// and each value must be 0 to 15
pub(crate) fn encode_table(v: Vec<Vec<u8>>) -> Vec<u8> {
    let n = v.len() as u8;

    // flatten
    let mut flat = Vec::with_capacity((n as usize) * (n as usize));
    for row in v {
        for val in row {
            debug_assert!(val <= 0x0F); // ensure 4-bit
            flat.push(val);
        }
    }

    // padding if odd length
    if flat.len() % 2 != 0 {
        flat.push(0);
    }

    let mut compressed = Vec::with_capacity(flat.len() / 2 + 1);

    // store size
    compressed.push(n);

    // pack 2 values into 1 byte
    for chunk in flat.chunks_exact(2) {
        compressed.push(merge(chunk[0], chunk[1]));
    }

    //BASE64_STANDARD.encode(compressed)
    compressed
}

pub fn decode_table(s: &[u8]) -> Option<Vec<Vec<u8>>> {
    //let bytes = BASE64_STANDARD.decode(s).ok()?;

    //let n = bytes[0] as usize;
    let n = s[0] as usize;
    let total = n * n;
    let bytes = s;

    let mut flat = Vec::with_capacity(total);

    for &b in &bytes[1..] {
        let (a, c) = split(b);
        flat.push(a);
        flat.push(c);
    }

    flat.truncate(total);
    // rebuild matrix
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let start = i * n;
        let end = start + n;
        result.push(flat[start..end].to_vec());
    }

    Some(result)
}

fn merge(a: u8, b: u8) -> u8 {
    ((a & 0x0F) << 4) | (0x0F & b)
}

fn split(x: u8) -> (u8, u8) {
    let a = x >> 4;
    let b = x & 0x0F;
    (a, b)
}

#[cfg(test)]
mod tests {
    use crate::{
        encode::v1::{decode_table, encode_table},
        encode_without_answer,
        game::create_game,
        utils::GemeType,
    };

    #[test]
    fn test_encode_decode_table_1() {
        let data = vec![vec![1u8; 9]; 9];
        let e = encode_table(data.clone());
        let d = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d);
    }
    #[test]
    fn test_encode_decode_table_2() {
        let data = vec![vec![0u8; 9]; 9];
        let e = encode_table(data.clone());
        let d = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d);
    }
    #[test]
    fn test_encode_decode_table_3() {
        let data = vec![vec![7u8; 15]; 15];
        let e = encode_table(data.clone());
        let d = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d);
    }
}
