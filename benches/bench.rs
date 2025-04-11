use criterion::{Criterion, criterion_group, criterion_main};
use phf::phf_set;

const DISALLOW_NEW_FOR_BUILTINS_1: [&str; 25] = [
    "2112e",
    "2dwqqd",
    "5169",
    "AABBWW",
    "BBCCDD",
    "BigInt",
    "Boolean",
    "HHAWD",
    "Number",
    "Q51515",
    "QQ",
    "QQDD",
    "String",
    "Symbol",
    "__+!@!",
    "Tencent",
    "adwad",
    "alibaba",
    "awf",
    "bytedance",
    "ccaw252",
    "dawgfaw",
    "dwqf",
    "mayi",
    "meituan",
];

const DISALLOW_NEW_FOR_BUILTINS_2: phf::Set<&'static str> = phf_set! {
    "BigInt",
    "Boolean",
    "Number",
    "adwad", "dawgfaw", "HHAWD", "awf", "5169", "2dwqqd", "dwqf",
    "BBCCDD", "AABBWW", "Q51515", "__+!@!", "ccaw252", "2112e",
    "QQDD","QQ","Tencent","meituan","alibaba","bytedance","mayi",
    "Symbol",
    "String",
};

fn phf_bad() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("-Stringa");
}

fn phf_first() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("BigInt");
}

fn phf_middle() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("Q51515");
}

fn phf_last() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_2.contains("String");
}

fn array_bad() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"-Stringa");
}

fn array_first() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"2112e");
}

fn array_middle() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"String");
}

fn array_last() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.contains(&"meituan");
}

fn array_binary_bad() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1
        .binary_search(&"-Stringa")
        .is_ok();
}

fn array_binary_first() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.binary_search(&"2112e").is_ok();
}

fn array_binary_middle() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1.binary_search(&"String").is_ok();
}

fn array_binary_last() {
    let _ = DISALLOW_NEW_FOR_BUILTINS_1
        .binary_search(&"meituan")
        .is_ok();
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("phf bad", |b| b.iter(|| phf_bad()));
    c.bench_function("array bad", |b| b.iter(|| array_bad()));
    c.bench_function("array binary bad", |b| b.iter(|| array_binary_bad()));

    c.bench_function("phf first", |b| b.iter(|| phf_first()));
    c.bench_function("array first", |b| b.iter(|| array_first()));
    c.bench_function("array binary first", |b| b.iter(|| array_binary_first()));

    c.bench_function("phf middle", |b| b.iter(|| phf_middle()));
    c.bench_function("array middle", |b| b.iter(|| array_middle()));
    c.bench_function("array binary middle", |b| b.iter(|| array_binary_middle()));

    c.bench_function("phf last", |b| b.iter(|| phf_last()));
    c.bench_function("array last", |b| b.iter(|| array_last()));
    c.bench_function("array binary last", |b| b.iter(|| array_binary_last()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
