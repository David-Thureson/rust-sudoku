#![allow(dead_code)]

// Differences from builder_vec_large:
// - Custom constraints such as the knight or king variations on Chess Sudokup
// - Custom symbols.
// This goes with grid.

use rand::{thread_rng, Rng};
use itertools::Itertools;
// use bit_vec::BitVec;
// use std::sync::Mutex;
// use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
//use std::fmt::{Display, Formatter, Error};
use std::time::Instant;

use crate::*;
use crate::grid_constraint::grid::Grid;

const RUN_INVARIANT: bool = false;
const VERBOSE: u8 = 0;
const LOG_LEVEL: u8 = 2;
// https://emojipedia.org/

type Rule = &'static dyn Fn(&Grid, &Cell, &Cell) -> bool;
const RULE_ROW: Rule = &|_grid, cell_1, cell_2| cell_1.row == cell_2.row;
const RULE_COLUMN: Rule = &|_grid, cell_1, cell_2| cell_1.column == cell_2.column;
const RULE_BLOCK: Rule = &|_grid, cell_1, cell_2| cell_1.block == cell_2.block;
const RULE_KING: Rule = &|_grid, cell_1, cell_2| cell_1.row_distance(cell_2) <= 1 && cell_1.column_distance(cell_2) <= 1;
const RULE_BISHOP: Rule = &|_grid, cell_1, cell_2| cell_1.row_distance(cell_2) == cell_1.column_distance(cell_2);
const RULE_BISHOP_2: Rule = &|_grid, cell_1, cell_2| {
    let row_distance = cell_1.row_distance(cell_2);
    let column_distance = cell_1.column_distance(cell_2);
    row_distance == column_distance && row_distance <= 2
};
const RULE_DIAGONALS: Rule = &|grid, cell_1, cell_2| {
    (cell_1.row == cell_1.column && cell_2.row == cell_2.column)
    || (cell_1.row == cell_1.column_from_end(grid) && cell_2.row == cell_2.column_from_end(grid))
};
const RULE_KNIGHT: Rule = &|_grid, cell_1, cell_2| {
    let row_distance = cell_1.row_distance(cell_2);
    let column_distance = cell_1.column_distance(cell_2);
    (row_distance == 2 && column_distance == 1) || (row_distance == 1 && column_distance == 2)
};

pub fn main() {
    try_build();
}

