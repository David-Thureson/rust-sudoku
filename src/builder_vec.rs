#![allow(dead_code)]

// Differences from builder_obj:
// - Eliminated cell objects, putting as much as possible in fixed-length vectors in Grid.
// - In other ways reduced creating objects.

use rand::{thread_rng, Rng};
use itertools::Itertools;
use bit_vec::BitVec;
use std::sync::Mutex;
use std::collections::HashSet;
use array2d::Array2D;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::ops::Range;
//use std::fmt::{Display, Formatter, Error};
use std::time::Instant;

use crate::*;

const NO_VALUE: u8 = 0;
const RUN_INVARIANT: bool = false;
const VERBOSE: u8 = 0;
const LOG_LEVEL: u8 = 2;

lazy_static! {
    static ref TRIED_GRIDS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

pub fn main() {
    try_build();
}

fn try_build() {
    // let max_grid_count = Some(0);
    let max_grid_count = None;

    // 2 by 2.
    // let grid = Grid::build(2, 2, 1, 1, None);
    // 3 by 2.
    // let grid = Grid::build(3, 2, 1, 1, None);
    // 3 by 3.
    // let grid = Grid::build(3, 3, 1, 1, max_grid_count);
    // 4 by 4 with four blocks.
    // let grid = Grid::build(4, 4, 2, 2, max_grid_count);
    // Standard size.
    // let grid = Grid::build(9, 9, 3, 3, max_grid_count);
    // 12 x 12.
    // let grid = Grid::build(12, 12, 4, 3, None);
    // 16 x 16 with 16 blocks.
    // let grid = Grid::build(16, 16, 4, 4);
    // 6 x 6 with 2x3 blocks.
    // let grid = Grid::build(6, 6, 2, 3, None);
    // 6 x 6 with 2x3 blocks.
    // let grid = Grid::build(6, 6, 2, 3, None);
    // 7 x 7 with 4x4 blocks.
    // let grid = Grid::build(7, 7, 4, 4, None);
    // 8 x 8 with 4x4blocks.
    // let grid = Grid::build(8, 8, 4, 4, None);
    // 9 x 9 with 4x4blocks.
    // let grid = Grid::build(9, 9, 4, 4, None);
    // 5 x 5 with 2x2 blocks, thus some smaller blocks
    // let grid = Grid::build(5, 5, 2, 2);
    // 16 x 16.
    let grid = Grid::build(16, 16, 4, 4, max_grid_count);

    match grid {
        Some(grid) => {
            //bg!(&grid);
            grid.print_simple("try_grid() end, success");
        },
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
    max_related_cells: u8,
    max_tried_grid_count: Option<usize>,
    values: Vec<u8>,
    remaining_value_count: u16,
    remaining_value_counts: Vec<u8>,
    // remaining_value_counts_map: Vec<HashSet<u8>>,
    remaining_values: BitVec,
    related_cell_indexes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct CellForRelated {
    pub row: u8,
    pub col: u8,
    pub block: u8,
}

struct FailedGridSet {
    hashes: HashSet<u64>,
}

#[derive(Hash)]
struct FailedGrid {
    values: Vec<(u8, u8)>,
}

impl Grid {
    pub fn new(grid_width: u8, grid_height: u8, block_width: u8, block_height: u8, max_tried_grid_count: Option<usize>) -> Self {
        let block_col_count = (grid_width as f64 / block_width as f64).ceil() as u8;
        let block_row_count = (grid_height as f64 / block_height as f64).ceil() as u8;
        let block_count = block_col_count * block_row_count;
        let block_cell_count = block_width * block_height;
        let cell_count = grid_width as u16 * grid_height as u16;

        let max_value = *[grid_width, grid_height, block_cell_count].iter().max().unwrap();

        let max_related_cells = (grid_width - block_width) + (grid_height - block_height) + (block_cell_count - 1);

        let mut values = Vec::with_capacity(cell_count as usize);

        let remaining_value_count = cell_count * max_value as u16;

        let mut remaining_value_counts = Vec::with_capacity(cell_count as usize);
        for _ in 0..cell_count {
            values.push(NO_VALUE);
            remaining_value_counts.push(max_value);
        }

        /*
        let mut remaining_value_counts_map= Vec::with_capacity(max_value as usize + 1);
        for i in 0..=max_value {
            if i == max_value {
                let mut remaining_set = HashSet::with_capacity(cell_count as usize);
                for i in 0..cell_count {
                    remaining_set.insert(i as u8);
                }
                remaining_value_counts_map.push(remaining_set);
            } else {
                remaining_value_counts_map.push(HashSet::new());
            }
        }
        */

        let remaining_values = BitVec::from_elem(max_value as usize * cell_count as usize, true);

        let mut cells_for_related = Vec::with_capacity(cell_count as usize);
        for index in 0..cell_count {
            let (row, col, block) = row_col_block(index as u8, grid_width, block_width, block_height, block_col_count);
            cells_for_related.push(CellForRelated::new(row, col, block));
        }
        //bg!(&cells_for_related);
        let mut related_cell_indexes= Vec::with_capacity(max_value as usize * cell_count as usize);
        for index in 0..cell_count as usize {
            let (row, col, block) = {
                let cell = &cells_for_related[index];
                (cell.row, cell.col, cell.block)
            };
            let mut filled_count = 0;
            for (other_cell_index, _) in cells_for_related
                    .iter()
                    .enumerate()
                    .filter(|(other_cell_index, other_cell)| *other_cell_index != index && (other_cell.row == row || other_cell.col == col || other_cell.block == block)) {
                related_cell_indexes.push(other_cell_index as u8);
                filled_count += 1;
            }
            let to_fill_count = max_related_cells - filled_count;
            for _ in 0..to_fill_count {
                // This will only come up in unusual cases where the grid is not symmetrical because
                // the block width does not divide evenly into the grid width and/or the same for
                // height. So we need a value that we know can't be right, in this case the
                // original index. Even if we forget to check for this case it probably won't do any
                // harm besides wasting cycles.
                related_cell_indexes.push(index as u8);
            }
            if VERBOSE >= 2 { println!("Grid::new() bottom of related cell loop: index = {}, related_cell_indexes = [{}]", index, related_cell_indexes.iter().join(", ")); }
        }
        // for (index, related_cell_mapping) in related_cell_indexes.iter().enumerate() {
        //     println!("\nindex = {}, related_cell_indexes = [{}]", index, related_cell_mapping.iter().map(|related_index| related_index.to_string()).join(", "));
        // }

        let grid = Self {
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
            max_related_cells,
            max_tried_grid_count,
            values,
            remaining_value_count,
            remaining_value_counts,
            // remaining_value_counts_map,
            remaining_values,
            related_cell_indexes,
        };
        if VERBOSE >= 1 { dbg!(&grid); }
        grid
    }

    pub fn build(grid_width: u8, grid_height: u8, block_width: u8, block_height: u8, max_tried_grid_count: Option<usize>) -> Option<Self> {
        TRIED_GRIDS.lock().unwrap().clear();

        let time_start = Instant::now();
        let grid = Grid::new(grid_width, grid_height, block_width, block_height, max_tried_grid_count);
        //bg!(&grid);
        let grid = Self::build_next_cell(&grid);
        if let Some(grid) = grid {
            if SHOW_ELAPSED_TIME { dbg!(Instant::now() - time_start); }
            grid.invariant();
            return Some(grid)
        }
        None
    }

    fn build_next_cell(grid_to_now: &Grid) -> Option<Grid> {

        if VERBOSE >= 1 {
            let label = "build_next_cell() top";
            grid_to_now.print_simple_and_remaining(label);
        }

        // if filled_cell_count >= 255 {
        //     return Some(grid_to_now.clone());
        // }
        //bg!(grid_to_now);
        // grid_to_now.debug_cell_and_related(0);
        // panic!();

        //let mut options = vec![];

        /*
        let try_cell_index = grid_to_now.values
            .iter()
            .enumerate()
            .find(|(_, value)| **value == NO_VALUE)
            .map(|x| x.0)
            .unwrap() as u8;
        */

        /*
        let try_cell_indexes = grid_to_now.values
            .iter()
            .enumerate()
            .filter(|(_, value)| **value == NO_VALUE)
            .map(|x| x.0)
            .collect::<Vec<_>>();
        let try_cell_index = try_cell_indexes[thread_rng().gen_range(0, try_cell_indexes.len())] as u8;
        */

        let try_cell_index = grid_to_now.choose_try_cell_index();

        // let max_attempts = 5;
        // let option_count = options.len();
        // let option_count_threshold = option_count - max_attempts;
        // let mut attempt_count = 1;

        let mut try_values = grid_to_now.remaining_values(try_cell_index);
        while !try_values.is_empty() {
            // let try_value = try_values.remove(thread_rng().gen_range(0, try_values.len()));
            let try_value = try_values.remove(0);

            // Register the grid we're about to try, and if this call returns false it means
            // we've already tried this grid.
            // if grid_to_now.register_attempt(try_cell_index, try_value) {
                let mut try_grid = grid_to_now.clone();
                //rintln!("\tOption is {} with value {}", &grid_to_now.cell(option.0), option.1);
                let set_value_ok = try_grid.set_value(try_cell_index, try_value);
                if set_value_ok {
                    // This option worked so keep going if there's anything left to do.
                    if try_grid.unsolved_cell_count == 0 {
                        //rintln!("\t\tOption worked, grid is completed.");
                        return Some(try_grid);
                    }
                    //rintln!("\t\tOption worked, continuing.");

                    // try_grid.resolve_single_remaining_values();


                    let completed_grid = Self::build_next_cell(&try_grid);
                    if let Some(completed_grid) = completed_grid {
                        return Some(completed_grid);
                    }
                    //rintln!("\t\tOption did not work: set_value() returned false.");
                }
            //}
        }
        // Given grid_to_now as the starting point, we tried all possible values in all remaining
        // cells and nothing worked, so fall back to an earlier version of the grid.
        None
    }

/*
    fn build_next_cell(grid_to_now: &Grid) -> Option<Grid> {

        if VERBOSE >= 1 {
            let label = "build_next_cell() top";
            grid_to_now.print_simple_and_remaining(label);
        }

        // if filled_cell_count >= 255 {
        //     return Some(grid_to_now.clone());
        // }
        //bg!(grid_to_now);
        // grid_to_now.debug_cell_and_related(0);
        // panic!();

        //let mut options = vec![];
        let mut remaining_cell_indexes = grid_to_now.values
            .iter()
            .enumerate()
            .filter(|(_, value)| **value == NO_VALUE)
            .map(|(index, _)| index as u8)
            .collect::<Vec<_>>();

        // let max_attempts = 5;
        // let option_count = options.len();
        // let option_count_threshold = option_count - max_attempts;
        // let mut attempt_count = 1;

        while !remaining_cell_indexes.is_empty() {
            // while attempt_count <= max_attempts {
            //rintln!("Top of loop: unsolved_cell_count = {}, options.len() == {}", grid_to_now.unsolved_cell_count, options.len());
            //rintln!("Top of loop: filled_cell_count = {}, options.len() == {}, attempt_count = {}", filled_cell_count, options.len(), attempt_count);
            let try_cell_index = remaining_cell_indexes.remove(thread_rng().gen_range(0, remaining_cell_indexes.len()));
            let mut try_values = grid_to_now.remaining_values(try_cell_index);
            while !try_values.is_empty() {
                let try_value = try_values.remove(thread_rng().gen_range(0, try_values.len()));

                // Register the grid we're about to try, and if this call returns false it means
                // we've already tried this grid.
                if grid_to_now.register_attempt(try_cell_index, try_value) {
                    let mut try_grid = grid_to_now.clone();
                    //rintln!("\tOption is {} with value {}", &grid_to_now.cell(option.0), option.1);
                    let set_value_ok = try_grid.set_value(try_cell_index, try_value);
                    if set_value_ok {
                        // This option worked so keep going if there's anything left to do.
                        if try_grid.unsolved_cell_count == 0 {
                            //rintln!("\t\tOption worked, grid is completed.");
                            return Some(try_grid);
                        }
                        //rintln!("\t\tOption worked, continuing.");

                        // try_grid.resolve_single_remaining_values();


                        let completed_grid = Self::build_next_cell(&try_grid);
                        if let Some(completed_grid) = completed_grid {
                            return Some(completed_grid);
                        }
                        //rintln!("\t\tOption did not work: set_value() returned false.");
                    }
                }
            }
        }
        // Given grid_to_now as the starting point, we tried all possible values in all remaining
        // cells and nothing worked, so fall back to an earlier version of the grid.
        None
    }
*/
    /*
    fn empty_cells(&self) -> Vec<u8> {
        self.cells.iter().enumerate().filter(|(index, value)| value == 0).map(|(index, _)| index).collect()
    }
    */

    /*
    fn resolve_single_remaining_values(&mut self) {
        while self.resolve_next_remaining_value() {}
    }

    fn resolve_next_remaining_value(&mut self) -> bool {
        for index in 0..self.cell_count {
            let index = index as u8;
            if self.remaining_value_counts[index as usize] == 1 {
                let remaining_value_range = self.remaining_value_range(index);
                let range_start = remaining_value_range.start;
                for remaining_value_index in remaining_value_range {
                    if self.remaining_values[remaining_value_index] {
                        let value = (remaining_value_index - range_start) + 1;
                        self.set_value(index, value as u8);
                        let label = format!("resolve_next_remaining_value(): cell = {}", self.cell_display(index));
                        self.print_simple_and_remaining(&label);
                        if RUN_INVARIANT { self.invariant(); }
                        return true;
                    }
                }
            }
        }
        false
    }
    */

    fn register_attempt(&self, try_cell_index: u8, try_value: u8) -> bool {
        let try_cell_index = try_cell_index as usize;
        assert!(self.values[try_cell_index] == NO_VALUE);
        assert!(try_value >= 1);
        assert!(try_value <= self.max_value);

        let mut tried_grids = TRIED_GRIDS.lock().unwrap();
        let tried_grid_count = tried_grids.len();

        if let Some(max_tried_grid_count) = self.max_tried_grid_count {
            if tried_grid_count >= max_tried_grid_count {
                return true;
            }
        }

        let mut try_values = self.values.clone();
        try_values[try_cell_index] = try_value;

        let mut hasher = DefaultHasher::new();
        try_values.hash(&mut hasher);
        let hash = hasher.finish();

        let is_new = {
            if tried_grids.contains(&hash) {
                // We've already tried this partial grid.
                false
            } else {
                tried_grids.insert(hash);
                true
            }
        };
        is_new
    }

    fn set_value(&mut self, index: u8, value: u8) -> bool {
        if VERBOSE >= 1 {
            let label = format!("\nset_value() top: index = {}, value = {}, {}", index, value, self.cell_display(index));
            self.debug_cell_and_related(&label, index);
        }

        assert!(self.values[index as usize] == NO_VALUE);
        assert!(value > 0);
        assert!(value <= self.max_value);

        self.values[index as usize] = value;
        // let label = format!("\nset_value() after setting value: index = {}, value = {}, {}", index, value, self.cell_display(index));
        // self.debug_cell_and_related(&label, index);

        for remaining_value_index in self.remaining_value_range(index) {
            if self.remaining_values[remaining_value_index] {
                self.remaining_values.set(remaining_value_index, false);
                self.remaining_value_count -= 1;
            }
        }
        self.remaining_value_counts[index as usize] = 0;

        self.unsolved_cell_count -= 1;
        if self.unsolved_cell_count == 0 {
            if RUN_INVARIANT { self.invariant(); }
            return true;
        }

        let related_cell_indexes = self.debug_related_cell_indexes(index);
        //bg!(&related_cell_indexes);

        let mut one_value_indexes = vec![];
        //for related_cell_lookup_index in self.related_cell_range(index) {
        //    let related_cell_index = self.related_cell_indexes[related_cell_lookup_index] as usize;
        for related_cell_index in related_cell_indexes.iter() {
            let related_cell_index = *related_cell_index as usize;
            // let label = format!("set_value() top of related cell loop: index = {}, value = {}, related_cell_index = {}, {}", index, value, related_cell_index, self.cell_display(related_cell_index as u8));
            // self.debug_cell_and_related(&label, index);

            if self.values[related_cell_index] == NO_VALUE {
                // if related_cell.rem_values.contains(&value) {
                //     rintln!("set_value(): removing value {} from related cell {}.", value, &related_cell);
                // }
                let remaining_value_index = ((related_cell_index * self.max_value as usize) + value as usize) - 1;
                //bg!(value);
                //bg!(related_cell_index);
                //bg!(remaining_value_index);
                //bg!(&self.remaining_values);
                if self.remaining_values[remaining_value_index] {
                    self.remaining_values.set(remaining_value_index, false);
                    self.remaining_value_count -= 1;
                    self.remaining_value_counts[related_cell_index] -= 1;
                    let remaining_value_count = self.remaining_value_counts[related_cell_index];

                    /*
                    self.remaining_value_counts_map[remaining_value_count as usize + 1].remove(&(related_cell_index as u8));
                    if remaining_value_count > 0 {
                        self.remaining_value_counts_map[remaining_value_count as usize].insert(related_cell_index as u8);
                    }
                    */

                    /*
                    if remaining_value_count < self.min_cell_remaining_value_count {
                        self.min_cell_remaining_value_count = remaining_value_count;
                        self.cells_with_min_cell_remaining_value_count = 1;
                    } else if remaining_value_count == self.min_cell_remaining_value_count {
                        self.cells_with_min_cell_remaining_value_count += 1;
                    }
                    */

                    match remaining_value_count {
                        0 => {
                            // This empty cell has zero options left for its value so this attempt at the
                            // grid won't work.
                            return false;
                        },
                        1 => {
                            // There's only one possible value left in the related cell.
                            one_value_indexes.push(related_cell_index);
                            // self.set_one_remaining_value(related_cell_index as u8);
                        },
                        _ => {},
                    }
                }
            }
        }

        for related_cell_index in one_value_indexes {
            if self.values[related_cell_index] == NO_VALUE {
                if !self.set_one_remaining_value(related_cell_index as u8) {
                    // This partial grid won't work.
                    return false;
                }
            }
        }

        if RUN_INVARIANT { self.invariant(); }
        true
    }

    #[inline]
    fn set_one_remaining_value(&mut self, index: u8) -> bool {
        if VERBOSE >= 2 {
            let label = format!("set_one_remaining_value(): index = {}, {}", index, self.cell_display(index));
            println!("{}", label);
        }
        // self.debug_cell_and_related(&label, index);
        let remaining_value_range = self.remaining_value_range(index);
        let range_start = remaining_value_range.start;
        for remaining_value_index in remaining_value_range {
            if self.remaining_values[remaining_value_index] {
                let value = (remaining_value_index - range_start) + 1;
                return self.set_value(index, value as u8);
            }
        }
        unreachable!()
    }

    /*
    #[inline]
    fn choose_try_cell_index(&self) -> u8 {
        //bg!(&self.remaining_value_counts_map);
        for i in 2..=self.max_value {
            let set = self.remaining_value_counts_map.get(i as usize).unwrap();
            if set.len() > 0 {
                let v = set.iter().collect::<Vec<_>>();
                let random_index = thread_rng().gen_range(0, v.len());
                //bg!(&v, random_index);
                // println!("choose_try_cell_index(): min = {}, found_count = {}, random_index = {}", min, found_count, random_index);
                return *v[random_index];
            }
        }
        unreachable!()
    }
    */

    /*
    #[inline]
    fn choose_try_cell_index(&self) -> u8 {
        let min = *self.remaining_value_counts.iter().filter(|count| **count > 0).min().unwrap();
        let found_indexes = self.remaining_value_counts.iter().filter(|count| **count == min).collect::<Vec<_>>();
        let random_index = thread_rng().gen_range(0, found_indexes.len());
        *found_indexes[random_index]
    }
    */

    #[inline]
    fn choose_try_cell_index(&self) -> u8 {
        let min = *self.remaining_value_counts.iter().filter(|count| **count > 0).min().unwrap();
        let found_count = self.remaining_value_counts.iter().filter(|count| **count == min).count();
        let random_index = thread_rng().gen_range(0, found_count);
        //rintln!("choose_try_cell_index(): min = {}, found_count = {}, random_index = {}", min, found_count, random_index);

        let mut i = 0;
        for try_cell_index in self.remaining_value_counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count == min)
            .map(|x| x.0) {
            if i == random_index {
                return try_cell_index as u8;
            }
            i += 1;
        }
        unreachable!()
    }

    /*
    fn choose_try_cell_index(&self) -> u8 {
        let random_index = thread_rng().gen_range(0, self.cells_with_min_cell_remaining_value_count);

        let found_count = self.remaining_value_counts.iter().filter(|count| **count == self.min_cell_remaining_value_count).count();
        println!("choose_try_cell_index(): min_cell_remaining_value_count = {}, cells_with_min_cell_remaining_value_count = {}, random_index = {}, found_count = {}", self.min_cell_remaining_value_count, self.cells_with_min_cell_remaining_value_count, random_index, found_count);

        let mut i = 0;
        for try_cell_index in self.remaining_value_counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count == self.min_cell_remaining_value_count)
            .map(|x| x.0) {
            if i == random_index {
                return try_cell_index as u8;
            }
            i += 1;
        }
        unreachable!()
    }
    */

    pub fn print_simple(&self, label: &str) {
        self.print_simple_and_remaining_internal(label, false, false);
    }

    pub fn print_simple_and_remaining_counts(&self, label: &str) {
        self.print_simple_and_remaining_internal(label, true, false);
    }

    pub fn print_simple_and_remaining(&self, label: &str) {
        self.print_simple_and_remaining_internal(label, true, true);
    }

    fn print_simple_and_remaining_internal(&self, label: &str, print_remaining_counts: bool, print_remaining: bool) {
        let cell_row_padding: usize = 0;
        let cell_col_padding: usize = 1;
        let block_row_padding: usize = 1;
        let block_col_padding: usize = 3;
        let grid_col_padding: usize = 5;
        let num_rows: usize = self.grid_height as usize + (cell_row_padding as usize * (self.grid_height as usize - 1)) + (block_row_padding * (self.block_row_count as usize - 1));
        let num_cols_one_grid: usize = self.grid_width as usize + (cell_col_padding * (self.grid_width as usize - 1)) + (block_col_padding * (self.block_col_count as usize - 1));
        let num_cols: usize = if print_remaining_counts {
            (num_cols_one_grid * 2) + grid_col_padding
        } else {
            num_cols_one_grid
        };
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows as usize, num_cols as usize);
        for index in 0..self.cell_count as usize {
            let (row, col, _block) = row_col_block(index as u8, self.grid_width, self.block_width, self.block_height, self.block_col_count);
            let x: usize = (col as usize * cell_col_padding) + (self.block_col_index(col) as usize * block_col_padding) + col as usize;
            let y: usize = (row as usize * cell_row_padding) + (self.block_row_index(row) as usize * block_row_padding) + row as usize;
            let value = self.values[index];
            let value = if value == 0 { ".".to_string() } else { value_to_char(value) };
            ar.set(y, x, value).unwrap();
            if print_remaining_counts {
                let x = x + num_cols_one_grid + grid_col_padding;
                let value = self.remaining_value_counts[index];
                let value = if value == 0 { ".".to_string() } else { value_to_char(value as u8) };
                // let value = cell.row.to_string();
                // let value = cell.col.to_string();
                // let value = cell.block.to_string();
                ar.set(y, x, value).unwrap();
            }
        }
        println!("\n{}", label);
        for mut row in ar.rows_iter() {
            let row_string = row.join("");
            println!("{}", row_string);
        }
        if print_remaining {
            self.print_remaining_values();
        }
    }

    fn print_remaining_values(&self) {
        let cell_row_padding: usize = 1;
        let cell_col_padding: usize = 3;
        let block_row_padding: usize = 3;
        let block_col_padding: usize = 5;
        let (cell_width, cell_height, completed_template) = if self.max_value <= 4 {
            (2, 2, "#│─┘")
        } else if self.max_value <= 6 {
            (3, 2, "│#│└─┘")
        } else if self.max_value <= 9 {
            (3, 3, "┌─┐│#│└─┘")
        } else if self.max_value <= 12 {
            (4, 3, "┌──┐│# │└──┘")
        } else if self.max_value <= 16 {
            (4, 4, "┌──┐│# ││  │└──┘")
        } else {
            panic!()
        };
        let num_rows: usize = (cell_height * self.grid_height as usize) + (cell_row_padding as usize * (self.grid_height as usize - 1)) + (block_row_padding * (self.block_row_count as usize - 1));
        let num_cols: usize = (cell_width * self.grid_width as usize) + (cell_col_padding * (self.grid_width as usize - 1)) + (block_col_padding * (self.block_col_count as usize - 1));
        //bg!(self.grid_width, self.block_col_count, cell_width, cell_col_padding, block_col_padding, num_cols);
        // let num_rows = num_rows * 2;
        // let num_cols = num_cols * 2;
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows as usize, num_cols as usize);
        for index in 0..self.cell_count as usize {
            let (row, col, _block) = row_col_block(index as u8, self.grid_width, self.block_width, self.block_height, self.block_col_count);
            let cell_x: usize = (col as usize * (cell_width + cell_col_padding)) + (self.block_col_index(col) as usize * block_col_padding) as usize;
            let cell_y: usize = (row as usize * (cell_height + cell_row_padding)) + (self.block_row_index(row) as usize * block_row_padding) as usize;
            let range = self.remaining_value_range(index as u8);
            let range_start = range.start;
            let cell_value = self.values[index];
            let internal_values = if cell_value > 0 {
                completed_template.replace("#", &value_to_char(cell_value))
            } else {
                let mut s = "".to_string();
                for remaining_value_index in range {
                    let offset = remaining_value_index - range_start;
                    let value = if self.remaining_values[remaining_value_index] {
                        offset as u8 + 1
                    } else {
                        0
                    };
                    let value = if value == 0 { ".".to_string() } else { value_to_char(value) };
                    s.push_str(&value);
                }
                s
            };
            //bg!(&internal_values, internal_values.len());
            for (offset, c) in internal_values.chars().enumerate() {
                let x = cell_x + (offset % cell_width);
                let y = cell_y + (offset / cell_width);
                ar.set(y, x, c.to_string()).unwrap();
            }
        }
        println!();
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

    pub fn debug_cell_and_related(&self, label: &str, index: u8) {
        self.print_simple_and_remaining(label);
        println!("cell and related:\n\t{}", self.cell_display(index));
        for related_cell_lookup_index in self.related_cell_range(index) {
            let related_cell_index = self.related_cell_indexes[related_cell_lookup_index];
            if related_cell_index != index {
                println!("\t\t{}", self.cell_display(related_cell_index as u8));
            }
        }
    }

    pub fn invariant(&self) {
        for index in 0..self.cell_count as usize {
            let value = self.values[index];
            let remaining_value_count = self.remaining_value_counts[index];

            let mut remaining_value_count_manual = 0;
            for remaining_value_index in self.remaining_value_range(index as u8) {
                if self.remaining_values[remaining_value_index] {
                    remaining_value_count_manual += 1;
                }
            }
            assert_eq!(remaining_value_count, remaining_value_count_manual);

            if value == NO_VALUE {
                // assert!(remaining_value_count >= 2);
                assert!(remaining_value_count >= 1);
            } else {
                // The cell has a value.
                assert_eq!(0, remaining_value_count);
            }

            let (effective_value, note) = if value == NO_VALUE {
                // let remaining_values = self.remaining_values(index as u8);
                // if remaining_values.len() == 1 {
                //     (Some(*remaining_values.get(0).unwrap()), " (from single remaining value)")
                //} else {
                    (None, "")
                //}
            } else {
                (Some(value), "")
            };
            if let Some(effective_value) = effective_value {
                // Make sure no related cell has this cell's value either as its own value or as
                // one of its potential values.
                for related_cell_lookup_index in self.related_cell_range(index as u8) {
                    let related_cell_index = self.related_cell_indexes[related_cell_lookup_index] as usize;
                    if related_cell_index != index {
                        if self.values[related_cell_index] == effective_value {
                            let error_message = format!("{} and {} have the same value{}.", self.cell_display(index as u8), self.cell_display(related_cell_index as u8), note);
                            self.print_simple_and_remaining(&error_message);
                            panic!(error_message);
                        }
                        let remaining_value_index = ((related_cell_index * self.max_value as usize) + effective_value as usize) - 1;
                        if self.remaining_values[remaining_value_index] {
                            let error_message = format!("{} has a remaining value found in {}{}.", self.cell_display(related_cell_index as u8), self.cell_display(index as u8), note);
                            self.print_simple_and_remaining(&error_message);
                            panic!(error_message);
                        }
                    }
                }
            }
        }
    }

        /*
    fn unsolved_cell_count(&self) -> u8 {
        self.cells.iter().filter(|cell| cell.value == 0).count() as u8
    }

    fn rem_value_count(&self) -> usize {
        self.cells.iter().map(|cell| cell.rem_value_count() as usize).sum::<usize>()
    }
    */

    fn cell_display(&self, index: u8) -> String {
        let value = self.values[index as usize];
        let values_string = if value == 0 {
            let remaining_values_string = self.remaining_values(index).iter().map(|value| value_to_char(*value)).collect::<Vec<_>>().join("");
            format!("remaining_values = [{}]", remaining_values_string)
        } else {
            format!("value = {}", value_to_char(value))
        };
        let related_indexes_string = format!("related_indexes = [{}]", self.debug_related_cell_indexes(index).iter().join(", "));
        let (row, col, block) = row_col_block(index, self.grid_width, self.block_width, self.block_height, self.block_col_count);
        format!("cell index {} r {} c {} b {}: {}, {}", index, row + 1, col + 1, block + 1, values_string, related_indexes_string)
    }

    #[inline]
    fn remaining_values(&self, index: u8) -> Vec<u8> {
        let mut v = vec![];
        let remaining_value_range = self.remaining_value_range(index);
        let range_start = remaining_value_range.start;
        for remaining_value_index in remaining_value_range {
            if self.remaining_values[remaining_value_index] {
                let value = (remaining_value_index - range_start) as u8 + 1;
                v.push(value);
            }
        }
        v
    }

    #[inline]
    fn remaining_value_range(&self, index: u8) -> Range<usize> {
        let start_index = index as usize * self.max_value as usize;
        let end_index = start_index + self.max_value as usize;
        Range { start: start_index, end: end_index }
    }

    #[inline]
    fn related_cell_range(&self, index: u8) -> Range<usize> {
        let start_index = index as usize * self.max_related_cells as usize;
        let end_index = start_index + self.max_related_cells as usize;
        let range = Range { start: start_index, end: end_index };
        //bg!(index, &range);
        range
    }

    fn debug_related_cell_indexes(&self, index: u8) -> Vec<u8> {
        // Use this only for debugging. For internal work use related_cell_range().
        let mut indexes = vec![];
        for related_cell_lookup_index in self.related_cell_range(index) {
            let related_cell_index = self.related_cell_indexes[related_cell_lookup_index];
            if related_cell_index != index {
                indexes.push(related_cell_index);
            }
        }
        indexes
    }

}

impl CellForRelated {
    pub fn new(row: u8, col: u8, block: u8) -> Self {
        Self {
            row,
            col,
            block,
        }
    }
}

impl FailedGridSet {
    fn new() -> Self {
        Self {
            hashes: HashSet::new(),
        }
    }
}

impl FailedGrid {
    fn new() -> Self {
        Self {
            values: vec![],
        }
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
