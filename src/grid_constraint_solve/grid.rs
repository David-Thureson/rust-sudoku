#![allow(dead_code)]

// Differences from builder_vec_large:
// - Custom constraints such as the knight or king variations on Chess Sudokup
// - Custom symbols.

use itertools::Itertools;
use bit_vec::BitVec;
// use std::collections::HashSet;
use array2d::Array2D;
// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
use std::ops::Range;
//use std::fmt::{Display, Formatter, Error};
// use std::time::Instant;
use rand::{thread_rng, Rng};

use crate::*;
use super::*;

const VERBOSE: u8 = 0;

pub fn main() {
}

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Grid {
    pub(crate) width: u8,
    pub(crate) height: u8,
    pub(crate) block_width: u8,
    pub(crate) block_height: u8,
    pub(crate) block_col_count: u8,
    pub(crate) block_row_count: u8,
    pub(crate) block_count: u8,
    pub(crate) block_cell_count: u8,
    pub(crate) cell_count: u16,
    pub(crate) unsolved_cell_count: u16,
    pub(crate) max_value: u8,
    pub(crate) max_related_cell_count: u8,
    #[derivative(Debug="ignore")]
    pub symbols: Vec<char>,
    #[derivative(Debug="ignore")]
    pub values: Vec<u8>,
    pub remaining_value_count: u32,
    #[derivative(Debug="ignore")]
    pub remaining_value_counts: Vec<u8>,
    // remaining_value_counts_map: Vec<HashSet<u8>>,
    #[derivative(Debug="ignore")]
    pub remaining_values: BitVec,
    #[derivative(Debug="ignore")]
    pub related_cell_indexes: Vec<u16>,
}

impl Grid {

    pub fn new(width: u8, height: u8, block_width: u8, block_height: u8) -> Self {
        let block_col_count = (width as f64 / block_width as f64).ceil() as u8;
        let block_row_count = (height as f64 / block_height as f64).ceil() as u8;
        let block_count = block_col_count * block_row_count;
        let block_cell_count = block_width * block_height;
        let cell_count = width as u16 * height as u16;

        let max_value = *[width, height, block_cell_count].iter().max().unwrap();

        let mut values = Vec::with_capacity(cell_count as usize);

        let remaining_value_count = cell_count as u32 * max_value as u32;

        let mut remaining_value_counts = Vec::with_capacity(cell_count as usize);
        for _ in 0..cell_count {
            values.push(NO_VALUE);
            remaining_value_counts.push(max_value);
        }

        let remaining_values = BitVec::from_elem(max_value as usize * cell_count as usize, true);

        let symbols = gen_char_array(if max_value <= 9 {
            SYMBOLS_STANDARD
        } else {
            SYMBOLS_EXTENDED
        });
        //bg!(&symbols);

        let grid = Self {
            width,
            height,
            block_width,
            block_height,
            block_col_count,
            block_row_count,
            block_count,
            block_cell_count,
            cell_count,
            unsolved_cell_count: cell_count,
            max_value,
            max_related_cell_count: 0,
            symbols,
            values,
            remaining_value_count,
            remaining_value_counts,
            // remaining_value_counts_map,
            remaining_values,
            related_cell_indexes: vec![],
        };
        if VERBOSE >= 1 { dbg!(&grid); }
        if RUN_INVARIANT { grid.invariant(); }
        grid
    }

    pub fn symbols(mut self, symbols: &str) -> Self {
        if RUN_INVARIANT { self.invariant(); }
        self.symbols = gen_char_array(symbols);
        if RUN_INVARIANT { self.invariant(); }
        self
    }

    #[inline]
    pub(crate) fn row_col_block(&self, index: u16) -> (u8, u8, u8) {
        if RUN_INVARIANT { self.invariant(); }
        let row = index / self.width as u16;
        let col = index % self.width as u16;
        let block = ((row / self.block_height as u16) * self.block_col_count as u16) + (col / self.block_width as u16);
        (row as u8, col as u8, block as u8)
    }

