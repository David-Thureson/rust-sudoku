#![allow(dead_code)]

use rand::{thread_rng, Rng};
use itertools::Itertools;

use std::collections::HashSet;
use std::iter::FromIterator;
use array2d::Array2D;
use std::fmt::{Display, Formatter, Error};

const ROWS: usize = 9;
const COLS: usize = 9;
const CELL_COUNT: usize = 81;
const NO_VALUE: u8 = 0;

pub fn main() {
    try_build();

}

fn try_build() {
    let g = Grid::build();
    //bg!(&g);
}

#[derive(Clone, Debug)]
struct GridModel {
    pub cells: Vec<GridModelCell>,
}

#[derive(Clone, Debug)]
struct GridModelCell {
    pub index: usize,
    pub row: usize,
    pub col: usize,
    pub block: usize,
}

#[derive(Clone, Debug)]
struct Grid {
    pub model: GridModel,
    pub cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
struct Cell {
    pub value: u8,
    pub index: usize,
    pub row: usize,
    pub col: usize,
    pub block: usize,
    pub rem_values: HashSet<u8>,
}

impl GridModel {
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(CELL_COUNT as usize);
        for index in 0..CELL_COUNT {
            cells.push(GridModelCell::new(index));
        }
        Self {
            cells,
        }
    }

    pub fn related_cell_indexes(&self, index: usize) -> Vec<usize> {
        let (row, col, block) = {
            let cell = self.cells.get(index).unwrap();
            (cell.row, cell.col, cell.block)
        };
        self.cells
            .iter()
            .filter(|other_cell| other_cell.index != index && (other_cell.row == row || other_cell.col == col || other_cell.block == block))
            .map(|other_cell| other_cell.index)
            .collect()
    }
}

impl GridModelCell {
    pub fn new(index: usize) -> Self {
        let row = index / COLS;
        let col = index % ROWS;
        let block = ((row / 3) * 3) + col / 3;
        Self {
            index,
            row,
            col,
            block,
        }
    }
}

impl Grid {
    fn new() -> Self {
        let mut cells = Vec::with_capacity(CELL_COUNT as usize);
        for index in 0..CELL_COUNT {
            cells.push(Cell::new(index));
        }
        Self {
            model: GridModel::new(),
            cells,
        }
    }

    pub fn build() -> Self {
        let g = Grid::new();
        Self::build_next_cell(&g, 0).unwrap()
    }

    fn build_next_cell(grid_to_now: &Grid, filled_cell_count: u8) -> Option<Grid> {

        //bg!(filled_cell_count);
        //grid_to_now.print_simple_and_remaining();

        if filled_cell_count >= 255 {
            return Some(grid_to_now.clone());
        }
        //bg!(grid_to_now);
        // grid_to_now.debug_cell_and_related(0);
        // panic!();

        assert!(filled_cell_count < CELL_COUNT as u8);
        let mut options = vec![];
        for (cell_index, cell) in grid_to_now.cells.iter().enumerate() {
            if cell.value == NO_VALUE {
                for value in cell.rem_values.iter() {
                    options.push((cell_index, value));
                }
            }
        }

        // let max_attempts = 5;
        // let option_count = options.len();
        // let option_count_threshold = option_count - max_attempts;
        // let mut attempt_count = 1;

        while !options.is_empty() {
        // while attempt_count <= max_attempts {
            println!("Top of loop: filled_cell_count = {}, options.len() == {}, attempt_count = {}", filled_cell_count, options.len(), attempt_count);
            //rintln!("Top of loop: filled_cell_count = {}, options.len() == {}, attempt_count = {}", filled_cell_count, options.len(), attempt_count);
            let option = options.remove(thread_rng().gen_range(0, options.len()));
            let mut grid = grid_to_now.clone();
            //rintln!("Option is {} with value {}", &grid_to_now.cells[option.0], option.1);
            let remaining_options = grid.set_value(option.0, *option.1);
            if filled_cell_count + 1 == CELL_COUNT as u8 {
                assert_eq!(0, remaining_options);
                return Some(grid);
            }
            if remaining_options > 0 {
                // This option worked so keep going.
                //rintln!("Option worked, continuing.");
                let completed_grid = Self::build_next_cell(&grid, filled_cell_count + 1);
                if let Some(completed_grid) = completed_grid {
                    return Some(completed_grid);
                }
            } else {
                //rintln!("Option did not work.");
            }
            // attempt_count += 1;
        }
        None
    }

    /*
    fn empty_cells(&self) -> Vec<u8> {
        self.cells.iter().enumerate().filter(|(index, value)| value == 0).map(|(index, _)| index).collect()
    }
    */

