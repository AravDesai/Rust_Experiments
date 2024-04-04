use criterion::{ criterion_group, criterion_main, Criterion};

use broken_url_finder::parallelization::parallel;
use broken_url_finder::no_parallelization::no_parallel;
use broken_url_finder::channels::channel;
use broken_url_finder::async_await::async_await;

use std::path::PathBuf;

fn multi_threaded_files(c: &mut Criterion) {
    c.bench_function(
        "multi_threaded_files", 
        |b| b.iter(|| parallel::directory_reader(PathBuf::from("./inputs/files")))
    );
}

fn no_parallelization_files(c: &mut Criterion){
    c.bench_function(
        "no_parallelization_files", 
        |b| b.iter(|| no_parallel::directory_reader(PathBuf::from("./inputs/files")))
    );
}

fn channel_files(c: &mut Criterion){
    c.bench_function(
        "channel_files", 
        |b| b.iter(|| channel::channel_handler(PathBuf::from("./inputs/files")))
    );
}

#[tokio::main]
async fn async_files(c: &mut Criterion){
    c.bench_function(
        "async_files", 
        |b| b.iter(|| async_await::directory_reader(PathBuf::from("./inputs/files")))
    );
}

fn multi_threaded_folders(c: &mut Criterion) {
    c.bench_function(
        "multi_threaded_folders", 
        |b| b.iter(|| parallel::directory_reader(PathBuf::from("./inputs/folders")))
    );
}

fn no_parallelization_folders(c: &mut Criterion){
    c.bench_function(
        "no_parallelization_folders", 
        |b| b.iter(|| no_parallel::directory_reader(PathBuf::from("./inputs/folders")))
    );
}

fn channel_folders(c: &mut Criterion){
    c.bench_function(
        "channel_folders", 
        |b| b.iter(|| channel::channel_handler(PathBuf::from("./inputs/folders")))
    );
}

#[tokio::main]
async fn async_folders(c: &mut Criterion){
    c.bench_function(
        "async_folders", 
        |b| b.iter(|| async_await::directory_reader(PathBuf::from("./inputs/folders")))
    );
}

fn multi_threaded_sample(c: &mut Criterion) {
    c.bench_function(
        "multi_threaded_sample", 
        |b| b.iter(|| parallel::directory_reader(PathBuf::from("./inputs/sample")))
    );
}

fn no_parallelization_sample(c: &mut Criterion){
    c.bench_function(
        "no_parallelization_sample", 
        |b| b.iter(|| no_parallel::directory_reader(PathBuf::from("./inputs/sample")))
    );
}

fn channel_sample(c: &mut Criterion){
    c.bench_function(
        "channel_sample", 
        |b| b.iter(|| channel::channel_handler(PathBuf::from("./inputs/sample")))
    );
}

#[tokio::main]
async fn async_sample(c: &mut Criterion){
    c.bench_function(
        "async_sample", 
        |b| b.iter(|| async_await::directory_reader(PathBuf::from("./inputs/sample")))
    );
}

fn multi_threaded_empty(c: &mut Criterion) {
    c.bench_function(
        "multi_threaded_empty", 
        |b| b.iter(|| parallel::directory_reader(PathBuf::from("./inputs/empty")))
    );
}

fn no_parallelization_empty(c: &mut Criterion){
    c.bench_function(
        "no_parallelization_empty", 
        |b| b.iter(|| no_parallel::directory_reader(PathBuf::from("./inputs/empty")))
    );
}

fn channel_empty(c: &mut Criterion){
    c.bench_function(
        "channel_empty", 
        |b| b.iter(|| channel::channel_handler(PathBuf::from("./inputs/empty")))
    );
}

#[tokio::main]
async fn async_empty(c: &mut Criterion){
    c.bench_function(
        "async_empty", 
        |b| b.iter(|| async_await::directory_reader(PathBuf::from("./inputs/empty")))
    );
}


criterion_group!(benches, multi_threaded_files, no_parallelization_files, channel_files, async_files, multi_threaded_empty, no_parallelization_empty, channel_empty, async_empty, multi_threaded_folders, no_parallelization_folders, channel_folders, async_folders, multi_threaded_sample, no_parallelization_sample, channel_sample, async_sample);
criterion_main!(benches);