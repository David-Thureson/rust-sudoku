#[derive(Clone)]
pub enum TextGridCell {
    Empty,
    Text {
        text: String,
    },
    Grid {
        grid: TextGrid,
    }
}

#[derive(Clone)]
pub struct TextGrid {
    row_padding: usize,
    col_padding: usize,
    rows: Vec<TextGridRow>,
    col_count: usize,
}

#[derive(Clone)]
pub struct TextGridRow {
    cells: Vec<TextGridCell>
}

impl TextGridCell {
    pub fn width(&self) -> usize {
        match self {
            TextGridCell::Empty => 1,
            TextGridCell::Text { text } => text.len(),
            TextGridCell::Grid { grid } => grid.width(),
        }
    }

    pub fn height(&self) -> usize {
        match self {
            TextGridCell::Empty => 1,
            TextGridCell::Text { text } => text.split('\n').collect::<Vec<_>>().len(),
            TextGridCell::Grid { grid } => grid.height(),
        }
    }
}

impl TextGrid {
    pub fn new(row_padding: usize, col_padding: usize) -> Self {
        Self {
            row_padding,
            col_padding,
            rows: vec![],
            col_count: 0,
        }
    }

    pub fn set_cell(&mut self, row_index: usize, col_index: usize, cell: TextGridCell) {
        let row_col_count = col_index + 1;
        for row in self.rows.iter_mut() {
            row.set_col_count(row_col_count);
        }
        if row_index >= self.row_count() {
            let rows_to_add = (row_index + 1) - self.row_count();
            for _ in 0..rows_to_add {
                self.rows.push(TextGridRow::new(row_col_count));
            }
        }
        let row = &mut self.rows[row_index];
        row.set_cell(col_index, cell);
    }

    pub fn set_cells(&mut self, from_row_index: usize, to_row_index: usize, from_col_index: usize, to_col_index: usize, cell: TextGridCell) {
        assert!(from_row_index <= to_row_index);
        assert!(from_col_index <= to_col_index);
        for row_index in from_row_index..=to_row_index {
            for col_index in from_col_index..=to_col_index {
                self.set_cell(row_index, col_index, cell.clone());
            }
        }
    }

    pub fn width(&self) -> usize {
        let content_width: usize = (0..self.col_count).map(|col_index| self.col_width(col_index)).sum();
        let padding_width = self.col_padding * (self.col_count - 1);
        content_width + padding_width
    }

    pub fn height(&self) -> usize {
        let content_height: usize = (0..self.row_count()).map(|row_index| self.row_height(row_index)).sum();
        let padding_height = self.row_padding * (self.row_count() - 1);
        content_height + padding_height
    }

    fn col_width(&self, col_index: usize) -> usize {
        self.rows.iter().map(|row| row.col_width(col_index)).max().unwrap()
    }

    fn row_height(&self, row_index: usize) -> usize {
        self.rows[row_index].height()
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn print() {

    }

    pub fn to_lines(&self) -> Vec<String> {
        let mut v = vec![];
        let width = self.width();
        let height = self.height();



        v
    }

}

impl TextGridRow {
    pub fn new(col_count: usize) -> Self {
        let mut cells = vec![];
        for _ in 0..col_count {
            cells.push(TextGridCell::Empty);
        }
        Self {
            cells,
        }
    }

    pub fn set_col_count(&mut self, col_count: usize) {
        assert!(col_count >= self.cells.len());
        if col_count > self.cells.len() {
            for _ in 0..(col_count - self.cells.len()) {
                self.cells.push(TextGridCell::Empty);
            }
        }
        assert_eq!(col_count, self.cells.len())
    }

    pub fn set_cell(&mut self, col_index: usize, cell: TextGridCell) {
        self.cells[col_index] = cell
    }

    pub fn col_width(&self, col_index: usize) -> usize {
        self.cells.get(col_index).unwrap().width()
    }

    pub fn height(&self) -> usize {
        self.cells.iter().map(|cell| cell.height()).max().unwrap()
    }
}
