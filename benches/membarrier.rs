#![feature(test)]

extern crate test;
extern crate membarrier;

use test::Bencher;
use std::{sync::atomic::{fence, Ordering}, hint::black_box};

#[bench]
fn light(b: &mut Bencher) {
    b.iter(|| {
        membarrier::light();
    });
}

#[bench]
fn normal(b: &mut Bencher) {
    b.iter(|| {
        fence(Ordering::SeqCst);
    });
}

#[bench]
fn heavy(b: &mut Bencher) {
    b.iter(|| {
        membarrier::heavy();
    });
}

#[bench]
fn bulk_light(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1000 {
            black_box(membarrier::light());
        }
    });
}
