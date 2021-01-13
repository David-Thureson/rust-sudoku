#![allow(unused_imports)]

// use std::iter;
// use test::black_box;

use criterion::{criterion_group, criterion_main, black_box, Criterion, BenchmarkId, BatchSize, Throughput, PlotConfiguration, AxisScale};

use sudoku::*;

pub fn build_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_compare");
    group.sample_size(10);

    let grid_height = 3;
    let block_width = 2;
    let block_height = 2;
    for grid_width in 3..=6 {
        group.bench_with_input(BenchmarkId::new("builder_1", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_1::Grid::build(grid_width, grid_height, block_width, block_height))
        });
        group.bench_with_input(BenchmarkId::new("builder_2", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_obj::Grid::build(grid_width, grid_height, block_width, block_height))
        });
    }
    group.finish();
}

pub fn build_compare_with_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_compare_with_vec");
    group.sample_size(10);

    let grid_height = 3;
    let block_width = 2;
    let block_height = 2;
    for grid_width in 3..=6 {
        group.bench_with_input(BenchmarkId::new("builder_obj", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_obj::Grid::build(grid_width, grid_height, block_width, block_height))
        });
        group.bench_with_input(BenchmarkId::new("builder_obj_log", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_obj_log::Grid::build(grid_width, grid_height, block_width, block_height, None, None))
        });
        group.bench_with_input(BenchmarkId::new("builder_vec_log", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_vec_log::Grid::build(grid_width, grid_height, block_width, block_height, None, None))
        });
    }
    group.finish();
}

pub fn build_vec_vary_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_vec_vary_size");
    group.sample_size(10);

    let grid_height = 4;
    let block_width = 2;
    let block_height = 2;
    for grid_width in 4..=6 {
        group.bench_with_input(BenchmarkId::new("builder_vec", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_vec::Grid::build(grid_width, grid_height, block_width, block_height, None))
        });
        group.bench_with_input(BenchmarkId::new("builder_vec_log", grid_width), &grid_width, |b, _| {
            b.iter(|| builder_vec_log::Grid::build(grid_width, grid_height, block_width, block_height, None, None))
        });
    }
    group.finish();
}

pub fn build_vary_tried_grid_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_vary_tried_grid_count");
    // group.sample_size(10);

    // Best is about 975.
    // let grid_width = 4;
    // let grid_height = 4;
    // let block_width = 2;
    // let block_height = 2;

    let grid_width = 9;
    let grid_height = 9;
    let block_width = 3;
    let block_height = 3;
    // for max_tried_grid_count in (0..=800).step_by(20) {
    for max_tried_grid_count in (0..=4000).step_by(1000) {
        group.bench_with_input(BenchmarkId::new("builder_vec", max_tried_grid_count), &max_tried_grid_count, |b, _| {
            b.iter(|| builder_vec::Grid::build(grid_width, grid_height, block_width, block_height, Some(max_tried_grid_count as usize)))
        });
    }
    group.finish();
}

pub fn build_vary_tried_grid_count_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_vary_tried_grid_count_2");
    // group.sample_size(10);
    let grid_size = 9;
    for max_tried_grid_count in (0..=10_000).step_by(1_000) {
        group.bench_with_input(BenchmarkId::new("grid_constraint::Builder", max_tried_grid_count), &max_tried_grid_count, |b, _| {
            b.iter(|| grid_constraint::builder::Builder::with_size(grid_size).build())
        });
    }
    group.finish();
}

pub fn build_regular_vs_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_regular_vs_large");
    group.sample_size(20);

    for block_size in 2..=3 {
        let grid_width = block_size * block_size;
        let grid_height = grid_width;
        let block_width = block_size;
        let block_height = block_width;
        group.bench_with_input(BenchmarkId::new("builder_vec", block_size), &block_size, |b, _| {
            b.iter(|| builder_vec::Grid::build(grid_width, grid_height, block_width, block_height, None))
        });
        group.bench_with_input(BenchmarkId::new("builder_vec_large", block_size), &block_size, |b, _| {
            b.iter(|| builder_vec_large::Grid::build(grid_width, grid_height, block_width, block_height, None))
        });
    }
    group.finish();
}

pub fn build_large_vs_constraint(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_large_vs_constraint");
    group.sample_size(20);

    for block_size in 2..=3 {
        let grid_width = block_size * block_size;
        let grid_height = grid_width;
        let block_width = block_size;
        let block_height = block_width;
        group.bench_with_input(BenchmarkId::new("builder_vec_large", block_size), &block_size, |b, _| {
            b.iter(|| builder_vec_large::Grid::build(grid_width, grid_height, block_width, block_height, None))
        });
        group.bench_with_input(BenchmarkId::new("grid_constraint::Builder", block_size), &block_size, |b, _| {
            b.iter(|| grid_constraint::builder::Builder::with_block_size(block_size).build())
        });
    }
    group.finish();
}

/*
pub fn build_constraint_with_or_without_calls_to_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_constraint_with_or_without_calls_to_grid");
    // group.sample_size(10);
    let grid_size = 9;
    for call_grid_for_set_remaining in [false, true].iter() {
        group.bench_with_input(BenchmarkId::new("grid_constraint::Builder", call_grid_for_set_remaining), &call_grid_for_set_remaining, |b, _| {
            b.iter(|| {
                let mut builder = grid_constraint::builder::Builder::with_size(grid_size);
                builder.call_grid_for_set_remaining = *call_grid_for_set_remaining;
                builder.build()
            })
        });
    }
    group.finish();
}
*/

criterion_group!(benches,
    // build_compare,
    // build_compare_with_vec,
    // build_vec_vary_size,
    // build_vary_tried_grid_count,
    build_vary_tried_grid_count_2,
    // build_regular_vs_large,
    // build_large_vs_constraint,
    // build_constraint_with_or_without_calls_to_grid,
    );
criterion_main!(benches);

// From the main project folder run:
//   cargo +nighly bench
// for all benchmarks or:
//   cargo +nightly bench --bench sort_benchmark
// for just the above group.
