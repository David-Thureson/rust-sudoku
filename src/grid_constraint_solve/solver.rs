#![allow(dead_code)]

// Differences from grid_constraint::builder::Builder:
// - Can solve from a partial grid.
// This goes with grid::Grid.

use crate::*;
use super::*;
use super::grid::Grid;
use super::builder::Builder;
use super::Runner;
// use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::time::{Instant, Duration};

const VERBOSE: u8 = 0;
const LOG_LEVEL: u8 = 2;
// https://emojipedia.org/

pub fn main() {
    // try_count_solutions();
    // try_unique_solution_type_2();
    // try_unique_solution_with_one_remaining_strategies();
    // try_reduce_exhaustive();
    // time_clones_in_unique_solution_type();
    profile_unique_solution_type();
    // try_has_solution_flat();
    // test_combination_count();
}

fn try_count_solutions() {
    let grid_size = 9;
    let remove_cell_count = 45;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 100_000;
    let repeat_count = 1;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec);
        let result = builder.build();
        match result {
            Ok(mut grid) => {
                grid.print_simple("");
                grid.remove_cells(remove_cell_count);
                grid.print_simple("");
                let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
                let result = solver.count_solutions();
                match result {
                    Ok(solution_count) => {
                        dbg!(&solver);
                        dbg!(solution_count);
                    },
                    Err(message) => {
                        println!("Solver error: {}", message);
                    },
                }
            },
            Err(message) => println!("Builder error: {}", message),
        }
    }
}

fn try_unique_solution_type() {
    let grid_size = 9;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 100_000;
    let repeat_count = 1;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec);
        let result = builder.build();
        match result {
            Ok(mut grid) => {
                grid.print_simple("");
                for remove_cell_count in 1..=grid.cell_count {
                    grid.remove_cells(1);
                    let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
                    let result = solver.unique_solution_type(&mut TriedGrids::new());
                    match result {
                        Ok(unique_solution_type) => {
                            println!("remove_cell_count = {}, time = {:?}, type = {:?}", remove_cell_count, solver.runner.time.unwrap(), unique_solution_type);
                            match unique_solution_type {
                                UniqueSolutionType::Many => {
                                    break;
                                },
                                _ => {}
                            }
                        },
                        Err(message) => {
                            println!("Solver error: {}", message);
                        },
                    }
                }
            },
            Err(message) => println!("Builder error: {}", message),
        }
    }
}

fn try_unique_solution_type_2() {
    let grid_size = 6;
    let remove_cell_count = 35;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 100_000;
    let repeat_count = 1;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec);
        let result = builder.build();
        match result {
            Ok(mut grid) => {
                grid.print_simple("");
                grid.remove_cells(remove_cell_count);
                let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
                let result = solver.unique_solution_type(&mut TriedGrids::new());
                match result {
                    Ok(unique_solution_type) => {
                        println!("remove_cell_count = {}, time = {:?}, type = {:?}", remove_cell_count, solver.runner.time.unwrap(), unique_solution_type);
                        match unique_solution_type {
                            UniqueSolutionType::Many => {
                                break;
                            },
                            _ => {}
                        }
                    },
                    Err(message) => {
                        println!("Solver error: {}", message);
                    },
                }
            },
            Err(message) => println!("Builder error: {}", message),
        }
    }
}

fn try_unique_solution_with_one_remaining_strategies() {
    let grid_size = 9;
    let remove_cell_count = 45;
    let build_limit_msec = 1_000;
    let solve_limit_msec = 100_000;
    let repeat_count = 5;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec);
        let result = builder.build();
        match result {
            Ok(mut grid) => {
                dbg!(builder.get_duration());
                grid.remove_cells(remove_cell_count);
                grid.print_simple("");
                for strategy in [SolverOneRemainingStrategy::Recursive, SolverOneRemainingStrategy::Straight].iter() {
                    let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
                    solver.one_remaining_strategy = (*strategy).clone();
                    let result = solver.unique_solution_type(&mut TriedGrids::new());
                    match result {
                        Ok(unique_solution_type) => {
                            println!("strategy = {:?}, time = {:?}, type = {:?}", strategy, solver.runner.time.unwrap(), unique_solution_type);
                        },
                        Err(message) => {
                            println!("Solver error: {}", message);
                        },
                    }
                }
            },
            Err(message) => println!("Builder error: {}", message),
        }
    }
}

