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

use crate::*;
use super::grid::Grid;
use super::Runner;

const RUN_INVARIANT: bool = false;
const VERBOSE: u8 = 0;
const LOG_LEVEL: u8 = 2;

const GRID_9_VALUE_COUNT: usize = 9;
const GRID_9_CELL_COUNT: usize = GRID_9_VALUE_COUNT * GRID_9_VALUE_COUNT;
// This is enough to cover a standard-rules grid. Revisit for more rules.
// const GRID_9_MAX_MAX_RELATED_CELL_COUNT: usize = 20;
// Recalculate this if the above value is changed.
// const GRID_9_RELATED_CELL_LIST_SIZE: usize = 1_620;
// Cell count times value count.
const GRID_9_REMANING_VALUE_LIST_SIZE: usize = GRID_9_CELL_COUNT * GRID_9_VALUE_COUNT;

const GRID_16_VALUE_COUNT: usize = 16;
const GRID_16_CELL_COUNT: usize = GRID_16_VALUE_COUNT * GRID_16_VALUE_COUNT;
const GRID_16_REMANING_VALUE_LIST_SIZE: usize = GRID_16_CELL_COUNT * GRID_16_VALUE_COUNT;
const GRID_25_VALUE_COUNT: usize = 25;
const GRID_25_CELL_COUNT: usize = GRID_25_VALUE_COUNT * GRID_25_VALUE_COUNT;
const GRID_25_REMANING_VALUE_LIST_SIZE: usize = GRID_25_CELL_COUNT * GRID_25_VALUE_COUNT;
const GRID_36_VALUE_COUNT: usize = 36;
const GRID_36_CELL_COUNT: usize = GRID_36_VALUE_COUNT * GRID_36_VALUE_COUNT;
const GRID_36_REMANING_VALUE_LIST_SIZE: usize = GRID_36_CELL_COUNT * GRID_36_VALUE_COUNT;
const GRID_49_VALUE_COUNT: usize = 49;
const GRID_49_CELL_COUNT: usize = GRID_49_VALUE_COUNT * GRID_49_VALUE_COUNT;
const GRID_49_REMANING_VALUE_LIST_SIZE: usize = GRID_49_CELL_COUNT * GRID_49_VALUE_COUNT;


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
    // try_build();
    // try_build_flat();
    try_build_flat_9();
    try_build_flat_usize();
    // profile_build_flat_9();
    // try_large_flat();
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

fn try_build_flat() {
    let size = 9;
    let limit_msec = 10_000;
    let repeat_count = 20;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(size).limit_milliseconds(limit_msec);
        builder.strategy = BuildStrategy::Flat;
        let result = builder.build();
        match result {
            Ok(_grid) => {
                //grid.print_simple("");
            },
            Err(message) => println!("{}", message),
        }
        //bg!(&builder);
        println!("try_build_flat(): size = {}, {}", size, builder.build_runs[0].runner.times_as_string());
    }
}

fn try_build_flat_9() {
    let size = 9;
    let limit_msec = 10_000;
    let repeat_count = 20;

    let mut builder = Builder::with_size(size).limit_milliseconds(limit_msec);
    builder.strategy = BuildStrategy::Flat9;
    for _ in 0..repeat_count {
        let result = builder.build();
        match result {
            Ok(_grid) => {
                //grid.print_simple("");
            },
            Err(message) => println!("{}", message),
        }
        //bg!(&builder);
        println!("try_build_flat_9(): size = {}, {}", size, builder.build_runs.last().unwrap().runner.times_as_string());
    }
}

fn try_build_flat_usize() {
    let size = 9;
    let limit_msec = 10_000;
    let repeat_count = 20;

    let mut builder = Builder::with_size(size).limit_milliseconds(limit_msec);
    builder.strategy = BuildStrategy::FlatUsize;
    for _ in 0..repeat_count {
        let result = builder.build();
        match result {
            Ok(_grid) => {
                //grid.print_simple("");
            },
            Err(message) => println!("{}", message),
        }
        //bg!(&builder);
        println!("try_build_flat_usize(): size = {}, {}", size, builder.build_runs.last().unwrap().runner.times_as_string());
    }
}

