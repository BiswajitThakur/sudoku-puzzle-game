use base64::prelude::*;

use crate::{
    game::{grid_to_bitmask, hide_answers_inplace},
    utils::Level,
};

// v1_<0:easy|1:medium|2:difficult><0:show|1:hide ans><len+game_data+(mask?):base64>
pub(crate) fn encode_game(table: &[Vec<u8>], mask: &[Vec<bool>], t: Level) -> [String; 2] {
    let e1 = encode_puzzle_only(table, mask, t);
    let e2 = encode_with_solution(table, mask, t);
    [e1, e2]
}

pub(crate) fn encode_with_solution(v: &[Vec<u8>], mask: &[Vec<bool>], game_level: Level) -> String {
    let prefix = match game_level {
        Level::Easy => "v1_00",
        Level::Medium => "v1_10",
        Level::Difficult => "v1_20",
    };
    let mut data = Vec::new();
    let table = encode_table(v);
    data.extend_from_slice(&table);
    let mask = grid_to_bitmask(mask);
    data.extend_from_slice(&mask);
    let data = BASE64_URL_SAFE.encode(data);
    let data = format!("{}{}", prefix, data);
    data
}

pub(crate) fn encode_puzzle_only(v: &[Vec<u8>], mask: &[Vec<bool>], game_level: Level) -> String {
    let prefix = match game_level {
        Level::Easy => "v1_01",
        Level::Medium => "v1_11",
        Level::Difficult => "v1_21",
    };
    let mut v = v.to_vec();
    hide_answers_inplace(&mut v, mask);
    let mut data = Vec::new();
    let table = encode_table(&v);
    data.extend_from_slice(&table);
    let data = BASE64_URL_SAFE.encode(data);
    let data = format!("{}{}", prefix, data);
    data
}

pub fn bitmask_to_grid(n: u8, data: &[u8]) -> Vec<Vec<bool>> {
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
pub(crate) fn encode_table(v: &[Vec<u8>]) -> Vec<u8> {
    let n = v.len() as u8;

    // flatten
    let mut flat = Vec::with_capacity((n as usize) * (n as usize));
    for row in v {
        for &val in row {
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

#[inline]
fn merge(a: u8, b: u8) -> u8 {
    ((a & 0x0F) << 4) | (0x0F & b)
}

#[cfg(test)]
mod tests {
    use crate::{decode::v1::decode_table, encode::v1::encode_table};

    #[test]
    fn test_encode_decode_table_1() {
        let data = vec![vec![1u8; 9]; 9];
        let mut e = encode_table(&data);
        let s = &[5, 6, 7, 8, 9, 10];
        e.extend_from_slice(s);
        let (d, k) = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d);
        assert_eq!(s, k);
    }
    #[test]
    fn test_encode_decode_table_2() {
        let data = vec![vec![0u8; 9]; 9];
        let e = encode_table(&data);
        let (d, _) = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d);
    }
    #[test]
    fn test_encode_decode_table_3() {
        let data = vec![vec![7u8; 15]; 15];
        let e = encode_table(&data);
        let d = decode_table(e.as_slice()).unwrap();
        assert_eq!(data, d.0);
    }
}
