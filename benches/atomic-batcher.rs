use atomic_batcher::Batcher;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("append", |b| {
    b.iter(|| {
      let tell_length = |s: Vec<String>| async move { s.len() };

      let batcher = Batcher::new(tell_length);
      batcher.append("world".to_string());
    });
  });

  c.bench_function("appendcb", |b| {
    b.iter(|| {
      let tell_length = |s: Vec<String>| async move { s.len() };
      let callback = |n| async move { n == 5 };

      let batcher = Batcher::new(tell_length);
      batcher.appendcb("world".to_string(), callback);
    });
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