fn try_reduce_exhaustive() {
    let grid_size = 9;
    let ascending = true;
    let solved_cells_min = 25;
    let solved_cells_max = 35;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 10_000_000_000;
    let inner_limit_msec = 10_000_000;
    let repeat_count = 1;

    for _ in 0..repeat_count {
        let mut builder = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec);
        let result = builder.build();
        match result {
            Ok(grid) => {
                grid.print_simple("");
                let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
                let result = solver.reduce_exhaustive(inner_limit_msec, ascending, solved_cells_min, solved_cells_max);
                match result {
                    Ok(grid) => {
                        dbg!(&grid);
                        dbg!(&solver);
                        grid.print_simple_and_remaining("");
                    },
                    Err(message) => {
                        println!("Solver error: {}", message);
                    },
                }
            },
            Err(message) => println!("Builder error: {}", message),
        }
    }
}

fn time_clones_in_unique_solution_type() {
    let grid_size = 9;
    let remove_cell_count = 55;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 10_000;
    let repeat_count = 10_000;

    let mut total_solve_time = Duration::from_secs(0);

    for i in 1..=repeat_count {
        let start_time = Instant::now();
        let mut grid = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec).build().unwrap();
        let build_time = Instant::now() - start_time;

        let start_time = Instant::now();
        grid.remove_cells(remove_cell_count);
        let remove_cells_time = Instant::now() - start_time;
        // grid.print_simple("");

        let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
        let result = solver.unique_solution_type(&mut TriedGrids::new());
        match result {
            Ok(unique_solution_type) => {
                let clone_count = solver.clone_grid_count + solver.clone_solution_grid_count + solver.clone_grid_ref_count;
                let clone_avg = solver.clone_time / clone_count as u32;
                total_solve_time += solver.runner.time.unwrap();
                let mean_solve_time = total_solve_time / i;
                println!("i = {}, type = {:?}, build_time = {:?}, remove_cells_time = {:?}, time = {:?}, clone_time = {:?}, clone_count = {:?}, clone_avg = {:?}, grid_count = {}, solution_grid_count = {}, grid_ref_count = {}, mean_solve_time = {:?}",
                         i, unique_solution_type, build_time, remove_cells_time, solver.runner.time.unwrap(), solver.clone_time, clone_count, clone_avg,
                         solver.clone_grid_count, solver.clone_solution_grid_count, solver.clone_grid_ref_count, mean_solve_time);
                match unique_solution_type {
                    UniqueSolutionType::One => {
                        grid.print_simple_and_remaining("");
                        return;
                    }
                    _ => {}
                }
            },
            Err(_message) => {
                //rintln!("Solver error: {}", message);
            },
        }
    }
}

fn profile_unique_solution_type() {
    let grid_size = 9;
    let remove_cell_count = 55;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 10_000;
    let repeat_count = 1;

    for _ in 1..=repeat_count {
        let mut grid = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec).build().unwrap();
        grid.remove_cells(remove_cell_count);
        let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
        let _result = solver.unique_solution_type(&mut TriedGrids::new());
   }
}

/*
fn try_has_solution_flat() {
    let grid_size = 9;
    let remove_cell_count = 55;
    let build_limit_msec = 10_000;
    let solve_limit_msec = 10_000;
    let repeat_count = 1;

    for _ in 1..=repeat_count {
        let mut grid = Builder::with_size(grid_size).limit_milliseconds(build_limit_msec).build().unwrap();
        grid.remove_cells(remove_cell_count);
        let mut solver = Solver::new(&grid).limit_milliseconds(solve_limit_msec);
        let _result = solver.unique_solution_type(&mut TriedGrids::new());
    }
}
*/

#[derive(Debug)]
pub enum SolverTask {
    CountSolutions,
    UniqueSolutionType,
    ReduceExhaustive,
    Unknown,
}

#[derive(Debug)]
pub enum UniqueSolutionType {
    Zero,
    One,
    Many,
}

