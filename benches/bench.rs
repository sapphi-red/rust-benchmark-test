use criterion::{Criterion, criterion_group, criterion_main};
use phf::phf_set;

const DISALLOW_NEW_FOR_BUILTINS_1: [&str; 25] = [
    "BigInt",
    "Boolean",
    "Number",
    "adwad",
    "dawgfaw",
    " HHAWD",
    "awf",
    "5169",
    "2dwqqd",
    "dwqf",
    "BBCCDD",
    "AABBWW",
    "Q51515",
    "__+!@!",
    "ccaw252",
    "2112e",
    "QQDD",
    "QQ",
    "Tencent",
    "meituan",
    "alibaba",
    "bytedance",
    "mayi",
    "Symbol",
    "String",
];

const DISALLOW_NEW_FOR_BUILTINS_2: phf::Set<&'static str> = phf_set! {
    "BigInt",
    "Boolean",
    "Number",
    "adwad", "dawgfaw", " HHAWD", "awf", "5169", "2dwqqd", "dwqf",
    "BBCCDD", "AABBWW", "Q51515", "__+!@!", "ccaw252", "2112e",
    "QQDD","QQ","Tencent","meituan","alibaba","bytedance","mayi",
    "Symbol",
    "String",
};

fn phf_bad() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("Stringa");
}

fn phf_first() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("BigInt");
}

fn phf_last() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("String");
}

fn array_bad() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"Stringa");
}

fn array_first() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"BigInt");
}

fn array_last() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"String");
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("phf bad", |b| b.iter(|| phf_bad()));
    c.bench_function("array bad", |b| b.iter(|| array_bad()));

    c.bench_function("phf first", |b| b.iter(|| phf_first()));
    c.bench_function("array first", |b| b.iter(|| array_first()));

    c.bench_function("phf last", |b| b.iter(|| phf_last()));
    c.bench_function("array last", |b| b.iter(|| array_last()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
