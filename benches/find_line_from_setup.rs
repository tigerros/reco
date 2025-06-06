#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn bench_find_line_from_setup(b: &mut Bencher) {
    // Worst case scenario
    let setup = reco::book::zukertort_opening::WARE_DEFENSE.lines()[0].setup();

    b.iter(|| {
        reco::book::find_line_from_setup(&setup);
    });
}