    #[inline]
    pub(crate) fn set_value(&mut self, index: u16, value: u8) {
        if RUN_INVARIANT { self.invariant(); }
        let current_value = self.values[index as usize];
        if value == current_value {
            return;
        }
        self.values[index as usize] = value;
        if current_value == NO_VALUE && value != NO_VALUE {
            self.unsolved_cell_count -= 1;
        } else if current_value != NO_VALUE && value == NO_VALUE {
            self.unsolved_cell_count += 1;
        }
        self.recalc_remaining_values_one_cell(index);
        for related_cell_index in self.index_to_related_cell_indexes(index) {
            self.recalc_remaining_values_one_cell(related_cell_index);
        }
        if RUN_INVARIANT { self.invariant(); }
    }

    #[inline]
    pub fn remove_cells(&mut self, remove_cell_count: u16) {
        if RUN_INVARIANT { self.invariant(); }
        let mut indexes = (0..self.cell_count)
            .filter(|index| self.values[*index as usize] != NO_VALUE)
            .collect::<Vec<_>>();
        let remove_cell_count = remove_cell_count.min(indexes.len() as u16);
        for _ in 0..remove_cell_count.min(self.cell_count) {
            let index = indexes.remove(thread_rng().gen_range(0, indexes.len()));
            self.values[index as usize] = NO_VALUE;
            self.unsolved_cell_count += 1;
        }
        self.recalc_remaining_values();
        if RUN_INVARIANT { self.invariant(); }
    }

    #[inline]
    fn recalc_remaining_values(&mut self) {
        for index in 0..self.cell_count {
            self.recalc_remaining_values_one_cell(index);
        }
    }

    #[inline]
    fn recalc_remaining_values_one_cell(&mut self, index: u16) {
        if self.values[index as usize] == NO_VALUE {
            let related_cell_values = self.index_to_related_cell_indexes(index)
                .iter()
                .map(|related_cell_index| self.values[*related_cell_index as usize])
                .collect::<HashSet<_>>();
            for value in 1..=self.max_value {
                if related_cell_values.contains(&value) {
                    self.clear_remaining_value(index, value);
                } else {
                    self.set_remaining_value(index, value);
                }
            }
        } else {
            self.clear_remaining_values(index);
        }
    }

    #[inline]
    fn clear_remaining_values(&mut self, index: u16) {
        // if RUN_INVARIANT { self.invariant(); }
        for value in 1..=self.max_value {
            self.clear_remaining_value(index, value);
        }
    }

    #[inline]
    pub fn has_remaining_value(&self, index: u16, value: u8) -> bool {
        let remaining_value_index = self.remaining_value_index(index, value);
        self.remaining_values[remaining_value_index]
    }

