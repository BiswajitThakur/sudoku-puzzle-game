use base64::prelude::*;

use crate::{
    game::bitmask_to_grid,
    utils::{Level, full_mask_from_table},
};

#[allow(clippy::type_complexity)]
pub fn decode_game(input: String) -> Option<(Level, bool, Vec<Vec<u8>>, Vec<Vec<bool>>)> {
    let v = input.strip_prefix("v1_")?;
    let mut iter = v.chars();
    let level = match iter.next()? {
        '0' => Level::Easy,
        '1' => Level::Medium,
        '2' => Level::Difficult,
        _ => return None,
    };
    let is_hidden = match iter.next()? {
        '0' => false,
        '1' => true,
        _ => return None,
    };
    let decoded = BASE64_URL_SAFE.decode(iter.as_str()).ok()?;
    let bytes = decoded.as_slice();
    let (table, rest) = decode_table(bytes)?;
    if !is_hidden && rest.is_empty() {
        return None;
    }
    let mask = if rest.is_empty() {
        full_mask_from_table(&table)
    } else {
        bitmask_to_grid(rest, table.len())
    };
    Some((level, is_hidden, table, mask))
}

pub fn decode_table(s: &[u8]) -> Option<(Vec<Vec<u8>>, &[u8])> {
    let n = s[0] as usize;
    let total = n * n;
    let bytes = s.get(1..total.div_ceil(2) + 1)?;
    let rest = s.get(1 + total.div_ceil(2)..).unwrap_or(&[]);
    let mut flat = Vec::with_capacity(total);

    for &b in bytes {
        let (a, c) = split(b);
        flat.push(a);
        flat.push(c);
    }

    flat.truncate(total);
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let start = i * n;
        let end = start + n;
        result.push(flat[start..end].to_vec());
    }

    Some((result, rest))
}

#[inline]
fn split(x: u8) -> (u8, u8) {
    let a = x >> 4;
    let b = x & 0x0F;
    (a, b)
}