#[derive(Clone, Debug)]
pub enum SolverOneRemainingStrategy {
    Recursive,
    Straight,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Solver {
    pub task: SolverTask,
    #[derivative(Debug="ignore")]
    pub grid: Grid,
    pub runner: Runner,
    pub unique_solution_type: Option<UniqueSolutionType>,
    pub one_remaining_strategy: SolverOneRemainingStrategy,
    pub min_cells_so_far: u16,
    pub solution_count: usize,
    #[derivative(Debug="ignore")]
    pub solution_grid: Option<Grid>,
    pub clone_grid_count: usize,
    pub clone_solution_grid_count: usize,
    pub clone_grid_ref_count: usize,
    pub clone_time: Duration,
}

impl Solver {
    fn new(grid: &Grid) -> Self {
        let solver = Self {
            task: SolverTask::Unknown,
            grid: grid.clone(),
            runner: Runner::new(None),
            unique_solution_type: None,
            one_remaining_strategy: SolverOneRemainingStrategy::Recursive,
            min_cells_so_far: std::u16::MAX,
            solution_count: 0,
            solution_grid: None,
            clone_grid_count: 0,
            clone_solution_grid_count: 0,
            clone_grid_ref_count: 0,
            clone_time: Duration::from_millis(0),
        };
        if VERBOSE >= 1 { dbg!(&solver); }
        solver
    }

    pub fn limit_seconds(mut self, seconds: u64) -> Self {
        self.runner.time_limit = Some(Duration::from_secs(seconds));
        self
    }

    pub fn limit_milliseconds(mut self, msec: u64) -> Self {
        self.runner.time_limit = Some(Duration::from_millis(msec));
        self
    }

    pub fn count_solutions(&mut self) -> Result<usize, String> {
        self.task = SolverTask::CountSolutions;
        self.runner = Runner::new(self.runner.time_limit);
        let clone_grid = self.clone_grid();
        let result = self.find_solutions(&clone_grid, &mut TriedGrids::new());
        match result {
            Ok(_0) => {
                self.runner.mark_end();
                Ok(self.solution_count)
            },
            Err(message) => Err(message),
        }
    }

    pub fn unique_solution_type(&mut self, tried_grids: &mut TriedGrids) -> Result<UniqueSolutionType, String> {
        self.task = SolverTask::UniqueSolutionType;
        self.runner = Runner::new(self.runner.time_limit);
        let clone_grid = self.clone_grid();
        let result = self.find_solutions(&clone_grid, tried_grids);
        match result {
            Ok(_0) => {
                self.runner.mark_end();
                Ok(match self.solution_count {
                    0 => UniqueSolutionType::Zero,
                    1 => UniqueSolutionType::One,
                    _ => UniqueSolutionType::Many,
                })
            }
            Err(message) => Err(message)
        }
    }

    pub fn reduce_exhaustive(&mut self, inner_time_limit_msec: u64, ascending: bool, solved_cells_min: u16, solved_cells_max: u16) -> Result<Grid, String> {
        self.task = SolverTask::CountSolutions;
        self.runner = Runner::new(self.runner.time_limit);
        let mut tried_grids = TriedGrids::new();
        tried_grids.enabled = true;
        for i in solved_cells_min..=solved_cells_max {
            let solved_cell_count = if ascending {
                i
            } else {
                self.grid.cell_count - i
            };
            let result = self.reduce_exhaustive_try_number_of_cells(solved_cell_count, inner_time_limit_msec, ascending, &mut tried_grids);
            match result {
                Ok(found_solution) => {
                    if found_solution {
                        self.runner.mark_end();
                        return Ok(self.clone_solution_grid())
                    }
                },
                Err(message) => {
                    return Err(message);
                },
            }
        }
        unreachable!()
    }

    /*
    pub fn reduce_exhaustive(&mut self, inner_time_limit_msec: u64) -> Result<Grid, String> {
        self.task = SolverTask::CountSolutions;
        self.runner = Runner::new(self.runner.time_limit);
        let result = self.reduce_exhaustive_next_cell(&self.grid.clone(), 0, inner_time_limit_msec);
        match result {
            Ok(_0) => {
                self.runner.mark_end();
                Ok(self.solution_grid.as_ref().unwrap().clone())
            }
            Err(message) => Err(message)
        }
    }
    */

    /*
    pub fn reduce_with_single_solution(&mut self) -> Result<usize, Grid> {
        self.task = SolverTask::ReduceWithSingleSolution;
        self.runner = Runner::new(self.runner.time_limit);
        let mut try_grid = self.grid.clone();
        while try_grid.solved_cell_count() > 0 {
            try_grid.remove_cells(1);
            let result = self.find_solutions(&self.grid.clone());
            match result {
                Ok(solution_count) => {
                    self.runner.mark_end();
                    Ok(solution_count)
                }
                _ => result
            }
    }
    */

