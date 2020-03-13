use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inverted_index_util::document_list::insert_document_mut;

fn insert_mut(n: u64) {
    let mut document_list: Vec<u8> = Vec::new();

    for _ in 0..n {
        let ulid_bytes: [u8; 16] = rusty_ulid::generate_ulid_bytes();
        insert_document_mut(&mut document_list, &ulid_bytes);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert_mut_2k", |b| b.iter(|| insert_mut(black_box(2000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