fn try_build() {
    // let max_grid_count = Some(0);
    // let max_grid_count = None;
    let limit_msec = 100_000;
    // let symbols = SYMBOLS_STANDARD;

    let repeat_count = 5;
    // let max_tried_grid_count = Some(0);
    // let duration_limit = Some(Duration::from_millis(1_000));

    let mut builders = vec![];

    // for grid_size in 1..=36 {
    //     grids.push(Grid::with_size(grid_size));
    // }

    let builder = Builder::with_size(9).limit_milliseconds(limit_msec);
    builders.push(builder);
    // builders.push(Builder::with_size(9).limit_milliseconds(limit_msec).rule(RULE_DIAGONALS).rule(RULE_KNIGHT));//.rule(RULE_KING));//.rule(RULE_BISHOP_2));//.clear_rules().rule(RULE_BISHOP).rule(RULE_BLOCK));
    // builders.push(Builder::with_size(16).limit_milliseconds(limit_msec).rule(RULE_KING).rule(RULE_KNIGHT).rule(RULE_DIAGONALS));//.rule(RULE_KING));//.rule(RULE_BISHOP_2));//.clear_rules().rule(RULE_BISHOP).rule(RULE_BLOCK));

    for builder in builders.iter_mut() {
        //bg!(&builder);
        for _ in 0..repeat_count {
            let result = builder.build();
            match result {
                Ok(grid) => {
                    //bg!(&grid);
                    grid.print_simple("");

                    /*
                    for _ in 0..15 {
                        grid.remove_cells(6);
                        grid.print_simple_and_remaining("");
                    }
                    */

                },
                Err(message) => println!("{}", message),
            }
        }
        dbg!(&builder);
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Builder {
    pub width: u8,
    pub height: u8,
    pub block_width: u8,
    pub block_height: u8,
    pub max_tried_grid_count: Option<usize>,
    pub symbols: Vec<char>,
    pub time_limit: Option<Duration>,
    pub cell_limit: Option<u16>,
    #[derivative(Debug = "ignore")]
    // pub related_cell_predicates: Vec<Box<dyn Fn(&Cell, &Cell) -> bool>>,
    pub related_cell_predicates: Vec<Rule>,
    #[derivative(Debug="ignore")]
    related_cell_indexes: Vec<u16>,
    build_runs: Vec<BuildRun>,
}

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct BuildRun {
    #[derivative(Debug="ignore")]
    pub success: Option<bool>,
    pub failure_message: Option<String>,
    pub time_start: Instant,
    #[derivative(Debug="ignore")]
    pub time_end: Option<Instant>,
    pub duration: Option<Duration>,
    pub fill_next_cell_count: usize,
    pub try_value_count: usize,
    pub set_cell_value_count: usize,
    pub set_one_remaining_value_count: usize,
    pub tried_grid_registered_count: usize,
    pub tried_grid_skipped_count: usize,
    pub tried_grid_found_count: usize,
    #[derivative(Debug="ignore")]
    pub filled_cell_counts: Vec<u16>,
    #[derivative(Debug="ignore")]
    pub tried_grids: HashSet<u64>,
    pub continue_build: bool,
    // pub continue_branch: bool,
    pub branch_sizes: Vec<u32>,
    #[derivative(Debug="ignore")]
    pub grid: Option<Grid>,
}

#[derive(Debug)]
pub struct Cell {
    pub index: u16,
    pub row: i8,
    pub column: i8,
    pub block: i8,
}

/*
struct FailedGridSet {
    hashes: HashSet<u64>,
}

#[derive(Hash)]
struct FailedGrid {
    values: Vec<(u8, u8)>,
}
*/

impl Builder {

    pub fn with_size(size: u8) -> Self {
        let (block_width, block_height) = match size {
            1 | 2 | 3 => (1, 1),
            4 | 5 => (2, 2),
            6 | 7 => (3, 2),
            8 => (4, 2),
            9 => (3, 3),
            10 | 11 => (5, 2),
            12 | 13 | 14 => (4, 3),
            15 => (5, 3),
            16 | 17 => (4, 4),
            18 | 19 => (6, 3),
            20 => (5, 4),
            21 | 22 | 23 => (7, 3),
            24 => (6, 4),
            25 | 26 | 27 => (5, 5),
            28 | 29 => (7, 4),
            30 | 31 => (6, 5),
            32 | 33 | 34 => (8, 4),
            35 => (7, 5),
            36 => (6, 6),
            _ => panic!("Unexpected grid_size = {}", size)
        };
        Self::new(size, size, block_width, block_height)
    }

    pub fn with_block_size(block_size: u8) -> Self {
        Self::with_block(block_size, block_size)
    }

    pub fn with_block(block_width: u8, block_height: u8) -> Self {
        let grid_width = block_width * block_height;
        let grid_height = grid_width;
        Self::new(grid_width, grid_height, block_width, block_height)
    }

    pub fn new(width: u8, height: u8, block_width: u8, block_height: u8) -> Self {
        /*
        let related_cell_predicates: Vec<Box<dyn Fn(&Cell, &Cell) -> bool>> = vec![
            Box::new(|cell_1, cell_2| cell_1.row == cell_2.row),
            Box::new(|cell_1, cell_2| cell_1.column == cell_2.column),
            Box::new(|cell_1, cell_2| cell_1.block == cell_2.block),
        ];
        */
        let related_cell_predicates= vec![RULE_ROW, RULE_COLUMN, RULE_BLOCK];

        let builder = Self {
            width,
            height,
            block_width,
            block_height,
            max_tried_grid_count: None,
            symbols: vec![],
            time_limit: None,
            cell_limit: None,
            related_cell_predicates,
            related_cell_indexes: vec![],
            build_runs: vec![],
        };
        if VERBOSE >= 1 { dbg!(&builder); }
        builder
    }

    pub fn max_tried_grid_count(mut self, max_tried_grid_count: usize) -> Self {
        self.max_tried_grid_count = Some(max_tried_grid_count);
        self
    }

    pub fn symbols(mut self, symbols: &str) -> Self {
        self.symbols = gen_char_array(symbols);
        self
    }

    pub fn limit_seconds(mut self, seconds: u64) -> Self {
        self.time_limit = Some(Duration::from_secs(seconds));
        self
    }

    pub fn limit_milliseconds(mut self, msec: u64) -> Self {
        self.time_limit = Some(Duration::from_millis(msec));
        self
    }

    pub fn cell_limit(mut self, cell_limit: u16) -> Self {
        self.cell_limit = Some(cell_limit);
        self
    }

    pub fn clear_rules(mut self) -> Self {
        self.related_cell_predicates.clear();
        self
    }

    pub fn rule(mut self, rule: Rule) -> Self {
        self.related_cell_predicates.push(rule);
        self
    }

    pub fn build(&mut self) -> Result<Grid, String> {

        let mut grid = Grid::new(self.width, self.height, self.block_width, self.block_height);
        self.set_up_related_cells(&mut grid);

        let mut build_run = BuildRun::new(grid.max_value);

        if self.symbols.is_empty() {
            self.symbols = gen_char_array(if grid.max_value <= 9 {
                SYMBOLS_STANDARD
            } else {
                SYMBOLS_EXTENDED
            });
        }

        self.build_next_cell(&mut build_run, &grid);
        let now = Instant::now();
        build_run.time_end = Some(now);
        let duration = now - build_run.time_start;
        build_run.duration = Some(duration);
        self.build_runs.push(build_run.clone());
        if build_run.success.unwrap() {
            let grid = build_run.grid.unwrap().clone();
            if SHOW_ELAPSED_TIME { dbg!(duration); }
            grid.invariant_for_builder(&self.symbols, &self.related_cell_indexes);
            Ok(grid)
        } else {
            Err(build_run.failure_message.unwrap().clone())
        }

    }

    fn build_next_cell(&self, build_run: &mut BuildRun, grid_to_now: &Grid) {
        build_run.fill_next_cell_count += 1;
        if !build_run.check_continue(self.time_limit) {
            //rintln!("build_next_cell: return a");
            return;
        }

        if VERBOSE >= 1 {
            let label = "build_next_cell() top";
            grid_to_now.print_simple_and_remaining(label);
        }

        let try_cell_index = Self::choose_try_cell_index(grid_to_now);

        let mut try_values = grid_to_now.remaining_values(try_cell_index);
        //bg!(try_cell_index, &try_values);
        let mut branch_size = 0u8;
        while !try_values.is_empty() {
            // let try_value = try_values.remove(thread_rng().gen_range(0, try_values.len()));
            let try_value = try_values.remove(0);
            branch_size += 1;
            build_run.branch_sizes[branch_size as usize - 1] += 1;

            build_run.try_value_count += 1;
            if !build_run.check_continue(self.time_limit) {
                //rintln!("build_next_cell: return b");
                return;
            }

            // Register the grid we're about to try, and if this call returns false it means
            // we've already tried this grid.
            // if self.register_attempt(build_run, grid_to_now, try_cell_index, try_value) {
                let mut try_grid = grid_to_now.clone();
                //rintln!("\tOption is {} with value {}", &grid_to_now.cell(option.0), option.1);
                let set_value_ok = self.set_value(build_run, &mut try_grid, try_cell_index, try_value);
                if !build_run.check_continue(self.time_limit) {
                    // Most likely the last cell was filled in during the call to set_value().
                    //rintln!("build_next_cell: return c");
                    return;
                }
                if set_value_ok {
                    // This option worked so keep going.
                    //rintln!("\t\tOption worked, continuing.");
                    self.build_next_cell(build_run, &try_grid);
                    if !build_run.check_continue(self.time_limit) {
                        // Most likely the last cell was filled in during the call to build_next_cell().
                        //rintln!("build_next_cell: return d");
                        return;
                    }
                //}
            }
        }
        // Given grid_to_now as the starting point, we tried all possible values in all remaining
        // cells and nothing worked, so fall back to an earlier version of the grid.
        //rintln!("build_next_cell: return e");
    }

    fn register_attempt(&self, build_run: &mut BuildRun, grid: &Grid, try_cell_index: u16, try_value: u8) -> bool {
        build_run.tried_grid_registered_count += 1;
        // if !build_run.check_continue(self.time_limit) {
        //     return false;
        // }

        let try_cell_index = try_cell_index as usize;
        assert!(grid.values[try_cell_index] == NO_VALUE);
        assert!(try_value >= 1);
        assert!(try_value <= grid.max_value);

        let tried_grid_count = build_run.tried_grids.len();

        if let Some(max_tried_grid_count) = self.max_tried_grid_count {
            if tried_grid_count >= max_tried_grid_count {
                build_run.tried_grid_skipped_count += 1;
                return true;
            }
        }

        // Start with a copy of the grid's values just before trying this new value.
        let mut try_values = grid.values.clone();
        // Set the new value we're about to try. The list of values including this new one forms a
        // signature for grids we don't want to attempt more than once.
        try_values[try_cell_index] = try_value;

        let mut hasher = DefaultHasher::new();
        try_values.hash(&mut hasher);
        let hash = hasher.finish();

        let is_new = {
            if build_run.tried_grids.contains(&hash) {
                // We've already tried this partial grid.
                build_run.tried_grid_found_count += 1;
                false
            } else {
                // We haven't tried this partial grid so add it to the list and give it a shot.
                build_run.tried_grids.insert(hash);
                true
            }
        };
        is_new
    }

    fn set_value(&self, build_run: &mut BuildRun, grid: &mut Grid, index: u16, value: u8) -> bool {
        build_run.set_cell_value_count += 1;
        if !build_run.check_continue(self.time_limit) {
            return false;
        }

        if VERBOSE >= 1 {
            let label = format!("\nset_value() top: index = {}, value = {}, {}", index, value, grid.cell_display(index));
            grid.debug_cell_and_related(&label, index);
        }

        assert!(grid.values[index as usize] == NO_VALUE);
        assert!(value > 0);
        assert!(value <= grid.max_value);

        grid.values[index as usize] = value;
        // let label = format!("\nset_value() after setting value: index = {}, value = {}, {}", index, value, self.cell_display(index));
        // self.debug_cell_and_related(&label, index);

        for remaining_value_index in grid.remaining_value_range(index) {
            if grid.remaining_values[remaining_value_index] {
                grid.remaining_values.set(remaining_value_index, false);
                grid.remaining_value_count -= 1;
            }
        }
        grid.remaining_value_counts[index as usize] = 0;
        grid.unsolved_cell_count -= 1;

        let reached_cell_limit = match self.cell_limit {
            Some(cell_limit) => grid.solved_cell_count() >= cell_limit,
            _ => false,
        };

        // rintln!("set_value(): filled_cell_count = {}", self.filled_cell_count());

        if grid.unsolved_cell_count > 0 {
            let related_cell_indexes = self.index_to_related_cell_indexes(grid, index);
            //bg!(&related_cell_indexes);

            let mut one_value_indexes = vec![];
            //for related_cell_lookup_index in self.related_cell_range(index) {
            //    let related_cell_index = self.related_cell_indexes[related_cell_lookup_index] as usize;
            for related_cell_index in related_cell_indexes.iter() {
                let related_cell_index = *related_cell_index as usize;
                // let label = format!("set_value() top of related cell loop: index = {}, value = {}, related_cell_index = {}, {}", index, value, related_cell_index, self.cell_display(related_cell_index as u8));
                // self.debug_cell_and_related(&label, index);

                if grid.values[related_cell_index] == NO_VALUE {
                    if grid.clear_remaining_value(related_cell_index as u16, value) {
                        match grid.remaining_value_counts[related_cell_index as usize] {
                            0 => {
                                // This empty cell has zero options left for its value so this attempt at the
                                // grid won't work.
                                return false;
                            },
                            1 => {
                                // We don't want to solve any more cells if we've reached the limit.
                                if !reached_cell_limit {
                                    // There's only one possible value left in the related cell. Add this
                                    // cell to the list of cells that have only one remaining value sa that
                                    // later in this function we can set the final value for this cell.
                                    one_value_indexes.push(related_cell_index);
                                    // self.set_one_remaining_value(related_cell_index as u8);
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }

            if RUN_INVARIANT { grid.invariant_for_builder(&self.symbols, &self.related_cell_indexes); }

            for related_cell_index in one_value_indexes {
                // It's possible that between the code a few lines above and now this cell has been
                // filled. This would happen when one call to set_one_remaining_value() calls
                // set_value() which calls set_one_remaining_value() and so on recursively, and a given
                // cell happens to be filled somewhere down in that tree of calls.
                if grid.values[related_cell_index] == NO_VALUE {
                    if !self.set_one_remaining_value(build_run, grid, related_cell_index as u16) {
                        // This partial grid won't work.
                        return false;
                    }
                    if !build_run.check_continue(self.time_limit) {
                        return false;
                    }
                }
            }
        }

        if RUN_INVARIANT { grid.invariant_for_builder(&self.symbols, &self.related_cell_indexes); }

        if grid.unsolved_cell_count == 0 || reached_cell_limit {
            //rintln!("\t\tOption worked, grid is completed.");
            build_run.success = Some(true);
            build_run.grid = Some(self.complete_grid_post_build(grid));
            return false;
        }

        true
    }

    #[inline]
    fn set_one_remaining_value(&self, build_run: &mut BuildRun, grid: &mut Grid, index: u16) -> bool {
        build_run.set_one_remaining_value_count += 1;
        if !build_run.check_continue(self.time_limit) {
            return false;
        }

        // This function should be called only when the cell is down to one remaining possible
        // value.
        debug_assert_eq!(1, grid.remaining_value_counts[index as usize]);

        if VERBOSE >= 2 {
            let label = format!("set_one_remaining_value(): index = {}, {}", index, grid.cell_display(index));
            println!("{}", label);
        }
        // self.debug_cell_and_related(&label, index);

        let value = grid.one_remaining_value(index);
        return self.set_value(build_run, grid, index, value);
    }

    #[inline]
    fn choose_try_cell_index(grid: &Grid) -> u16 {
        let min = *grid.remaining_value_counts.iter().filter(|count| **count > 0).min().unwrap();
        let found_count = grid.remaining_value_counts.iter().filter(|count| **count == min).count();
        let random_index = thread_rng().gen_range(0, found_count);
        //rintln!("choose_try_cell_index(): min = {}, found_count = {}, random_index = {}", min, found_count, random_index);

        let mut i = 0;
        for try_cell_index in grid.remaining_value_counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count == min)
            .map(|x| x.0) {
            if i == random_index {
                return try_cell_index as u16;
            }
            i += 1;
        }
        unreachable!()
    }

    fn index_to_cell(grid: &Grid, index: u16) -> Cell {
        let (row, col, block) = grid.row_col_block(index);
        Cell::new(index + 1, row as i8 + 1, col as i8 + 1, block as i8 + 1)
    }

    fn set_up_related_cells(&mut self, grid: &mut Grid) {
        let mut cells = Vec::with_capacity(grid.cell_count as usize);
        for index in 0..grid.cell_count {
            cells.push(Self::index_to_cell(grid, index));
        }
        //bg!(&cells);

        //bg!(grid_width, grid_height, block_width, block_height, block_cell_count);
        // let max_related_cell_count = (grid_width - block_width) + (grid_height - block_height) + (block_cell_count - 1);

        let mut related_cell_index_lists = Vec::with_capacity(grid.cell_count as usize);
        // cell_1_index is the zero-based index.
        for cell_1_index in 0..grid.cell_count {
            let cell_1= Self::index_to_cell(grid, cell_1_index);
            related_cell_index_lists.push(cells
                .iter()
                .enumerate()
                .filter(|(_, cell_2)| {
                    cell_1.index != cell_2.index
                        && self.related_cell_predicates
                        .iter()
                        .map(|f| f(&grid, &cell_1, cell_2))
                        .any(|is_related| is_related)
                })
                // The index here is zero-based and it's the one we want to retain.
                .map(|(index, _)| index as u16)
                .collect::<Vec<_>>()
            );
        }
        // We should have one entry in related_cell_index_lists for each cell. Every entry is a list
        // of the indexes of the related cells.
        debug_assert_eq!(grid.cell_count as usize, related_cell_index_lists.len());

        grid.max_related_cell_count = related_cell_index_lists.iter().map(|x| x.len()).max().unwrap() as u8;

        //bg!(&cells, &related_cell_index_lists, grid.max_related_cell_count);
        //anic!();

        // Each cell will have max_related_cell_count slots in related_cell_indexes.
        let related_cell_total = grid.cell_count as usize * grid.max_related_cell_count as usize;
        self.related_cell_indexes = Vec::with_capacity(related_cell_total);
        for (index, related_index_list) in related_cell_index_lists.iter().enumerate() {
            let list_size = related_index_list.len() as u8;
            for related_cell_index in related_index_list.iter() {
                self.related_cell_indexes.push(*related_cell_index);
            }
            // In some nonstandard grids it's possible that some cells will have more related cells
            // than others. We take the largest number of related cells and allocate that many slots
            // in self.related_cell_indexes so in these cases we need something to fill the empty
            // slots for some cells. to_fill_count is the number of slots to fill with a dummy
            // value. For a standard grid this will be zero.
            let to_fill_count = grid.max_related_cell_count - list_size;
            for _ in 0..to_fill_count {
                // We can't use zero because that's a legitimate index number. We use the index of
                // the cell itself because even if we forget to check at some point there is no harm
                // done.
                self.related_cell_indexes.push(index as u16);
            }
            if VERBOSE >= 2 { println!("Grid::new() bottom of related cell loop: index = {}, related_cell_indexes = [{}]", index, self.related_cell_indexes.iter().join(", ")); }
        }
        debug_assert_eq!(related_cell_total, self.related_cell_indexes.len());
    }

    fn complete_grid_post_build(&self, grid: &Grid) -> Grid {
        let mut complete_grid = grid.clone();
        complete_grid.symbols = self.symbols.clone();
        complete_grid.related_cell_indexes = self.related_cell_indexes.clone();
        complete_grid
    }

    fn print_grid_simple(&self, grid: &Grid, label: &str) {
        self.complete_grid_post_build(grid).print_simple(label);
    }

    fn print_grid_simple_and_remaining_counts(&self, grid: &Grid, label: &str) {
        self.complete_grid_post_build(grid).print_simple_and_remaining_counts(label);
    }

    fn print_grid_simple_and_remaining(&self, grid: &Grid, label: &str) {
        self.complete_grid_post_build(grid).print_simple_and_remaining(label);
    }

    fn index_to_related_cell_indexes(&self, grid: &Grid, index: u16) -> Vec<u16> {
        let mut indexes = vec![];
        for related_cell_lookup_index in grid.related_cell_range(index) {
            let related_cell_index = self.related_cell_indexes[related_cell_lookup_index];
            // In some nonstandard grids it's possible that some cells will have more related
            // cells than others. We take the largest number of related cells and allocate that many
            // slots in self.related_cell_indexes so in these cases we need something to fill the
            // empty slots for some cells. We use the index of the cell itself because even if we
            // forget to check at some point there is no harm done.
            if related_cell_index != index {
                indexes.push(related_cell_index);
            }
        }
        indexes
    }

}

impl BuildRun {
    pub fn new(max_values: u8) -> Self {
        let mut branch_sizes = Vec::with_capacity(max_values as usize);
        for _ in 0..max_values {
            branch_sizes.push(0);
        }
        Self {
            success: None,
            failure_message: None,
            time_start: Instant::now(),
            time_end: None,
            duration: None,
            fill_next_cell_count: 0,
            try_value_count: 0,
            set_cell_value_count: 0,
            set_one_remaining_value_count: 0,
            tried_grid_registered_count: 0,
            tried_grid_skipped_count: 0,
            tried_grid_found_count: 0,
            filled_cell_counts: vec![],
            tried_grids: Default::default(),
            continue_build: false,
            branch_sizes,
            grid: None
        }
    }

    pub fn check_continue(&mut self, time_limit: Option<Duration>) -> bool {
        if self.success.is_some() {
            // We already have a result, either success or failure.
            false
        } else {
            match time_limit {
                Some(time_limit) => {
                    if Instant::now() - self.time_start >= time_limit {
                        // We've gone over the time limit so cancel the build.
                        self.success = Some(false);
                        self.failure_message = Some("Exceeded time limit.".to_string());
                        false
                    } else {
                        // We haven't reached the time limit so continue the build.
                        true
                    }
                }
                _ => {
                    // No time limit was specified.
                    true
                }
            }
        }
    }
}

impl Cell {
    pub fn new(index: u16, row: i8, column: i8, block: i8) -> Self {
        debug_assert!(index > 0);
        debug_assert!(row > 0);
        debug_assert!(column > 0);
        debug_assert!(block > 0);
        Self {
            index,
            row,
            column,
            block,
        }
    }

    pub fn row_distance(&self, other: &Cell) -> i8 {
        (self.row - other.row).abs()
    }

    pub fn column_distance(&self, other: &Cell) -> i8 {
        (self.column - other.column).abs()
    }

    pub fn row_from_end(&self, grid: &Grid) -> i8 {
        let row = (grid.height as i8 - self.row) + 1;
        debug_assert!(row >= 1);
        debug_assert!(row <= grid.height as i8);
        row
    }

    pub fn column_from_end(&self, grid: &Grid) -> i8 {
        let column = (grid.width as i8 - self.column) + 1;
        debug_assert!(column >= 1);
        debug_assert!(column <= grid.width as i8);
        column
    }
}

/*
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
*/

