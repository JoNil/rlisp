#![allow(unstable)]

extern crate rlisp;
extern crate test;

use rlisp::Rlisp;

use test::Bencher;

#[bench]
fn bench_rlisp_add(b: &mut Bencher) {
    let mut rlisp = Rlisp::new();
   
    b.iter(|| {
        test::black_box(rlisp.execute("(+ 1 1)"));
    });
}

#[bench]
fn bench_rlisp_lookup(b: &mut Bencher) {
    let mut rlisp = Rlisp::new();

    rlisp.execute("(def {a} 12)");
   
    b.iter(|| {
        test::black_box(rlisp.execute("a"));
    });
}

#[bench]
fn bench_rlisp_add_lookup(b: &mut Bencher) {
    let mut rlisp = Rlisp::new();

    rlisp.execute("(def {a} 12)");
   
    b.iter(|| {
        test::black_box(rlisp.execute("(+ a a)"));
    });
}

#[bench]
fn bench_rlisp_recursion_test_parse(b: &mut Bencher) {
    let mut rlisp = Rlisp::new();
   
    b.iter(|| {
        test::black_box(rlisp.execute(
            "(def {rev l} {if (== l {}) {list} {join (rev (tail l)) (list (head l))}})"));
    });
}

#[bench]
fn bench_rlisp_recursion_test_reverse(b: &mut Bencher) {
    let mut rlisp = Rlisp::new();
   
    rlisp.execute("(def {rev l} {if (== l {}) {list} {join (rev (tail l)) (list (head l))}})");

    b.iter(|| {
        test::black_box(rlisp.execute("(rev {1 2 3})"));
    });
}