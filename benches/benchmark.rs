use rand::prelude::*;
use weighted_rand::builder::*;

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

const WEIGHTS: [u32; 1000] = [1; 1000];

fn bench_constructor(c: &mut Criterion) {
    let vector = WEIGHTS.to_vec();
    c.bench_function("constructor", |b| {
        b.iter(|| WalkerTableBuilder::new(&vector))
    });
}

fn bench_generate_by_wam_next(c: &mut Criterion) {
    let builder = WalkerTableBuilder::new(&WEIGHTS.to_vec());
    let table = builder.build();

    let mut result = [0; 100_000];

    c.bench_function("generate_by_wam_next", |b| {
        b.iter(|| {
            for r in &mut result {
                *r = table.next();
            }
        })
    });
}

fn bench_generate_by_wam_next_rng(c: &mut Criterion) {
    let builder = WalkerTableBuilder::new(&WEIGHTS.to_vec());
    let table = builder.build();

    let mut rng = rand::thread_rng();

    let mut result = [0; 100_000];

    c.bench_function("generate_by_wam_next_rng", |b| {
        b.iter(|| {
            for r in &mut result {
                *r = table.next_rng(&mut rng);
            }
        })
    });
}

fn bench_generate_by_csm(c: &mut Criterion) {
    let mut probs = Vec::new();
    for i in 0..1000 {
        probs.push(i + 1);
    }
    let probs = probs
        .iter()
        .map(|v| *v as f32 / 1000.0)
        .collect::<Vec<f32>>()
        .to_vec();

    let csm = CSM { probs: probs };
    let mut rng = rand::thread_rng();
    let mut result = [0; 100_000];

    c.bench_function("generate_by_csm", |b| {
        b.iter(|| {
            for r in &mut result {
                *r = csm.next(&mut rng);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_constructor,
    bench_generate_by_wam_next,
    bench_generate_by_wam_next_rng,
    bench_generate_by_csm
);
criterion_main!(benches);

// ========================================================= //

// Weighted random sampling using Cumulative Sum Method

struct CSM {
    probs: Vec<f32>,
}

impl CSM {
    fn next(&self, rng: &mut ThreadRng) -> usize {
        let r = rng.gen::<f32>();
        let mut result = 0;

        for (i, p) in self.probs.iter().enumerate() {
            if r <= *p {
                result = i;
                break;
            }
        }

        result
    }
}