fn try_large_flat() {
    let repeat_count = 10;

    for grid_size in [9, 16, 25, 36, 49].iter() {
        let mut builder = Builder::with_size(*grid_size);
        builder.strategy = match grid_size {
            9 => BuildStrategy::Flat9,
            16 => BuildStrategy::Flat16,
            25 => BuildStrategy::Flat25,
            49 => BuildStrategy::Flat49,
            _ => panic!("Unexpected grid_size = {}", grid_size)
        };
        for repeat in 1..=repeat_count {
            let result = builder.build();
            match result {
                Ok(_grid) => {
                    //grid.print_simple("");
                },
                Err(message) => println!("{}", message),
            }
            //bg!(&builder);
            println!("try_build_flat: size = {}, {}", grid_size, builder.build_runs.last().unwrap().runner.times_as_string());
            if repeat == repeat_count {
                builder.build_runs.last().unwrap().grid.as_ref().unwrap().print_simple_and_remaining_counts("");
            }
        }
    }
}

fn profile_build_flat_9() {
    let grid_size = 9;
    let mut builder = Builder::with_size(grid_size);
    builder.strategy = BuildStrategy::Flat9;
    let _result = builder.build();
}

#[derive(Debug, Clone)]
pub enum BuildStrategy {
    NextCell,
    Flat,
    FlatUsize,
    Flat9,
    Flat16,
    Flat25,
    Flat36,
    Flat49,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Builder {
    pub strategy: BuildStrategy,
    pub width: u8,
    pub height: u8,
    pub block_width: u8,
    pub block_height: u8,
    pub max_tried_grid_count: Option<usize>,
    pub symbols: Vec<char>,
    pub time_limit: Option<Duration>,
    pub cell_limit: Option<u16>,
    #[derivative(Debug = "ignore")]
    pub related_cell_predicates: Vec<Rule>,
    #[derivative(Debug="ignore")]
    related_cell_indexes: Vec<u16>,
    #[derivative(Debug="ignore")]
    //fixed_9_related_cell_indexes: Option<[usize; GRID_9_RELATED_CELL_LIST_SIZE]>,
    fixed_related_cell_indexes: Vec<usize>,
    build_runs: Vec<BuildRun>,
}

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct BuildRun {
    pub runner: Runner,
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
            strategy: BuildStrategy::NextCell,
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
            fixed_related_cell_indexes: vec![],
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

        match self.strategy {
            BuildStrategy::FlatUsize | BuildStrategy::Flat9 | BuildStrategy::Flat16 | BuildStrategy::Flat25 | BuildStrategy::Flat36 | BuildStrategy::Flat49 => {
                if self.fixed_related_cell_indexes.is_empty() {
                    self.set_up_fixed_related_cell_indexes();
                }
            },
            _ => {},
        }

        let mut build_run = BuildRun::new(self.time_limit, grid.max_value);

        if self.symbols.is_empty() {
            self.symbols = gen_char_array(if grid.max_value <= 9 {
                SYMBOLS_STANDARD
            } else {
                SYMBOLS_EXTENDED
            });
        }

        match self.strategy {
            BuildStrategy::NextCell => {
                self.build_next_cell(&mut build_run, &grid);
            },
            BuildStrategy::Flat => {
                self.build_flat(&mut build_run, &grid);
            },
            BuildStrategy::FlatUsize => {
                self.build_flat_usize(&mut build_run, &grid);
            },
            BuildStrategy::Flat9 => {
                self.build_flat_9(&mut build_run, &grid);
            },
            BuildStrategy::Flat16 => {
                self.build_flat_16(&mut build_run, &grid);
            },
            BuildStrategy::Flat25 => {
                self.build_flat_25(&mut build_run, &grid);
            },
            BuildStrategy::Flat36 => {
                self.build_flat_36(&mut build_run, &grid);
            },
            BuildStrategy::Flat49 => {
                self.build_flat_49(&mut build_run, &grid);
            },
        }

        build_run.runner.mark_end();
        self.build_runs.push(build_run.clone());
        if build_run.runner.success.unwrap() {
            let grid = build_run.grid.unwrap().clone();
            if SHOW_ELAPSED_TIME { dbg!(build_run.runner.time); }
            grid.invariant();
            Ok(grid)
        } else {
            Err(build_run.runner.failure_message.unwrap().clone())
        }

    }

