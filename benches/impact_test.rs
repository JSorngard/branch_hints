use branch_hints::likely;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::hint::black_box;

const N: usize = 1_000_000;

fn random_numbers(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([0b10101010; 32]);
    let mut group = c.benchmark_group("random numbers");

    group.bench_function("no hints", |b| {
        b.iter_batched(
            || (0..N).map(|_| rng.gen()).collect::<Vec<u32>>(),
            |data| {
                black_box({
                    data.iter()
                        .map(|&val| {
                            if val > 1 {
                                1
                            } else {
                                val.pow(u32::from(val.next_multiple_of(13)))
                                    .checked_div(val + 1)
                                    .unwrap_or(0)
                            }
                        })
                        .sum::<u32>()
                })
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("with hints", |b| {
        b.iter_batched(
            || (0..N).map(|_| rng.gen()).collect::<Vec<u32>>(),
            |data| {
                black_box({
                    data.iter()
                        .map(|&val| {
                            if likely(val > 1) {
                                1
                            } else {
                                val.pow(u32::from(val.next_multiple_of(13)))
                                    .checked_div(val + 1)
                                    .unwrap_or(0)
                            }
                        })
                        .sum::<u32>()
                })
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {impact_test, random_numbers}
criterion_main! {impact_test}
