use crate::utils::GemeType;

struct Sudoku {
    data: Vec<Vec<u8>>,
    mask: Vec<Vec<bool>>,
    t: GemeType,
}

impl Sudoku {
    fn new(n: u8, t: GemeType, max_try: usize) -> Option<Self> {
        let data = create_game(n, max_try)?;
        let mask = hide_answer_mask(n as usize, t);
        Some(Self { data, mask, t })
    }
}

fn random_select(vec: &[u8]) -> Option<u8> {
    if vec.is_empty() {
        return None;
    }
    let index = rand::random_range(0..vec.len());
    Some(vec[index])
}

struct ColumnState {
    used_mask: u16,
    available: Vec<u8>,
}

impl ColumnState {
    fn new(size: u8) -> Self {
        Self {
            used_mask: 0,
            available: (1..=size).collect(),
        }
    }

    fn clear(&mut self, size: u8) {
        self.used_mask = 0;
        self.available.clear();
        self.available.extend(1..=size);
    }

    fn next(&mut self, row_mask: &mut u16) -> Option<u8> {
        let mut select_from = Vec::with_capacity(self.available.len());

        for &val in &self.available {
            let bit = 1 << val;

            // already used in row
            if (*row_mask & bit) != 0 {
                continue;
            }

            // already used in column
            if (self.used_mask & bit) != 0 {
                continue;
            }

            select_from.push(val);
        }

        let selected = random_select(&select_from)?;

        let bit = 1 << selected;

        // mark used
        self.used_mask |= bit;
        *row_mask |= bit;

        // remove from available (O(1))
        if let Some(pos) = self.available.iter().position(|&x| x == selected) {
            self.available.swap_remove(pos);
        }

        Some(selected)
    }
}

struct SudokuGenerator {
    columns: Vec<ColumnState>,
    row_index: usize,
    size: u8,
}

impl SudokuGenerator {
    fn new(n: u8) -> Self {
        Self {
            columns: (0..n).map(|_| ColumnState::new(n)).collect(),
            row_index: 0,
            size: n,
        }
    }

    fn clear(&mut self) {
        self.row_index = 0;
        for col in &mut self.columns {
            col.clear(self.size);
        }
    }
}

impl Iterator for SudokuGenerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index >= self.columns.len() {
            return None;
        }

        let mut row = Vec::with_capacity(self.columns.len());
        let mut row_mask: u16 = 0;

        for col in &mut self.columns {
            let v = col.next(&mut row_mask)?;
            row.push(v);
        }

        self.row_index += 1;
        Some(row)
    }
}

pub fn create_game(n: u8, max_try: usize) -> Option<Vec<Vec<u8>>> {
    let mut generator = SudokuGenerator::new(n);
    let mut table = Vec::with_capacity(n as usize);

    let mut retries = 0;

    loop {
        if table.len() == n as usize {
            break;
        }

        if let Some(row) = generator.next() {
            table.push(row);
        } else {
            retries += 1;
            if retries >= max_try {
                return None;
            }
            generator.clear();
            table.clear();
        }
    }

    Some(table)
}

pub fn hide_answer_mask(n: usize, g: GemeType) -> Vec<Vec<bool>> {
    let total = n * n;
    let max_remove = match g {
        GemeType::Easy => total / 3,
        GemeType::Medium => total / 2,
        GemeType::Difficult => (total * 2) / 3,
    };
    let mut count = 0;
    let mut total_try = 0;
    let mut arr = vec![vec![false; n]; n];
    while count < max_remove && total_try < 1000000 {
        let ranw_raw = rand::random_range(0..n);
        let rand_coll = rand::random_range(0..n);
        if !arr[rand_coll][ranw_raw] {
            arr[rand_coll][ranw_raw] = true;
            count += 1;
        }
        total_try += 1;
    }
    return arr;
}

pub fn hide_answer_mask_mut(arr: &mut Vec<Vec<u8>>, g: GemeType) -> Vec<Vec<bool>> {
    let total = arr.len() * arr.len();
    let max_remove = match g {
        GemeType::Easy => total / 3,
        GemeType::Medium => total / 2,
        GemeType::Difficult => (total * 2) / 3,
    };
    let mut count = 0;
    let mut total_try = 0;
    let mut arr_bool = vec![vec![false; arr.len()]; arr.len()];
    while count < max_remove && total_try < 1000000 {
        let ranw_raw = rand::random_range(0..arr.len());
        let rand_coll = rand::random_range(0..arr.len());
        if !arr_bool[rand_coll][ranw_raw] {
            arr_bool[rand_coll][ranw_raw] = true;
            arr[rand_coll][ranw_raw] = 0;
            count += 1;
        }
        total_try += 1;
    }
    return arr_bool;
}

pub(crate) fn vec_vec_bool_to_vec_u8(v: Vec<Vec<bool>>) -> Vec<u8> {
    let total = v.len() * v.len();

    // flatten
    let mut large = Vec::with_capacity(total);
    for row in v {
        large.extend_from_slice(&row);
    }
    let rem = large.len() % 8;
    if rem != 0 {
        large.extend(std::iter::repeat(false).take(8 - rem));
    }

    // pack 8 bools -> 1 byte
    let mut arr = Vec::with_capacity(large.len() / 8);

    for chunk in large.chunks_exact(8) {
        let mut byte = 0u8;

        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |= 1 << i;
            }
        }

        arr.push(byte);
    }

    arr
}

pub(crate) fn vec_u8_to_vec_vec_bool(data: &[u8], n: usize) -> Vec<Vec<bool>> {
    let total = n * n;

    // unpack bits
    let mut flat = Vec::with_capacity(data.len() * 8);

    for &byte in data {
        for i in 0..8 {
            flat.push((byte & (1 << i)) != 0);
        }
    }

    // remove padding
    flat.truncate(total);

    // rebuild matrix
    let mut grid = Vec::with_capacity(n);
    for i in 0..n {
        let start = i * n;
        let end = start + n;
        grid.push(flat[start..end].to_vec());
    }

    grid
}

#[cfg(test)]
mod tests {
    use crate::game::{vec_u8_to_vec_vec_bool, vec_vec_bool_to_vec_u8};

    #[test]
    fn test_111() {
        let v = vec![vec![true; 9]; 9];
        let u = vec_vec_bool_to_vec_u8(v.clone());
        let got = vec_u8_to_vec_vec_bool(u.as_slice(), 9);
        assert_eq!(v, got);

        let v = vec![vec![false; 9]; 9];
        let u = vec_vec_bool_to_vec_u8(v.clone());
        let got = vec_u8_to_vec_vec_bool(u.as_slice(), 9);
        assert_eq!(v, got);
    }
}