    fn build_next_cell(&self, build_run: &mut BuildRun, grid_to_now: &Grid) {
        build_run.fill_next_cell_count += 1;
        if !build_run.runner.check_continue() {
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
            if !build_run.runner.check_continue() {
                //rintln!("build_next_cell: return b");
                return;
            }

            // Register the grid we're about to try, and if this call returns false it means
            // we've already tried this grid.
            // if self.register_attempt(build_run, grid_to_now, try_cell_index, try_value) {
            let mut try_grid = grid_to_now.clone();
            //rintln!("\tOption is {} with value {}", &grid_to_now.cell(option.0), option.1);
            let set_value_ok = self.set_value(build_run, &mut try_grid, try_cell_index, try_value);
            if !build_run.runner.check_continue() {
                // Most likely the last cell was filled in during the call to set_value().
                //rintln!("build_next_cell: return c");
                return;
            }
            if set_value_ok {
                // This option worked so keep going.
                //rintln!("\t\tOption worked, continuing.");
                self.build_next_cell(build_run, &try_grid);
                if !build_run.runner.check_continue() {
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
        if !build_run.runner.check_continue() {
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
                    if !build_run.runner.check_continue() {
                        return false;
                    }
                }
            }
        }

        if RUN_INVARIANT { grid.invariant_for_builder(&self.symbols, &self.related_cell_indexes); }

        if grid.unsolved_cell_count == 0 || reached_cell_limit {
            //rintln!("\t\tOption worked, grid is completed.");
            build_run.runner.success = Some(true);
            let start_time = Instant::now();
            build_run.grid = Some(self.complete_grid_post_build(grid));
            build_run.runner.return_object_time = Some(Instant::now() - start_time);
            return false;
        }

        true
    }

    #[inline]
    fn set_one_remaining_value(&self, build_run: &mut BuildRun, grid: &mut Grid, index: u16) -> bool {
        build_run.set_one_remaining_value_count += 1;
        if !build_run.runner.check_continue() {
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
        // Don't run the invariant because we might be calling this function so we can print out the
        // grid and find a problem.
        let mut complete_grid = grid.clone();
        complete_grid.symbols = self.symbols.clone();
        complete_grid.related_cell_indexes = self.related_cell_indexes.clone();
        complete_grid
    }

    fn complete_grid_post_build_with_values_u8(&self, grid: &Grid, values: &[u8]) -> Grid {
        // Don't run the invariant because we might be calling this function so we can print out the
        // grid and find a problem.
        let mut complete_grid = self.complete_grid_post_build(grid);
        complete_grid.replace_values(values);
        complete_grid
    }

    fn complete_grid_post_build_with_values_usize(&self, grid: &Grid, values: &[usize]) -> Grid {
        // Don't run the invariant because we might be calling this function so we can print out the
        // grid and find a problem.
        let values = values.iter().map(|x| *x as u8).collect::<Vec<_>>();
        self.complete_grid_post_build_with_values_u8(grid, &values)
    }

    fn build_flat(&self, build_run: &mut BuildRun, grid: &Grid) {

        let setup_start_time = Instant::now();

        let cell_count = grid.cell_count as usize;
        let value_count = grid.max_value as usize;
        let max_related_cell_count = grid.max_related_cell_count as usize;

        let mut values = Vec::with_capacity(cell_count);
        // Set the first cell to an arbitrary value and the rest to empty.
        values.push(1);
        for _ in 1..cell_count {
            values.push(NO_VALUE);
        }

        // Work out the related cell indexes in advance. Since we're not going to clone the grid or
        // the list of related cell indexes it's OK to put them in a more convenient form. Also we
        // only care about related cells with an index lower than the given cell.
        let mut related_cell_indexes = Vec::with_capacity(cell_count);
        for index in 0..cell_count {
            let first_lookup_index = index * max_related_cell_count;
            let mut related_indexes_one_cell = vec![];
            for lookup_index in first_lookup_index..(first_lookup_index + max_related_cell_count) {
                let related_cell_index = self.related_cell_indexes[lookup_index] as usize;
                if related_cell_index < index {
                    related_indexes_one_cell.push(related_cell_index);
                } else {
                    break;
                }
            }
            related_cell_indexes.push(related_indexes_one_cell);
        }
        //bg!(&related_cell_indexes);
        // Self::debug_print_related_cell_indexes_flat(&related_cell_indexes);

        /*
        // Work out the related cell indexes in advance. Since we're not going to clone the grid or
        // the list of related cell indexes it's OK to put them in a more convenient form. Also we
        // only care about related cells with an index lower than the given cell.
        let mut related_cell_indexes = Vec::with_capacity(cell_count);
        for index in 0..cell_count {
            let related_indexes = self.index_to_related_cell_indexes(grid, index as u16)
                .iter()
                .map(|related_index| *related_index as usize)
                .filter(|related_index| *related_index < index)
                .collect::<Vec<_>>();
            //bg!(index, &related_indexes);
            related_cell_indexes.push(related_indexes);
        }
        //bg!(&related_cell_indexes);
        */

        let mut remaining_values = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            remaining_values.push(vec![]);
        }

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\n{}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                dbg!(&remaining_values);
                let mut debug_values = values.clone();
                for i in current_cell_index..cell_count {
                    debug_values[i] = NO_VALUE;
                }
                let debug_grid = self.complete_grid_post_build_with_values(grid, debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let taken_values = related_cell_indexes[current_cell_index]
                    .iter()
                    .map(|related_index| values[*related_index])
                    .collect::<HashSet<_>>();
                //bg!(&taken_values);
                let remaining_value_count = value_count - taken_values.len();
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    remaining_values[current_cell_index] = (1..=value_count as u8)
                        .filter(|value| !taken_values.contains(value))
                        .collect::<Vec<_>>();
                    //bg!(&remaining_values[current_cell_index]);
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                while !remaining_values[current_cell_index].is_empty() {
                    let remaining_value_count = remaining_values[current_cell_index].len();
                    let remaining_value_index = if remaining_value_count == 1 {
                        0
                    } else {
                        thread_rng().gen_range(0, remaining_value_count)
                    };
                    let value = remaining_values[current_cell_index].remove(remaining_value_index);
                    //rintln!("build_flat(): Setting value {} at index {}.", value, current_cell_index);
                    values[current_cell_index] = value as u8;
                    if current_cell_index == cell_count - 1 {
                        // We've set the last value. The grid is complete. For now just show the
                        // values but really we need to create a real grid.

                        build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                        build_run.runner.success = Some(true);

                        let start_time = Instant::now();
                        build_run.grid = Some(self.complete_grid_post_build_with_values_u8(grid, &values));
                        build_run.runner.return_object_time = Some(Instant::now() - start_time);

                        return;
                    }
                    prev_cell_index = current_cell_index;
                    current_cell_index += 1;
                    continue 'main_loop;
                }
                // We've reached the end of the possible values and nothing has worked so go down
                // a level.
                //rintln!("build_flat(): We've reached the end of the possible values and nothing has worked so go down a level.");
                prev_cell_index = current_cell_index;
                current_cell_index -= 1;
            }
        }
    }

