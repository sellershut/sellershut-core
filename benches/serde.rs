use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prost::Message;
use sellershut_core::category::Category;

fn bench(c: &mut Criterion) {
    let category = Category::default();

    c.bench_function("serialise category [json]", |b| {
        b.iter(|| black_box(serde_json::to_string(&category)))
    });

    c.bench_function("serialise category [prost]", |b| {
        b.iter(|| {
            black_box(|| {
                let mut bytes = Vec::with_capacity(category.encoded_len());
                category.encode(&mut bytes).unwrap();
            })
        })
    });

    let cat_str = serde_json::to_string(&category).unwrap();

    c.bench_function("deserialise category [json]", |b| {
        b.iter(|| black_box(serde_json::from_str::<Category>(&cat_str)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