    fn find_solutions(&mut self, grid_to_now: &Grid, tried_grids: &mut TriedGrids) -> Result<(), String> {
        if !self.runner.check_continue() {
            if self.runner.success.unwrap() {
                return Ok(());
            } else {
                return Err(self.runner.failure_message_clone());
            }
        }

        // Simply take the first empty cell.
        let try_cell_index = (0..grid_to_now.cell_count)
            .find(|index| grid_to_now.values[*index as usize] == NO_VALUE)
            .unwrap();

        for try_value in 1..=grid_to_now.max_value {
            let has_remaining_value = grid_to_now.has_remaining_value(try_cell_index, try_value);
            if has_remaining_value {

                //let try_this_value = tried_grids.register_attempt(grid_to_now, try_cell_index, try_value);
                let try_this_value = true;

                if try_this_value {
                    let mut try_grid = self.clone_grid_ref(grid_to_now);
                    // try_grid.print_simple_and_remaining(&format!("find_solutions(), before setting value {} at index {}", try_value, try_cell_index));
                    let result = self.set_value(&mut try_grid, try_cell_index, try_value);
                    // try_grid.print_simple_and_remaining(&format!("find_solutions(), after setting value {} at index {} for solved cell count = {}", try_value, try_cell_index, try_grid.solved_cell_count()));
                    match result {
                        Ok(grid_is_valid) => {
                            if grid_is_valid {
                                // We were able to set the cell's value and those of any related cells that ended up
                                // with only one remaining value (and so on recursively) without running into any
                                // cells that had zero remaining values. Thus the grid is still valid and we need to
                                // continue.
                                if try_grid.unsolved_cell_count == 0 {
                                    // The grid is complete. What we do with that fact depends on what solver task
                                    // we're trying to accomplish.
                                    self.solution_count += 1;
                                    match &self.task {
                                        SolverTask::CountSolutions => {},
                                        SolverTask::UniqueSolutionType => {
                                            // There's no point in counting solutions past the second one because
                                            // we're only trying to find out whether there are zero, one, or more
                                            // than one solutions.
                                            if self.solution_count > 1 {
                                                self.runner.success = Some(true);
                                                return Ok(())
                                            }
                                        },
                                        _ => panic!("Unexpected solver task = {:?}", &self.task)
                                    }
                                } else {
                                    // The grid is valid though still incomplete, so keep going by filling in
                                    // another cell.
                                    let result = self.find_solutions(&try_grid, tried_grids);
                                    match result {
                                        Ok(_0) => {}
                                        _ => return result
                                    }
                                }
                            }
                        },
                        Err(message) => {
                            return Err(message);
                        }
                    }
                    if !self.runner.check_continue() {
                        if self.runner.success.unwrap() {
                            return Ok(());
                        } else {
                            return Err(self.runner.failure_message_clone());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn set_value(&mut self, grid: &mut Grid, index: u16, value: u8) -> Result<bool, String> {
        // Return true if the grid was still valid after setting the value. That is, none of the
        // other cells ended up with zero possible values.

        if !self.runner.check_continue() {
            // We're out of time.
            return Err(self.runner.failure_message_clone());
        }

        debug_assert!(grid.values[index as usize] == NO_VALUE);
        debug_assert!(value > 0);
        debug_assert!(value <= grid.max_value);

        grid.set_value(index, value);
        // grid.values[index as usize] = value;
        // grid.clear_remaining_values(index);
        // grid.unsolved_cell_count -= 1;

        if grid.unsolved_cell_count > 0 {
            let related_cell_indexes = grid.index_to_related_cell_indexes(index);
            let mut one_value_indexes = vec![];
            for related_cell_index in related_cell_indexes.iter() {
                let related_cell_index = *related_cell_index as usize;
                if grid.values[related_cell_index] == NO_VALUE {
                    if grid.clear_remaining_value(related_cell_index as u16, value) {
                        // The remaining value was in the related cell so it's possible we're down
                        // to zero or one value.
                        match grid.remaining_value_counts[related_cell_index as usize] {
                            0 => {
                                // This empty cell has zero options left for its value so this attempt at the
                                // grid won't work.
                                return Ok(false);
                            },
                            1 => {
                                match self.one_remaining_strategy {
                                    SolverOneRemainingStrategy::Recursive => {
                                        one_value_indexes.push(related_cell_index);
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }

            if RUN_INVARIANT { grid.invariant(); }

            match self.one_remaining_strategy {
                SolverOneRemainingStrategy::Recursive => {
                    for related_cell_index in one_value_indexes {
                        // It's possible that between the code a few lines above and now this cell has been
                        // filled. This would happen when one call to set_one_remaining_value() calls
                        // set_value() which calls set_one_remaining_value() and so on recursively, and a given
                        // cell happens to be filled somewhere down in that tree of calls.
                        if grid.values[related_cell_index] == NO_VALUE {
                            let related_cell_index = related_cell_index as u16;
                            let value = grid.one_remaining_value(related_cell_index);
                            let result = self.set_value(grid, related_cell_index, value);
                            match result {
                                Ok(grid_is_valid) => {
                                    if !grid_is_valid {
                                        // This partial grid won't work.
                                        return Ok(false);
                                    }
                                },
                                Err(message) => {
                                    return Err(message);
                                }
                            }
                        }
                    }
                },
                SolverOneRemainingStrategy::Straight => {
                    if !self.resolve_cells_with_one_remaining(grid) {
                        // We found an unsolved cell with zero remaining values.
                    }
                },
            }
        }

        if RUN_INVARIANT { grid.invariant(); }

        Ok(true)
    }

    fn resolve_cells_with_one_remaining(&mut self, grid: &mut Grid) -> bool {
        // Returns true if it's OK to keep moving forward with this partial grid because there
        // are no unsolved cells with zero remaining value or false if there are such cells
        // meaning the partial grid must be abandoned.
        loop {
            let index = (0..grid.cell_count)
                .find(|index| grid.values[*index as usize] == NO_VALUE && grid.remaining_value_counts[*index as usize] <= 1);
            match index {
                Some(index) => {
                    // We found a cell with 0 or 1 remaining value.
                    match grid.remaining_value_counts[index as usize] {
                        0 => {
                            return false;
                        },
                        1 => {
                            let value = grid.one_remaining_value(index);
                            grid.set_value(index, value);
                        },
                        _ => panic!("Unexpected remaining counts at index {}.", index)
                    }
                },
                None => {
                    // There are no cells with either 0 or 1 remaining value.
                    return true;
                }
            }
        }
    }

    fn reduce_exhaustive_try_number_of_cells(&mut self, solved_cell_count: u16, inner_time_limit_msec: u64, ascending: bool, tried_grids: &mut TriedGrids) -> Result<bool, String> {

        let combination_limit = 10_000;
        let include = solved_cell_count < self.grid.cell_count / 2;

        let found_many = false;

        if !self.runner.check_continue() {
            return Err(self.runner.failure_message_clone());
        }

        let mut good_index_combinations = vec![];

        let combination_size = if include {
            solved_cell_count
        } else {
            self.grid.cell_count - solved_cell_count
        };
        let effective_combination_limit = self.effective_combination_limit(combination_limit, combination_size as usize);
        let mut index_combinations = self.index_combinations(combination_size as usize, effective_combination_limit);

        //let index_combinations = (0..self.grid.cell_count ).combinations(solved_cell_count as usize).collect::<Vec<_>>();
        //bg!(solved_cell_count, &index_combinations.len());
        //bg!(&tried_grids);
        // for index_combination in index_combinations {
        let mut combinations_tried = 0;
        while !index_combinations.is_empty() {
            let one_combination = index_combinations.remove(thread_rng().gen_range(0, index_combinations.len()));
            //bg!(&index_combination);
            let try_grid = self.grid.partial_grid_from_indexes(&one_combination, include);
            combinations_tried += 1;
            //dbg!(combinations_tried, &one_combination);
            println!("reduce_exhaustive_try_number_of_cells(): solved_cell_count = {}, combinations_tried = {}", solved_cell_count, combinations_tried);
            //try_grid.print_simple("");
            //bg!(&try_grid);
            // try_grid.print_simple_and_remaining("reduce_exhaustive_try_number_of_cells()");
            let mut solver = Solver::new(&try_grid).limit_milliseconds(inner_time_limit_msec);
            let result = solver.unique_solution_type(tried_grids);
            //bg!(&solver);
            match result {
                Ok(unique_solution_type) => {
                    match unique_solution_type {
                        UniqueSolutionType::Zero => {
                        }
                        UniqueSolutionType::One => {
                            // This is a legal grid with a unique solution.
                            if ascending {
                                self.solution_grid = Some(self.grid.partial_grid_from_indexes(&one_combination, include));
                                self.runner.success = Some(true);
                                return Ok(true);
                            }
                            good_index_combinations.push(one_combination.clone());
                        }
                        UniqueSolutionType::Many => {
                            if !ascending {
                                // This is the first grid found on the way down that has many
                                // solutions. For now stop the process at this level even though
                                // this isn't quite what we want.
                                //rintln!("found many");
                                //found_many = true;
                            }
                        },
                    }
                },
                Err(message) => {
                    return Err(message);
                }
            }
            if !self.runner.check_continue() {
                return Err(self.runner.failure_message_clone());
            }
        }

        println!("reduce_exhaustive_try_number_of_cells(): solved_cell_count = {}, single solution options = {}", solved_cell_count, good_index_combinations.len());

        if good_index_combinations.is_empty() {
            return Ok(false);
        }

        //bg!(&good_index_combinations);
        if ascending || found_many {
            let index_combination_index = if good_index_combinations.len() == 1 {
                // We found only one good index combination.
                0
            } else {
                // We found more than one good index combination so choose one at random.
                thread_rng().gen_range(0, good_index_combinations.len())
            };
            self.solution_grid = Some(self.grid.partial_grid_from_indexes(&good_index_combinations[index_combination_index], include));
            self.runner.success = Some(true);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn effective_combination_limit(&self, combination_limit: usize, solved_cell_count: usize) -> usize {
        //rintln!("effective_combination_limit(): combination_limit = {}, solved_cell_count = {}", combination_limit, solved_cell_count);
        let cell_count = self.grid.cell_count as usize;
        let mut possible_combinations = 1;
        for i in 0..solved_cell_count {
            possible_combinations = combination_count(cell_count, i);
            if combination_limit < possible_combinations {
                //rintln!("effective_combination_limit(): using combination_limit = {}", combination_limit);
                return combination_limit;
            }
        }
        //rintln!("effective_combination_limit(): using possible_combinations = {}", possible_combinations);
        possible_combinations
    }

    fn index_combinations(&self, solved_cell_count: usize, effective_combination_limit: usize) -> Vec<Vec<u16>> {
        let cell_count = self.grid.cell_count as usize;
        //rintln!("index_combinations(): cell_count = {}, solved_cell_count = {}, effective_combination_limit = {}", cell_count, solved_cell_count, effective_combination_limit);
        let mut v = Vec::with_capacity(effective_combination_limit);
        while v.len() < effective_combination_limit {
            let mut one_combination = Vec::with_capacity(solved_cell_count);
            while one_combination.len() < solved_cell_count {
                let index = thread_rng().gen_range(0, cell_count) as u16;
                if !one_combination.contains(&index) {
                    one_combination.push(index as u16);
                }
            }
            one_combination.sort();
            debug_assert_eq!(solved_cell_count, one_combination.len());
            if !v.contains(&one_combination) {
                //rintln!("index_combinations(): adding {:?}", &one_combination);
                v.push(one_combination);
            } else {
                //rintln!("index_combinations(): accepted_count = {}, rejecting {:?}", v.len(), &one_combination);
            }
        }
        debug_assert_eq!(effective_combination_limit, v.len());
        v
    }

/*
    pub clone_grid_count: u16,
    pub clone_solution_grid_count: u16,
    pub clone_grid_ref_count: u16,
    pub clone_time: Duration,

*/

    fn clone_grid(&mut self) -> Grid {
        self.clone_grid_count += 1;
        let start_time = Instant::now();
        let grid = self.grid.clone();
        self.clone_time += Instant::now() - start_time;
        grid
    }

    fn clone_solution_grid(&mut self) -> Grid {
        self.clone_solution_grid_count += 1;
        let start_time = Instant::now();
        let grid = self.solution_grid.as_ref().unwrap().clone();
        self.clone_time += Instant::now() - start_time;
        grid
    }

    fn clone_grid_ref(&mut self, grid: &Grid) -> Grid {
        self.clone_grid_ref_count += 1;
        let start_time = Instant::now();
        let grid = grid.clone();
        self.clone_time += Instant::now() - start_time;
        grid
    }

    /*
    fn reduce_exhaustive_next_cell(&mut self, grid_to_now: &Grid, min_index: u16, inner_time_limit_msec: u64) -> Result<(), String> {
        if min_index >= grid_to_now.cell_count {
            return Ok(())
        }

        if !self.runner.check_continue() {
            return Err(self.runner.failure_message_clone());
        }

        //bg!(min_index);
        //grid_to_now.print_simple("reduce_exhaustive_next_cell()");

        for try_index in min_index..grid_to_now.cell_count {
            //bg!(try_index);
            let mut try_grid = grid_to_now.clone();
            try_grid.set_value(try_index, NO_VALUE);
            let mut solver = Solver::new(&try_grid).limit_milliseconds(inner_time_limit_msec);
            let result = solver.unique_solution_type();
            match result {
                Ok(unique_solution_type) => {
                    match unique_solution_type {
                        UniqueSolutionType::Zero => {
                            panic!("Did not expect to find a partial grid with no solutions.");
                        }
                        UniqueSolutionType::One => {
                            // This is a legal grid with a unique solution.
                            let solved_cell_count = try_grid.solved_cell_count();
                            if solved_cell_count < self.min_cells_so_far {
                                // This is the smallest grid we've seen so far with a unique solution.
                                self.min_cells_so_far = solved_cell_count;
                                self.solution_grid = Some(try_grid.clone());
                            }
                            // Continue down this branch looking for a better solution.
                            let result = self.reduce_exhaustive_next_cell(&try_grid, try_index + 1, inner_time_limit_msec);
                            match result {
                                Ok(_0) => {
                                    // We've fully explored this branch.
                                },
                                Err(message) => {
                                    return Err(message)
                                }
                            }
                        }
                        UniqueSolutionType::Many => {
                            // This is a legal partial grid with multiple solutions so this branch
                            // will not get us anywhere.
                            return Ok(());
                        },
                    }
                },
                Err(message) => {
                    return Err(message);
                }
            }
        }
        Ok(())
    }
    */

}

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct TriedGrids {
    pub enabled: bool,
    pub registered_count: usize,
    pub found_count: usize,
    #[derivative(Debug="ignore")]
    pub hashes: HashSet<u64>,
}

impl TriedGrids {
    fn new() -> Self {
        Self {
            enabled: false,
            registered_count: 0,
            found_count: 0,
            hashes: Default::default()
        }
    }

    fn register_attempt(&mut self, grid: &Grid, try_cell_index: u16, try_value: u8) -> bool {
        // Return true if this is a grid we haven't tried yet.

        if !self.enabled {
            return true
        }

        self.registered_count += 1;
        let try_cell_index = try_cell_index as usize;
        debug_assert!(grid.values[try_cell_index] == NO_VALUE);
        debug_assert!(try_value >= 1);
        debug_assert!(try_value <= grid.max_value);

        // Start with a copy of the grid's values just before trying this new value.
        let mut try_values = grid.values.clone();
        // Set the new value we're about to try. The list of values including this new one forms a
        // signature for grids we don't want to attempt more than once.
        try_values[try_cell_index] = try_value;

        let mut hasher = DefaultHasher::new();
        try_values.hash(&mut hasher);
        let hash = hasher.finish();

        let is_new = {
            if self.hashes.contains(&hash) {
                // We've already tried this partial grid.
                self.found_count += 1;
                false
            } else {
                // We haven't tried this partial grid so add it to the list and give it a shot.
                self.hashes.insert(hash);
                true
            }
        };
        is_new
    }

}

fn combination_count(set_size: usize, chosen_count: usize) -> usize {
    assert!(set_size >= chosen_count);
    if chosen_count == 0 {
        return 1;
    }
    if set_size == chosen_count {
        return 1;
    }
    let mut f = 1;
    for i in (set_size - chosen_count) + 1..=set_size {
        f *= i;
    }
    f /= factorial(chosen_count);
    f
}

fn factorial(n: usize) -> usize {
    let mut f = 1;
    for i in 2..=n {
        f *= i;
    }
    f
}

fn test_combination_count() {
    assert_eq!(1, combination_count(4, 0));
    assert_eq!(4, combination_count(4, 1));
    assert_eq!(6, combination_count(4, 2));
    assert_eq!(4, combination_count(4, 3));
    assert_eq!(1, combination_count(4, 4));

    assert_eq!(1, combination_count(5, 0));
    assert_eq!(5, combination_count(5, 1));
    assert_eq!(10, combination_count(5, 2));
    assert_eq!(10, combination_count(5, 3));
    assert_eq!(5, combination_count(5, 4));
    assert_eq!(1, combination_count(5, 5));
}