    /*
    fn set_up_fixed_9_related_cell_indexes(&mut self) {
        let mut related_cell_indexes = [std::usize::MAX; GRID_9_RELATED_CELL_LIST_SIZE];
        for i in 0..GRID_9_RELATED_CELL_LIST_SIZE {
            related_cell_indexes[i] = self.related_cell_indexes[i] as usize;
        }
        self.fixed_9_related_cell_indexes = Some(related_cell_indexes);
    }
    */

    fn set_up_fixed_related_cell_indexes(&mut self) {
        self.fixed_related_cell_indexes = self.related_cell_indexes.iter().map(|x| *x as usize).collect::<Vec<_>>();
    }

    fn build_flat_usize(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is similar to build_flat_9 in that it avoids allocating memory and converting
        // number types, but it allaws grids af any size.

        let value_count= grid.max_value as usize;
        let cell_count= grid.cell_count as usize;
        let max_related_cell_count = grid.max_related_cell_count as usize;

        // related_cell_indexes = self.fixed_9_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            values.push(NO_VALUE_USIZE);
        }
        values[0] = 1;

        let remaining_values_list_size = cell_count * value_count;
        let mut remaining_values = Vec::with_capacity(remaining_values_list_size);
        for _ in 0..remaining_values_list_size {
            remaining_values.push(false);
        }

