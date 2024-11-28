#![feature(test)]

extern crate membarrier;
extern crate test;

use std::sync::atomic::{fence, Ordering};
use test::Bencher;

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
