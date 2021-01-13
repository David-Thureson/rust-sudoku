#![allow(dead_code)]

use rand::{thread_rng, Rng};
use itertools::Itertools;

use std::collections::HashSet;
use array2d::Array2D;
use std::fmt::{Display, Formatter, Error};

const NO_VALUE: u8 = 0;
const RUN_INVARIANT: bool = false;

pub fn main() {
    try_build();

}

fn try_build() {
    // 2 by 2.
    // let grid = Grid::build(2, 2, 1, 1);
    // 3 by 3.
    // let grid = Grid::build(3, 3, 1, 1);
    // 4 by 4 with four blocks.
    let grid = Grid::build(4, 4, 2, 2);
    // Standard size.
    // let grid = Grid::build(9, 9, 3, 3);
    // 16 x 16 with 16 blocks.
    // let grid = Grid::build(16, 16, 4, 4);
    // 6 x 6 with 2x3 blocks.
    // let grid = Grid::build(6, 6, 2, 3);
    // 5 x 5 with 2x2 blocks, thus some smaller blocks
    // let grid = Grid::build(5, 5, 2, 2);

    // let grid = Grid::build(11, 11, 3, 3);

    match grid {
        Some(grid) => grid.print_simple(),
        None => println!("Unable to produce grid."),
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    grid_width: u8,
    grid_height: u8,
    block_width: u8,
    block_height: u8,
    block_col_count: u8,
    block_row_count: u8,
    block_count: u8,
    block_cell_count: u8,
    cell_count: u16,
    unsolved_cell_count: u16,
    max_value: u8,
    cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
struct Cell {
    pub value: u8,
    pub index: u8,
    pub row: u8,
    pub col: u8,
    pub block: u8,
    pub rem_values: HashSet<u8>,
}

impl Grid {
    pub fn new(grid_width: u8, grid_height: u8, block_width: u8, block_height: u8) -> Self {
        let block_col_count = (grid_width as f64 / block_width as f64).ceil() as u8;
        let block_row_count = (grid_height as f64 / block_height as f64).ceil() as u8;
        let block_count = block_col_count * block_row_count;
        let block_cell_count = block_width * block_height;
        let cell_count = grid_width as u16 * grid_height as u16;
        
        let max_value = *[grid_width, grid_height, block_cell_count].iter().max().unwrap();
        let mut rem_values = HashSet::with_capacity(max_value as usize);
        for value in 1..=max_value {
            rem_values.insert(value);
        }
            
        let mut cells = Vec::with_capacity(cell_count as usize);
        for index in 0..cell_count {
            let (row, col, block) = row_col_block(index as u8, grid_width, block_width, block_height, block_col_count);
            cells.push(Cell::new(index as u8, row, col, block, rem_values.clone()));
        }
        Self {
            grid_width,
            grid_height,
            block_width,
            block_height,
            block_col_count,
            block_row_count,
            block_count,
            block_cell_count,
            cell_count,
            unsolved_cell_count: cell_count,
            max_value,
            cells,
        }
    }

    pub fn build(grid_width: u8, grid_height: u8, block_width: u8, block_height: u8) -> Option<Self> {
        let grid = Grid::new(grid_width, grid_height, block_width, block_height);
        //bg!(&grid);
        let grid = Self::build_next_cell(&grid);
        if let Some(grid) = grid {
            grid.invariant();
            return Some(grid)
        }
        None
    }

    fn cell(&self, index: u8) -> &Cell {
        &self.cells[index as usize]
    }

    fn cell_mut(&mut self, index: u8) -> &mut Cell {
        self.cells.get_mut(index as usize).unwrap()
    }

    fn related_cell_indexes(&self, index: u8) -> Vec<u8> {
        let (row, col, block) = {
            let cell = self.cell(index);
            (cell.row, cell.col, cell.block)
        };
        self.cells
            .iter()
            .filter(|other_cell| other_cell.index != index && (other_cell.row == row || other_cell.col == col || other_cell.block == block))
            .map(|other_cell| other_cell.index)
            .collect()
    }

    fn build_next_cell(grid_to_now: &Grid) -> Option<Grid> {

        // rintln!("build_next_cell() top: unsolved cells = {}, remaining options = {}", grid_to_now.unsolved_cell_count, grid_to_now.rem_value_count());
        // grid_to_now.print_simple_and_remaining();

        // if filled_cell_count >= 255 {
        //     return Some(grid_to_now.clone());
        // }
        //bg!(grid_to_now);
        // grid_to_now.debug_cell_and_related(0);
        // panic!();

        let mut options = vec![];
        for (cell_index, cell) in grid_to_now.cells.iter().enumerate() {
            if cell.value == NO_VALUE {
                for value in cell.rem_values.iter() {
                    options.push((cell_index as u8, value));
                }
            }
        }

        // let max_attempts = 5;
        // let option_count = options.len();
        // let option_count_threshold = option_count - max_attempts;
        // let mut attempt_count = 1;

        while !options.is_empty() {
            // while attempt_count <= max_attempts {
            //rintln!("Top of loop: unsolved_cell_count = {}, options.len() == {}", grid_to_now.unsolved_cell_count, options.len());
            //rintln!("Top of loop: filled_cell_count = {}, options.len() == {}, attempt_count = {}", filled_cell_count, options.len(), attempt_count);
            let option = options.remove(thread_rng().gen_range(0, options.len()));
            let mut grid = grid_to_now.clone();
            //rintln!("\tOption is {} with value {}", &grid_to_now.cell(option.0), option.1);
            let remaining_options = grid.set_value(option.0, *option.1);
            if remaining_options == 0 {
                if grid.unsolved_cell_count == 0 {
                    return Some(grid);
                } else {
                    //rintln!("\t\tOption did not work.");
                    return None;
                }
            } else {
                // This option worked so keep going.
                //rintln!("\t\tOption worked, continuing.");
                let completed_grid = Self::build_next_cell(&grid);
                if let Some(completed_grid) = completed_grid {
                    return Some(completed_grid);
                }
            }
        }
        None
    }

    /*
    fn empty_cells(&self) -> Vec<u8> {
        self.cells.iter().enumerate().filter(|(index, value)| value == 0).map(|(index, _)| index).collect()
    }
    */

    fn set_value(&mut self, index: u8, value: u8) -> u16 {
        {
            let mut cell = self.cell_mut(index);
            cell.value = value;
            cell.rem_values.clear();
            //rintln!("set_value() for {}", &cell);
        };
        self.unsolved_cell_count -= 1;
        if self.unsolved_cell_count == 0 {
            if RUN_INVARIANT { self.invariant(); }
            return 0;
        }

        for related_cell_index in self.related_cell_indexes(index) {
            let related_cell = self.cell_mut(related_cell_index);
            if related_cell.value == 0 {
                // if related_cell.rem_values.contains(&value) {
                //     rintln!("set_value(): removing value {} from related cell {}.", value, &related_cell);
                // }
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

        let mut remaining_options: u16 = 0;
        for related_cell_index in self.related_cell_indexes(index) {
            let related_cell = self.cell(related_cell_index);
            if related_cell.value == 0 {
                let one_cell_remaining_options = related_cell.rem_value_count();
                if one_cell_remaining_options == 0 {
                    return 0;
                }
                remaining_options += one_cell_remaining_options as u16;
            }
        }

        if RUN_INVARIANT { self.invariant(); }
        remaining_options
    }

    fn clear_single_remaining(&mut self) -> bool {
        // Return true if a cell was found with zero remaining options.
        loop {
            let cell_indexes_with_one_remaining = self.cells
                .iter()
                .enumerate()
                .filter(|(_index, cell)| cell.rem_values.len() == 1)
                .map(|(index, cell)| (index as u8, cell))
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
                return true;
            }
        }
    }

    pub fn print_simple(&self) {
        self.print_simple_and_remaining_internal(false);
    }

    pub fn print_simple_and_remaining(&self) {
        self.print_simple_and_remaining_internal(true);
    }

    fn print_simple_and_remaining_internal(&self, print_remaining: bool) {
        let cell_row_padding: usize = 0;
        let cell_col_padding: usize = 1;
        let block_row_padding: usize = 1;
        let block_col_padding: usize = 3;
        let grid_col_padding: usize = 5;
        let num_rows: usize = self.grid_height as usize + (cell_row_padding as usize * (self.grid_height as usize - 1)) + (block_row_padding * (self.block_row_count as usize - 1));
        let num_cols_one_grid: usize = self.grid_width as usize + (cell_col_padding * (self.grid_width as usize - 1)) + (block_col_padding * (self.block_col_count as usize - 1));
        let num_cols: usize = if print_remaining {
            (num_cols_one_grid * 2) + grid_col_padding
        } else {
            num_cols_one_grid
        };
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows as usize, num_cols as usize);
        for cell in self.cells.iter() {
            let x: usize = (cell.col as usize * cell_col_padding) + (self.block_col_index(cell.col) as usize * block_col_padding) + cell.col as usize;
            let y: usize = (cell.row as usize * cell_row_padding) + (self.block_row_index(cell.row) as usize * block_row_padding) + cell.row as usize;
            let value = if cell.value == 0 { ".".to_string() } else { value_to_char(cell.value) };
            ar.set(y, x, value).unwrap();
            if print_remaining {
                let x = x + num_cols_one_grid + grid_col_padding;
                let value = cell.rem_values.len();
                let value = if value == 0 { ".".to_string() } else { value_to_char(value as u8) };
                // let value = cell.row.to_string();
                // let value = cell.col.to_string();
                // let value = cell.block.to_string();
                ar.set(y, x, value).unwrap();
            }
        }
        println!("\n");
        for mut row in ar.rows_iter() {
            let row_string = row.join("");
            println!("{}", row_string);
        }
        println!();
    }

    fn block_row_index(&self, row: u8) -> u8 {
        row / self.block_height
    }

    fn block_col_index(&self, col: u8) -> u8 {
        col / self.block_width
    }

    pub fn debug_cell_and_related(&self, index: u8) {
        self.print_simple_and_remaining();
        dbg!(self.cell(index));
        for related_cell_index in self.related_cell_indexes(index) {
            dbg!(self.cell(related_cell_index));
        }
    }

    pub fn invariant(&self) {
        for cell in self.cells.iter().filter(|cell| cell.value != 0) {
            for related_cell_index in self.related_cell_indexes(cell.index) {
                let related_cell = self.cell(related_cell_index);
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

    fn unsolved_cell_count(&self) -> u8 {
        self.cells.iter().filter(|cell| cell.value == 0).count() as u8
    }

    fn rem_value_count(&self) -> usize {
        self.cells.iter().map(|cell| cell.rem_value_count() as usize).sum::<usize>()
    }

}

impl Cell {
    pub fn new(index: u8, row: u8, col: u8, block: u8, rem_values: HashSet<u8>) -> Self {
        Self {
            value: 0,
            index,
            row,
            col,
            block,
            rem_values,
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

    pub fn rem_value_count(&self) -> usize {
        self.rem_values.len()
    }

}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let values = if self.value == 0 {
            let remaining_values = self.rem_values.iter().sorted().map(|value| value_to_char(*value)).collect::<Vec<_>>().join("");
            format!("[{}]", remaining_values)
        } else {
            value_to_char(self.value)
        };
        write!(f, "cell r {} c {}: {}", self.row + 1, self.col + 1, values)
    }
}

fn row_col_block(index: u8, grid_width: u8, block_width: u8, block_height: u8, block_col_count: u8) -> (u8, u8, u8) {
    let row = index / grid_width;
    let col = index % grid_width;
    let block = ((row / block_height) * block_col_count) + (col / block_width);
    (row, col, block)
}

fn value_to_char(value: u8) -> String {
    match value {
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        16 => "G".to_string(),
        _ => value.to_string()
    }
}
