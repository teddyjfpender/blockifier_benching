//! Benchmark module for the blockifier crate. It provides functionalities to benchmark
//! various aspects related to transferring between accounts, including preparation
//! and execution of transfers.
//!
//! The main benchmark function is `transfers_benchmark`, which measures the performance
//! of transfers between randomly created accounts, which are iterated over round-robin.
//!
//! Run the benchmarks using `cargo bench --bench blockifier_bench`.

use std::time::Duration;

use blockifier::{blockifier::config::ConcurrencyConfig, test_utils::transfers_generator::{
    RecipientGeneratorType,
    TransfersGenerator,
    TransfersGeneratorConfig,
}};
use criterion::{criterion_group, criterion_main, Criterion};


pub fn transfers_benchmark(c: &mut Criterion) {
    let n_workers_vals = [1, 2, 4, 8, 16, 32];
    let chunk_sizes = [50, 100, 200, 400];

    let concurrency_configs: Vec<ConcurrencyConfig> = n_workers_vals
        .iter()
        .flat_map(|&n_workers| {
            chunk_sizes.iter().map(move |&chunk_size| ConcurrencyConfig {
                enabled: true,
                n_workers,
                chunk_size,
            })
        })
        .collect();

    // create a group which will contain all the benchmarks
    let mut group = c.benchmark_group("transfers");

    for config in concurrency_configs {
        let transfers_generator_config = TransfersGeneratorConfig {
            recipient_generator_type: RecipientGeneratorType::Random,
            n_accounts: 1000,
            n_txs: 1000,
            concurrency_config: config.clone(),
            ..Default::default()
        };

        // create copies of the config values for each benchmark
        let n_workers = config.n_workers;
        let chunk_size = config.chunk_size;

        let mut transfers_generator = TransfersGenerator::new(transfers_generator_config);
        // set a longer measurement time to get more accurate results
        group.measurement_time(std::time::Duration::from_secs(10));
        group.sample_size(100);
        group.warm_up_time(Duration::from_secs(10));
        // and performs transfers.
        group.bench_function(
            format!(
                "transfers_n_workers_{}_chunk_size_{}_concurrency",
                n_workers,
                chunk_size
            ),
            |b| {
                b.iter(|| {
                    transfers_generator.execute_transfers();
                });
            },
        );
    }
}

criterion_group!(benches, transfers_benchmark);
criterion_main!(benches);
