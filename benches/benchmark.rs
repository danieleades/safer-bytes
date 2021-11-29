use criterion::{black_box, criterion_group, criterion_main, Criterion};
use safer_bytes::{BytesMut, SafeBuf};

fn copy_to_slice(_dummy: usize) {
    let mut buffer = BytesMut::new();
    buffer.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let output = &mut [0_u8; 6];
    let _output = buffer.try_copy_to_slice(output).unwrap();
}

fn copy_to_bytes(_dummy: usize) {
    let mut buffer = BytesMut::new();
    buffer.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let _output = buffer.try_copy_to_bytes(6).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("copy_to_slice", |b| b.iter(|| copy_to_slice(black_box(6))));
    c.bench_function("copy_to_bytes", |b| b.iter(|| copy_to_bytes(black_box(6))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
