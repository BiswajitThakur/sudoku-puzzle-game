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

pub(crate) fn create_game(n: u8, max_try: usize) -> Option<Vec<Vec<u8>>> {
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