        let mut remaining_value_counts = Vec::with_capacity(cell_count);
        for _ in 0..cell_count {
            remaining_value_counts.push(0)
        }

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_9() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..cell_count {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * value_count;
                for i in remaining_values_start_index..(remaining_values_start_index + value_count) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, value_count, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = value_count;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_9(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_9(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat_9(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_9(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * value_count;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + value_count) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= value_count);
                values[current_cell_index] = value;
                if current_cell_index == cell_count - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }

    fn build_flat_9(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is a specialized version of build_flat() that's only for 9x9 grids and that uses
        // fixed-size arrays as much as possible and avoids number conversions. It also takes in
        // some of the setup data rather than working it out each time, so it's better for repeated
        // calls that involve a 9x9 grid with the same rules.

        let max_related_cell_count = grid.max_related_cell_count as usize;
        // debug_assert!(max_related_cell_count <= GRID_9_MAX_MAX_RELATED_CELL_COUNT);

        // related_cell_indexes = self.fixed_9_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = [NO_VALUE_USIZE; GRID_9_CELL_COUNT];
        values[0] = 1;

        let mut remaining_values = [false; GRID_9_REMANING_VALUE_LIST_SIZE];
        let mut remaining_value_counts = [0; GRID_9_CELL_COUNT];

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_9() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..GRID_9_CELL_COUNT {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * GRID_9_VALUE_COUNT;
                for i in remaining_values_start_index..(remaining_values_start_index + GRID_9_VALUE_COUNT) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, GRID_9_VALUE_COUNT, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = GRID_9_VALUE_COUNT;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_9(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_9(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat_9(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_9(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * GRID_9_VALUE_COUNT;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + GRID_9_VALUE_COUNT) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= GRID_9_VALUE_COUNT);
                values[current_cell_index] = value;
                if current_cell_index == GRID_9_CELL_COUNT - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }

    fn build_flat_16(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is a specialized version of build_flat() that's only for 9x9 grids and that uses
        // fixed-size arrays as much as possible and avoids number conversions. It also takes in
        // some of the setup data rather than working it out each time, so it's better for repeated
        // calls that involve a 9x9 grid with the same rules.

        let max_related_cell_count = grid.max_related_cell_count as usize;
        // debug_assert!(max_related_cell_count <= GRID_16_MAX_MAX_RELATED_CELL_COUNT);

        // related_cell_indexes = self.fixed_16_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = [NO_VALUE_USIZE; GRID_16_CELL_COUNT];
        values[0] = 1;

        let mut remaining_values = [false; GRID_16_REMANING_VALUE_LIST_SIZE];
        let mut remaining_value_counts = [0; GRID_16_CELL_COUNT];

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_16() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..GRID_16_CELL_COUNT {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * GRID_16_VALUE_COUNT;
                for i in remaining_values_start_index..(remaining_values_start_index + GRID_16_VALUE_COUNT) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, GRID_16_VALUE_COUNT, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = GRID_16_VALUE_COUNT;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_16(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_16(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat_16(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_16(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * GRID_16_VALUE_COUNT;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + GRID_16_VALUE_COUNT) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= GRID_16_VALUE_COUNT);
                values[current_cell_index] = value;
                if current_cell_index == GRID_16_CELL_COUNT - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }

    fn build_flat_25(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is a specialized version of build_flat() that's only for 9x9 grids and that uses
        // fixed-size arrays as much as possible and avoids number conversions. It also takes in
        // some of the setup data rather than working it out each time, so it's better for repeated
        // calls that involve a 9x9 grid with the same rules.

        let max_related_cell_count = grid.max_related_cell_count as usize;
        // debug_assert!(max_related_cell_count <= GRID_25_MAX_MAX_RELATED_CELL_COUNT);

        // related_cell_indexes = self.fixed_25_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = [NO_VALUE_USIZE; GRID_25_CELL_COUNT];
        values[0] = 1;

        let mut remaining_values = [false; GRID_25_REMANING_VALUE_LIST_SIZE];
        let mut remaining_value_counts = [0; GRID_25_CELL_COUNT];

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_25() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..GRID_25_CELL_COUNT {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }

            if current_cell_index > prev_cell_index {
                println!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * GRID_25_VALUE_COUNT;
                for i in remaining_values_start_index..(remaining_values_start_index + GRID_25_VALUE_COUNT) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, GRID_25_VALUE_COUNT, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = GRID_25_VALUE_COUNT;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    println!("build_flat_25(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_25(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                println!("build_flat_25(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_25(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * GRID_25_VALUE_COUNT;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + GRID_25_VALUE_COUNT) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= GRID_25_VALUE_COUNT);
                values[current_cell_index] = value;
                if current_cell_index == GRID_25_CELL_COUNT - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }
    
    fn build_flat_36(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is a specialized version of build_flat() that's only for 9x9 grids and that uses
        // fixed-size arrays as much as possible and avoids number conversions. It also takes in
        // some of the setup data rather than working it out each time, so it's better for repeated
        // calls that involve a 9x9 grid with the same rules.

        let max_related_cell_count = grid.max_related_cell_count as usize;
        // debug_assert!(max_related_cell_count <= GRID_36_MAX_MAX_RELATED_CELL_COUNT);

        // related_cell_indexes = self.fixed_36_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = [NO_VALUE_USIZE; GRID_36_CELL_COUNT];
        values[0] = 1;

        let mut remaining_values = [false; GRID_36_REMANING_VALUE_LIST_SIZE];
        let mut remaining_value_counts = [0; GRID_36_CELL_COUNT];

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_36() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..GRID_36_CELL_COUNT {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * GRID_36_VALUE_COUNT;
                for i in remaining_values_start_index..(remaining_values_start_index + GRID_36_VALUE_COUNT) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, GRID_36_VALUE_COUNT, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = GRID_36_VALUE_COUNT;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_36(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_36(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat_36(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_36(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * GRID_36_VALUE_COUNT;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + GRID_36_VALUE_COUNT) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= GRID_36_VALUE_COUNT);
                values[current_cell_index] = value;
                if current_cell_index == GRID_36_CELL_COUNT - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }
    
    fn build_flat_49(&self, build_run: &mut BuildRun, grid: &Grid) {
        // This is a specialized version of build_flat() that's only for 9x9 grids and that uses
        // fixed-size arrays as much as possible and avoids number conversions. It also takes in
        // some of the setup data rather than working it out each time, so it's better for repeated
        // calls that involve a 9x9 grid with the same rules.

        let max_related_cell_count = grid.max_related_cell_count as usize;
        // debug_assert!(max_related_cell_count <= GRID_49_MAX_MAX_RELATED_CELL_COUNT);

        // related_cell_indexes = self.fixed_49_related_cell_indexes.unwrap();
        //bg!(&related_cell_indexes);

        let setup_start_time = Instant::now();

        let mut values = [NO_VALUE_USIZE; GRID_49_CELL_COUNT];
        values[0] = 1;

        let mut remaining_values = [false; GRID_49_REMANING_VALUE_LIST_SIZE];
        let mut remaining_value_counts = [0; GRID_49_CELL_COUNT];

        build_run.runner.setup_time = Some(Instant::now() - setup_start_time);

        // Hold a collection of remaining values for each cell throughout the loop. When going up to
        // some current_cell_index, recalculate the remaining values for that new cell. When going
        // down, don't recalculate since the previous cells haven't changed. Instead mark the path
        // we tried by removing one remaining value (unless we removed it while choosing it) and try
        // another one. If there are no choices remaining, decrement current_cell_index.

        let loop_start_time = Instant::now();

        let mut prev_cell_index = 0;
        let mut current_cell_index = 1;
        'main_loop: loop {

            /*
            {
                // Everything in this block is only for debugging.
                let direction = if current_cell_index > prev_cell_index { "UP" } else if current_cell_index == prev_cell_index { "SAME" } else { "DOWN" };
                println!("\nprint_flat_49() top of loop: {}: current = {}, prev = {}", direction, current_cell_index, prev_cell_index);
                let mut debug_values = values.clone();
                for i in current_cell_index..GRID_49_CELL_COUNT {
                    debug_values[i] = NO_VALUE_USIZE;
                }
                let debug_grid = self.complete_grid_post_build_with_values_usize(grid, &debug_values);
                debug_grid.print_simple_and_remaining_counts("");
            }
            */

            if current_cell_index > prev_cell_index {
                //rintln!("build_flat(): The current cell index is higher so we need to determine the available values.");
                // Find the available values for the current cell.
                let remaining_values_start_index = current_cell_index * GRID_49_VALUE_COUNT;
                for i in remaining_values_start_index..(remaining_values_start_index + GRID_49_VALUE_COUNT) {

                    if i >= remaining_values.len() {
                        //bg!(current_cell_index, GRID_49_VALUE_COUNT, remaining_values_start_index, i);
                    }

                    remaining_values[i] = true;
                }
                let mut remaining_value_count = GRID_49_VALUE_COUNT;
                let related_cell_lookup_start_index = current_cell_index * max_related_cell_count;
                'related_indexes: for i in related_cell_lookup_start_index..(related_cell_lookup_start_index + max_related_cell_count) {
                    let related_cell_index = self.fixed_related_cell_indexes[i];
                    if related_cell_index < current_cell_index {
                        let found_value = values[related_cell_index];
                        if found_value != NO_VALUE_USIZE {
                            let remaining_values_index = (remaining_values_start_index + found_value) - 1;
                            if remaining_values[remaining_values_index] {
                                remaining_values[remaining_values_index] = false;
                                remaining_value_count -= 1;
                            }
                        }
                    } else {
                        break 'related_indexes;
                    }
                }
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_49(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                } else {
                    //rintln!("build_flat_49(): There are remaining values so stay at this level.");
                    remaining_value_counts[current_cell_index] = remaining_value_count;
                    prev_cell_index = current_cell_index;
                }
                continue 'main_loop;
            } else {
                //rintln!("build_flat_49(): The current cell index is the same or lower so we need to keep going at the current level.");
                //bg!(&remaining_values[current_cell_index]);
                let remaining_value_count = remaining_value_counts[current_cell_index];
                if remaining_value_count == 0 {
                    // There are no remaining values for this cell, so this branch is not going to work.
                    //rintln!("build_flat_49(): There are no remaining values for this cell, so this branch is not going to work.");
                    prev_cell_index = current_cell_index;
                    current_cell_index -= 1;
                    continue 'main_loop;
                }
                let remaining_value_offset = if remaining_value_count == 1 {
                    0
                } else {
                    thread_rng().gen_range(0, remaining_value_count)
                };
                let mut value = NO_VALUE_USIZE;
                let remaining_values_start_index = current_cell_index * GRID_49_VALUE_COUNT;
                let mut remaining_values_used_index = std::usize::MAX;
                let mut found_count = 0;
                'find_value: for i in remaining_values_start_index..(remaining_values_start_index + GRID_49_VALUE_COUNT) {
                    if remaining_values[i] {
                        // This is a legal remaining value entry for the current cell.
                        if found_count == remaining_value_offset {
                            value = (i - remaining_values_start_index) + 1;
                            remaining_values_used_index = i;
                            break 'find_value;
                        } else {
                            found_count += 1;
                        }
                    }
                }
                debug_assert!(value != NO_VALUE_USIZE);
                debug_assert!(value <= GRID_49_VALUE_COUNT);
                values[current_cell_index] = value;
                if current_cell_index == GRID_49_CELL_COUNT - 1 {
                    // We've set the last value. The grid is complete. For now just show the
                    // values but really we need to create a real grid.

                    build_run.runner.loop_time = Some(Instant::now() - loop_start_time);

                    build_run.runner.success = Some(true);

                    let start_time = Instant::now();
                    build_run.grid = Some(self.complete_grid_post_build_with_values_usize(grid, &values));
                    build_run.runner.return_object_time = Some(Instant::now() - start_time);

                    return;
                }
                // Mark off the value we just set for the current cell so that we don't try it
                // again.
                remaining_values[remaining_values_used_index] = false;
                remaining_value_counts[current_cell_index] -= 1;
                prev_cell_index = current_cell_index;
                current_cell_index += 1;
                continue 'main_loop;
            }
        }
    }
    
    fn debug_print_related_cell_indexes_flat(related_cell_indexes: &Vec<Vec<usize>>) {
        println!("debug_print_related_cell_indexes_flat():");
        for (index, v) in related_cell_indexes.iter().enumerate() {
            let related_indexes = v.iter().map(|index| index.to_string()).join(", ");
            println!("\t{}: {}", index, related_indexes);
        }

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

    pub fn get_duration(&self) -> Duration {
        // This works only if there is a single build run and it has a duration.
        debug_assert!(self.build_runs.len() == 1);
        self.build_runs[0].runner.time.unwrap()
    }

}

impl BuildRun {
    pub fn new(time_limit: Option<Duration>, max_values: u8) -> Self {
        let mut branch_sizes = Vec::with_capacity(max_values as usize);
        for _ in 0..max_values {
            branch_sizes.push(0);
        }
        Self {
            runner: Runner::new(time_limit),
            fill_next_cell_count: 0,
            try_value_count: 0,
            set_cell_value_count: 0,
            set_one_remaining_value_count: 0,
            tried_grid_registered_count: 0,
            tried_grid_skipped_count: 0,
            tried_grid_found_count: 0,
            filled_cell_counts: vec![],
            tried_grids: Default::default(),
            branch_sizes,
            grid: None
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

