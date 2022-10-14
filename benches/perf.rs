use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enigma::{
    enigma::Enigma,
    plugboard::{Plugboard, Plugs},
    reflector::Reflectors,
    rotor::{RotorConfig, Rotors},
};
use rand::Rng; // failed to resolve: use of undeclared crate or module `enigma` use of undeclared crate or module `enigma`rustcE0433

fn gen_rand_string(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// Lots of red squigglies because of imports
fn construct_enigma() -> Enigma {
    let rotors: RotorConfig =
        RotorConfig::try_from([(Rotors::I, 'A'), (Rotors::II, 'X'), (Rotors::IV, 'N')]).unwrap();
    let plugs = Plugs::try_from(vec![]).unwrap();
    let plugboard: Plugboard = Plugboard::try_from(plugs).unwrap();
    let reflector: Reflectors = Reflectors::B;

    Enigma::new(rotors, plugboard, reflector)
}

fn criterion_benchmark(c: &mut Criterion) {
    let e = construct_enigma();
    let k1 = gen_rand_string(1000);
    let k10 = gen_rand_string(10_000);
    let k100 = gen_rand_string(100_000);
    let m1 = gen_rand_string(1_000_000);
    let m10 = gen_rand_string(10_000_000);
    let m100 = gen_rand_string(100_000_000);

    c.bench_function("1k", |b| b.iter(|| e.encode(black_box(&k1))));
    c.bench_function("10k", |b| b.iter(|| e.encode(black_box(&k10))));
    c.bench_function("100k", |b| b.iter(|| e.encode(black_box(&k100))));
    c.bench_function("1m", |b| b.iter(|| e.encode(black_box(&m1))));
    c.bench_function("10m", |b| b.iter(|| e.encode(black_box(&m10))));
    c.bench_function("100m", |b| b.iter(|| e.encode(black_box(&m100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