    #[inline]
    pub fn set_remaining_value(&mut self, index: u16, value: u8) -> bool {
        // Returns true if the remaining value was not already in the cell and thus was added.
        let remaining_value_index = self.remaining_value_index(index, value);
        if !self.remaining_values[remaining_value_index] {
            self.remaining_values.set(remaining_value_index, true);
            self.remaining_value_counts[index as usize] += 1;
            self.remaining_value_count += 1;
            if RUN_INVARIANT { self.invariant(); }
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn clear_remaining_value(&mut self, index: u16, value: u8) -> bool {
        // Returns true if the remaining value was in the cell and thus was removed.
        // Don't run the invariant here because this function will be called after setting the
        // value for some other cell. Thus the current cell may have extra remaining values.
        let remaining_value_index = self.remaining_value_index(index, value);
        if self.remaining_values[remaining_value_index] {
            self.remaining_values.set(remaining_value_index, false);
            self.remaining_value_counts[index as usize] -= 1;
            self.remaining_value_count -= 1;
            true
        } else {
            false
        }
    }

    #[inline]
    fn remaining_value_index(&self, index: u16, value: u8) -> usize {
        ((index as usize * self.max_value as usize) + value as usize) - 1
    }

    #[inline]
    pub fn one_remaining_value(&self, index: u16) -> u8 {
        if RUN_INVARIANT { self.invariant(); }
        debug_assert_eq!(1, self.remaining_value_counts[index as usize]);

        // Within the overall self.remaining_values vector there is a range dedicated to this cell.
        // It has one boolean entry for each possible value.
        let remaining_value_range = self.remaining_value_range(index);
        let range_start = remaining_value_range.start;
        for remaining_value_index in remaining_value_range {
            if self.remaining_values[remaining_value_index] {
                // This cell has this value marked as being a possible value.
                let value = (remaining_value_index - range_start) + 1;
                return value as u8
            }
        }
        unreachable!()
    }

    #[inline]
    fn block_row_index(&self, row: u8) -> u8 {
        row / self.block_height
    }

    #[inline]
    fn block_col_index(&self, col: u8) -> u8 {
        col / self.block_width
    }

    #[inline]
    pub fn solved_cell_count(&self) -> u16 {
        self.cell_count - self.unsolved_cell_count
    }

    #[inline]
    pub(crate) fn remaining_values(&self, index: u16) -> Vec<u8> {
        if RUN_INVARIANT { self.invariant(); }
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
    pub(crate) fn remaining_value_range(&self, index: u16) -> Range<usize> {
        let start_index = index as usize * self.max_value as usize;
        let end_index = start_index + self.max_value as usize;
        Range { start: start_index, end: end_index }
    }

    #[inline]
    pub fn related_cell_range(&self, index: u16) -> Range<usize> {
        // This is called from within the invariant, so don't call the invariant here.
        let start_index = index as usize * self.max_related_cell_count as usize;
        let end_index = start_index + self.max_related_cell_count as usize;
        let range = Range { start: start_index, end: end_index };
        //bg!(index, &range);
        range
    }

    pub fn partial_grid_from_indexes(&self, indexes: &[u16], include: bool) -> Self {
        // If include is true, this is a list of indexes of cells to keep, that is to give a value
        // in the new grid taken from the old grid. If include is false, this is a list of indexes
        // to exclude meaning their corresponding cells will be left empty while the others will be
        // filled.
        if RUN_INVARIANT { self.invariant(); }
        let mut grid = self.clone_empty();
        for index in 0..self.cell_count {
            let in_list = indexes.contains(&index);
            if include == in_list {
                let index = index as usize;
                grid.values[index] = self.values[index];
            }
        }
        grid.unsolved_cell_count = if include {
            grid.cell_count - indexes.len() as u16
        } else {
            indexes.len() as u16
        };
        grid.recalc_remaining_values();
        if RUN_INVARIANT { grid.invariant(); }
        grid
    }

    pub fn clone_empty(&self) -> Self {
        if RUN_INVARIANT { self.invariant(); }
        let mut grid = Self::new(self.width, self.height, self.block_width, self.block_height);
        //bg!(&self.symbols, &self.related_cell_indexes);
        grid.symbols = self.symbols.clone();
        grid.related_cell_indexes = self.related_cell_indexes.clone();
        grid.max_related_cell_count = self.max_related_cell_count;
        if RUN_INVARIANT { grid.invariant(); }
        grid
    }

    pub fn clone_with_value_list(&self, values: Vec<u8>) -> Self {
        debug_assert!(values.len() == self.cell_count as usize);
        let mut grid = self.clone_empty();
        grid.values = values;
        grid.recalc_remaining_values();
        if RUN_INVARIANT { grid.invariant(); }
        grid
    }

    pub fn replace_values(&mut self, values: &[u8]) {
        let value_count = values.len() as u16;
        debug_assert!(value_count == self.cell_count);
        self.unsolved_cell_count = values
            .iter()
            .filter(|value| **value == NO_VALUE)
            .count() as u16;
        self.values = values.to_vec();
        self.recalc_remaining_values();
        if RUN_INVARIANT { self.invariant(); }
    }

    #[inline]
    pub(crate) fn index_to_related_cell_indexes(&self, index: u16) -> Vec<u16> {
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

    pub fn print_simple(&self, label: &str) {
        // Don't run the invariant because we might be displaying the grid to illustrate something
        // that has broken the invariant.
        self.print_simple_and_remaining_internal(label, false, false);
    }

    pub fn print_simple_and_remaining_counts(&self, label: &str) {
        // Don't run the invariant because we might be displaying the grid to illustrate something
        // that has broken the invariant.
        self.print_simple_and_remaining_internal(label, true, false);
    }

    pub fn print_simple_and_remaining(&self, label: &str) {
        // Don't run the invariant because we might be displaying the grid to illustrate something
        // that has broken the invariant.
        self.print_simple_and_remaining_internal(label, true, true);
    }

    fn print_simple_and_remaining_internal(&self, label: &str, print_remaining_counts: bool, print_remaining: bool) {
        let cell_row_padding: usize = 0;
        let cell_col_padding: usize = 2;
        let block_row_padding: usize = 1;
        let block_col_padding: usize = 3;
        let grid_col_padding: usize = 5;
        let num_rows: usize = self.height as usize + (cell_row_padding as usize * (self.height as usize - 1)) + (block_row_padding * (self.block_row_count as usize - 1));
        let num_cols_one_grid: usize = self.width as usize + (cell_col_padding * (self.width as usize - 1)) + (block_col_padding * (self.block_col_count as usize - 1));
        let num_cols: usize = if print_remaining_counts {
            (num_cols_one_grid * 2) + grid_col_padding
        } else {
            num_cols_one_grid
        };
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows as usize, num_cols as usize);
        for index in 0..self.cell_count as usize {
            let (row, col, _block) = row_col_block(index as u16, self.width, self.block_width, self.block_height, self.block_col_count);
            let x: usize = (col as usize * cell_col_padding) + (self.block_col_index(col) as usize * block_col_padding) + col as usize;
            let y: usize = (row as usize * cell_row_padding) + (self.block_row_index(row) as usize * block_row_padding) + row as usize;
            let value = self.values[index];
            let value = self.get_symbol(value).to_string();
            ar.set(y, x, value).unwrap();
            if print_remaining_counts {
                let x = x + num_cols_one_grid + grid_col_padding;
                let value = self.remaining_value_counts[index];
                let value = self.get_symbol(value as u8).to_string();
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
        let num_rows: usize = (cell_height * self.height as usize) + (cell_row_padding as usize * (self.height as usize - 1)) + (block_row_padding * (self.block_row_count as usize - 1));
        let num_cols: usize = (cell_width * self.width as usize) + (cell_col_padding * (self.width as usize - 1)) + (block_col_padding * (self.block_col_count as usize - 1));
        //bg!(self.grid_width, self.block_col_count, cell_width, cell_col_padding, block_col_padding, num_cols);
        // let num_rows = num_rows * 2;
        // let num_cols = num_cols * 2;
        let mut ar = Array2D::filled_with(" ".to_string(), num_rows as usize, num_cols as usize);
        for index in 0..self.cell_count as usize {
            let (row, col, _block) = row_col_block(index as u16, self.width, self.block_width, self.block_height, self.block_col_count);
            let cell_x: usize = (col as usize * (cell_width + cell_col_padding)) + (self.block_col_index(col) as usize * block_col_padding) as usize;
            let cell_y: usize = (row as usize * (cell_height + cell_row_padding)) + (self.block_row_index(row) as usize * block_row_padding) as usize;
            let range = self.remaining_value_range(index as u16);
            let range_start = range.start;
            let cell_value = self.values[index];
            let internal_values = if cell_value > 0 {
                completed_template.replace("#", &self.get_symbol(cell_value).to_string())
            } else {
                let mut s = "".to_string();
                for remaining_value_index in range {
                    let offset = remaining_value_index - range_start;
                    let value = if self.remaining_values[remaining_value_index] {
                        offset as u8 + 1
                    } else {
                        0
                    };
                    let value = self.get_symbol(value);
                    s.push(value);
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

    pub fn debug_cell_and_related(&self, label: &str, index: u16) {
        self.print_simple_and_remaining(label);
        println!("cell and related:\n\t{}", self.cell_display(index));
        for related_cell_lookup_index in self.related_cell_range(index) {
            let related_cell_index = self.related_cell_indexes[related_cell_lookup_index];
            if related_cell_index != index {
                println!("\t\t{}", self.cell_display(related_cell_index as u16));
            }
        }
    }

    #[inline]
    fn get_symbol(&self, value: u8) -> char {
        if value == NO_VALUE {
            SYMBOL_NO_VALUE
        } else {
            self.symbols[value as usize - 1]
        }
    }

    pub fn invariant(&self) {
        self.invariant_for_builder(&self.symbols, &self.related_cell_indexes)
    }

    pub fn invariant_for_builder(&self, symbols: &[char], related_cell_indexes: &[u16]) {
        // This version of the invariant allows the builder to check the invariant even though the
        // grid doesn't have its own copy of the symbols and related cell indexes yet.

        assert!(self.width > 0);

        assert!(self.height > 0);

        assert!(self.block_width > 0);
        assert!(self.width >= self.block_width);

        assert!(self.block_height > 0);
        assert!(self.height >= self.block_height);

        assert!(self.block_col_count > 0);
        assert_eq!(self.width, self.block_width * self.block_col_count);

        assert!(self.block_row_count > 0);
        assert_eq!(self.height, self.block_height * self.block_row_count);

        assert!(self.block_cell_count > 0);
        assert_eq!(self.block_cell_count, self.block_width * self.block_height);

        assert!(self.cell_count > 0);
        assert_eq!(self.cell_count, self.width as u16* self.height as u16);

        assert!(self.unsolved_cell_count <= self.cell_count);
        let calc_unsolved_cell_count = (0..self.cell_count)
            .filter(|index| self.values[*index as usize] == NO_VALUE)
            .count();
        assert_eq!(calc_unsolved_cell_count, self.unsolved_cell_count as usize);

        assert!(self.max_value > 0);

        if !related_cell_indexes.is_empty() {
            assert!(self.max_related_cell_count > 0);
        }

        if !symbols.is_empty() {
            // Note that we're using the argument symbols rather than the field of the same name. See
            // notes at the top of the function.
            assert!(symbols.len() >= self.max_value as usize);
            // Every symbol should be unique.
            assert_eq!(symbols.len(), symbols.iter().unique().count());
        }

        assert_eq!(self.cell_count as usize, self.values.len());

        assert_eq!(self.cell_count as usize, self.remaining_value_counts.len());
        let calc_remaining_value_count: u32 = self.remaining_value_counts.iter().map(|x| *x as u32).sum::<u32>();
        assert_eq!(self.remaining_value_count, calc_remaining_value_count);

        let calc_remaining_value_bit_count = self.cell_count as usize * self.max_value as usize;
        assert_eq!(calc_remaining_value_bit_count, self.remaining_values.len());

        if !related_cell_indexes.is_empty() {
            let calc_related_cells_total_count = self.cell_count as usize * self.max_related_cell_count as usize;
            // Note that we're using the argument related_cell_indexes rather than the field
            // of the same name. See notes at the top of the function.
            assert_eq!(calc_related_cells_total_count, related_cell_indexes.len());
        }

        for index in 0..self.cell_count as usize {
            let value = self.values[index];
            assert!(value <= self.max_value);

            let remaining_value_count = self.remaining_value_counts[index];
            assert!(remaining_value_count <= self.max_value);

            {
                let mut calc_remaining_value_count = 0;
                let start_index = index as usize * self.max_value as usize;
                let end_index = start_index + self.max_value as usize;
                for remaining_value_index in start_index..end_index {
                    if self.remaining_values[remaining_value_index] {
                        calc_remaining_value_count += 1;
                    }
                }
                assert_eq!(calc_remaining_value_count, remaining_value_count);
            }

            if value == NO_VALUE {
                // The cell has not been filled so there should be at least one possible value.
                // if remaining_value_count == 0 {
                //     let error_message = format!("{} has not been filled so there should be at least one possible value.", self.cell_display_for_invariant(index as u16));
                //     self.print_simple_and_remaining(&error_message);
                //    panic!(error_message);
                //}
                // assert!(remaining_value_count >= 1);
            } else {
                // The cell has a value so there should not be any remaining possible values.
                assert_eq!(0, remaining_value_count);
                if !related_cell_indexes.is_empty() {
                    // Make sure no related cell has this cell's value either as its own value or as
                    // one of its potential values.
                    let start_index = index as usize * self.max_related_cell_count as usize;
                    let end_index = start_index + self.max_related_cell_count as usize;
                    for related_cell_lookup_index in start_index..end_index {
                        // Note that we're using the argument related_cell_indexes rather than the field
                        // of the same name. See notes at the top of the function.
                        let related_cell_index = related_cell_indexes[related_cell_lookup_index] as usize;
                        if related_cell_index != index {
                            if self.values[related_cell_index] == value {
                                let error_message = format!("{} and {} have the same value {}.", self.cell_display_for_invariant(index as u16), self.cell_display_for_invariant(related_cell_index as u16), value);
                                self.print_simple_and_remaining(&error_message);
                                panic!(error_message);
                            }
                            let remaining_value_index = ((related_cell_index * self.max_value as usize) + value as usize) - 1;
                            if self.remaining_values[remaining_value_index] {
                                let error_message = format!("{} has a remaining value found in {}.", self.cell_display_for_invariant(related_cell_index as u16), self.cell_display_for_invariant(index as u16));
                                self.print_simple_and_remaining(&error_message);
                                panic!(error_message);
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn cell_display(&self, index: u16) -> String {
        let value = self.values[index as usize];
        let values_string = if value == NO_VALUE {
            if self.related_cell_indexes.is_empty() {
                "".to_string()
            } else {
                let remaining_values_string = self.remaining_values(index).iter().map(|value| self.get_symbol(*value).to_string()).collect::<Vec<_>>().join("");
                format!("remaining_values = [{}]", remaining_values_string)
            }
        } else {
            format!("value = {}", self.get_symbol(value))
        };
        let related_indexes_string = if self.related_cell_indexes.is_empty() {
            "".to_string()
        } else {
            format!("related_indexes = [{}]", self.index_to_related_cell_indexes(index).iter().join(", "))
        };
        let (row, col, block) = row_col_block(index, self.width, self.block_width, self.block_height, self.block_col_count);
        format!("cell index {} r {} c {} b {}: {}, {}", index, row + 1, col + 1, block + 1, values_string, related_indexes_string)
    }

    fn cell_display_for_invariant(&self, index: u16) -> String {
        // Don't make any other calls.
        let value = self.values[index as usize];
        let symbol = if value == NO_VALUE {
            SYMBOL_NO_VALUE
        } else {
            self.symbols[value as usize - 1]
        };
        let value_string = format!("value = {}", symbol);
        let (row, col, block) = row_col_block(index, self.width, self.block_width, self.block_height, self.block_col_count);
        format!("cell index {} r {} c {} b {}: {}", index, row + 1, col + 1, block + 1, value_string)
    }

}

fn row_col_block(index: u16, grid_width: u8, block_width: u8, block_height: u8, block_col_count: u8) -> (u8, u8, u8) {
    let row = index / grid_width as u16;
    let col = index % grid_width as u16;
    let block = ((row / block_height as u16) * block_col_count as u16) + (col / block_width as u16);
    (row as u8, col as u8, block as u8)
}

