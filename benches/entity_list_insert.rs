use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inverted_index_util::entity_list::*;

fn insert_mut(n: usize) {
    let mut document_list: Vec<u8> = Vec::with_capacity(n);

    for _ in 0..n {
        let ulid_bytes: [u8; 16] = rusty_ulid::generate_ulid_bytes();
        insert_entity_mut(&mut document_list, &ulid_bytes);
    }
}

fn insert_immut(n: usize) {
    let mut document_list: Vec<u8> = Vec::with_capacity(n);
    for _ in 0..n {
        let ulid_bytes: [u8; 16] = rusty_ulid::generate_ulid_bytes();

        match insert_entity_immut(&document_list, &ulid_bytes) {
            ImmutResult::Changed(l) => document_list = l,
            ImmutResult::Unchanged => {}
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert_mut_2k", |b| b.iter(|| insert_mut(black_box(2000))));
    c.bench_function("insert_immut_2k", |b| b.iter(|| insert_immut(black_box(2000))));
    c.bench_function("insert_mut_200k", |b| b.iter(|| insert_mut(black_box(200_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