    fn set_value(&mut self, index: usize, value: u8) -> usize {
        {
            let mut cell = self.cells.get_mut(index).unwrap();
            cell.value = value;
            cell.rem_values.clear();
            //rintln!("set_value() for {}", &cell);
        };

        for related_cell_index in self.model.related_cell_indexes(index) {
            let related_cell = self.cells.get_mut(related_cell_index).unwrap();
            if related_cell.value == 0 {
                // if related_cell.rem_values.contains(&value) {
                //     //rintln!("set_value(): removing value {} from related cell {}.", value, &related_cell);
                //}
                let one_cell_remaining_options = related_cell.remove_rem_value(value);
                if one_cell_remaining_options == 0 {
                    return 0;
                }
            }
        }

        let zero_found = self.clear_single_remaining();
        if zero_found {
            return 0;
        }

        let mut remaining_options = 0;
        for related_cell_index in self.model.related_cell_indexes(index) {
            let related_cell = self.cells.get(related_cell_index).unwrap();
            if related_cell.value == 0 {
                let one_cell_remaining_options = related_cell.rem_values.len();
                if one_cell_remaining_options == 0 {
                    return 0;
                }
                remaining_options += one_cell_remaining_options;
            }
        }

        self.invariant();
        remaining_options
    }

    fn clear_single_remaining(&mut self) -> bool {
        // Return true if a cell was found with zero remaining options.
        while true {
            let cell_indexes_with_one_remaining = self.cells
                .iter()
                .enumerate()
                .filter(|(index, cell)| cell.rem_values.len() == 1)
                .collect::<Vec<_>>();
            if cell_indexes_with_one_remaining.is_empty() {
                return false;
            }
            let (index, value) = {
                let (index, cell) = cell_indexes_with_one_remaining[0];
                //rintln!("clear_single_remaining() found {}", &cell);
                (index, *cell.rem_values.iter().collect::<Vec<_>>()[0])
            };
            let remaining_after_set_value = self.set_value(index, value);
            if remaining_after_set_value == 0 {
                return true
            }
        }
        false
    }

    fn print_simple_and_remaining(&self) {
        let cell_row_padding = 0;
        let cell_col_padding = 1;
        let block_row_padding = 1;
        let block_col_padding = 3;
        let grid_col_padding = 5;
        let num_rows = 9 + (cell_row_padding * 8) + (block_row_padding * 2);
        let num_cols_one_grid = 9 + (cell_col_padding * 8) + (block_col_padding * 2);
        let num_cols = (num_cols_one_grid * 2) + grid_col_padding;
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows, num_cols);
        for cell in self.cells.iter() {
            let x = (cell.col * cell_col_padding) + (Self::block_col_index(cell.col) * block_col_padding) + cell.col;
            let y = (cell.row * cell_row_padding) + (Self::block_row_index(cell.row) * block_row_padding) + cell.row;
            let value = if cell.value == 0 { ".".to_string() } else { cell.value.to_string() };
            ar.set(y, x, value);
            let x = x + num_cols_one_grid + grid_col_padding;
            let value = cell.rem_values.len();
            let value = if value == 0 { ".".to_string() } else { value.to_string() };
            // let value = cell.row.to_string();
            // let value = cell.col.to_string();
            // let value = cell.block.to_string();
            ar.set(y, x, value);
        }
        println!("\n");
        for mut row in ar.rows_iter() {
            let row_string = row.join("");
            println!("{}", row_string);
        }
        println!();
    }

    fn block_row_index(row: usize) -> usize {
        row / 3
    }

    fn block_col_index(col: usize) -> usize {
        col / 3
    }

    pub fn debug_cell_and_related(&self, index: usize) {
        self.print_simple_and_remaining();
        dbg!(&self.cells[index]);
        for related_cell_index in self.model.related_cell_indexes(index) {
            dbg!(&self.cells[related_cell_index]);
        }
    }

    fn invariant(&self) {
        for cell in self.cells.iter().filter(|cell| cell.value != 0) {
            for related_cell_index in self.model.related_cell_indexes(cell.index) {
                let related_cell = &self.cells[related_cell_index];
                if related_cell.value == cell.value {
                    self.print_simple_and_remaining();
                    panic!("{} and {} have the same value.", cell, related_cell);
                }
                if related_cell.rem_values.contains(&cell.value) {
                    self.print_simple_and_remaining();
                    panic!("{} has a remaining value found in {}.", related_cell, cell);
                }
            }
        }
    }

}

impl Cell {
    pub fn new(index: usize) -> Self {
        let row = index / COLS;
        let col = index % ROWS;
        let block = (row / 3 * 3) + col / 3;
        Self {
            value: 0,
            index,
            row,
            col,
            block,
            rem_values: HashSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9].iter().map(|x| *x as u8))
        }
    }

    pub fn remove_rem_value(&mut self, value: u8) -> usize {
        if self.value == 0 {
            self.rem_values.remove(&value);
            self.rem_values.len()
        } else {
            0
        }
    }

}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let values = if self.value == 0 {
            let remaining_values = self.rem_values.iter().sorted().map(|value| value.to_string()).collect::<Vec<_>>().join("");
            format!("[{}]", remaining_values)
        } else {
            self.value.to_string()
        };
        write!(f, "cell r {} c {}: {}", self.row + 1, self.col + 1, values)
    }
}

