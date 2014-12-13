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