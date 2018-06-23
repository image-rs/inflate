#[macro_use]
extern crate criterion;
extern crate inflate;

use criterion::Criterion;
use std::path::Path;
use std::io::Read;
use std::fs::File;

fn load_file<P: AsRef<Path>>(path: P) -> std::vec::Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

fn abc_txt(c: &mut Criterion) {
    let data = load_file("bench_samples/abc.txt.zz");
    c.bench_function("abc_txt", move |b| b.iter(|| inflate::inflate_bytes_zlib(&data) ));
}

fn heavily_compressed_text(c: &mut Criterion) {
    let data = load_file("bench_samples/heavily_compressed_text.zz");
    c.bench_function("heavily_compressed_text", move |b| b.iter(|| inflate::inflate_bytes_zlib(&data) ));
}

fn lightly_compressed_text(c: &mut Criterion) {
    let data = load_file("bench_samples/lightly_compressed_text.zz");
    c.bench_function("lightly_compressed_text", move |b| b.iter(|| inflate::inflate_bytes_zlib(&data) ));
}

fn knock_ogg(c: &mut Criterion) {
    let data = load_file("bench_samples/knock.ogg.zz");
    c.bench_function("knock_ogg", move |b| b.iter(|| inflate::inflate_bytes_zlib(&data) ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = abc_txt, heavily_compressed_text, lightly_compressed_text, knock_ogg
}

criterion_main!(benches);