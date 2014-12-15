extern crate rlisp;

use self::rlisp::Rlisp;

#[test]
fn test_rlisp() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a} 12)"), "()");

    assert_eq!(rlisp.execute("(+ a a)"), "24");

    assert_eq!(rlisp.execute("(- a 4)"), "8");

    assert_eq!(rlisp.execute("(eval (head {(+ 1 2) (+ 10 20)}))"), "3");

    assert_eq!(rlisp.execute("((lambda {a} {+ a 1}) 2)"), "3");

    assert_eq!(rlisp.execute("((lambda {a} {+ a 1}) a)"), "13");

    assert_eq!(rlisp.execute("(def {addgen} (lambda {a} {lambda {} {+ a 1}}))"), "()");
    assert_eq!(rlisp.execute("(def {adder} (addgen 1))"), "()");
    assert_eq!(rlisp.execute("(adder)"), "2");

    assert_eq!(rlisp.execute("(def {defgen} (lambda {a} {lambda {} {def {o} a}}))"), "()");
    assert_eq!(rlisp.execute("(def {defer} (defgen 10))"), "()");
    assert_eq!(rlisp.execute("(defer)"), "()");
    assert_eq!(rlisp.execute("o"), "10");

    assert_eq!(rlisp.execute("((lambda {x y} {+ x y}) 10 20)"), "30");

    assert_eq!(rlisp.execute("(def {add-mul} (lambda {x y} {+ x (* x y)}))"), "()");
    assert_eq!(rlisp.execute("(add-mul 10 20)"), "210");

    assert_eq!(rlisp.execute("(def {add-mul-10} (add-mul 10))"), "()");
    assert_eq!(rlisp.execute("(add-mul-10 50)"), "510");

    assert_eq!(rlisp.execute("(def {f x y} {+ x (/ (+ x x) y)})"), "()");
    assert_eq!(rlisp.execute("(f 10 20)"), "11");

    assert_eq!(rlisp.execute("(def {hof f} {f 12 12})"), "()");
    assert_eq!(rlisp.execute("(hof +)"), "24");
    assert_eq!(rlisp.execute("(hof (lambda {a b} {+ a (/ b 2)}))"), "18");

    assert_eq!(rlisp.execute("(def {elipse ...} {head (list ...)})"), "()");
    assert_eq!(rlisp.execute("(elipse 1 2 3)"), "{1 2 3}");
    assert_eq!(rlisp.execute("(elipse 1 2 3 4 5)"), "{1 2 3 4 5}");
 }

#[test]
fn test_elipsis_empty_arg_list() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a ...} {head (list ...)})"), "()");

    assert_eq!(rlisp.execute("(a)"), "{}");
}

#[test]
fn test_curried_builtin() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a} (* 5))"), "()");

    assert_eq!(rlisp.execute("(a 10)"), "50");
}

#[test]
fn test_no_input() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute(""), "()");
}

#[test]
fn test_utf8_string() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("\"ߚ\""), "\"ߚ\"");
}

#[test]
fn test_unpack() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(unpack + {1 2 3 4 5 6})"), "21");

    assert_eq!(rlisp.execute("(def {add-unpacked} (unpack +))"), "()");

    assert_eq!(rlisp.execute("(add-unpacked {1 2 3 4 5 6})"), "21");
}

#[test]
fn test_pack() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(pack head 5 6 7)"), "5");
}

#[test]
fn test_no_extra_args() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("((* 1))"), "Error: func: (* Integer|Float) got no arguments");

    assert_eq!(rlisp.execute("(def {mul a b} {* a b})"), "()");

    assert_eq!(rlisp.execute("((mul 5))"), "Error: (lambda {b} {* a b}) got no arguments");
}

#[test]
fn test_comparisons() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(> 1 2)"), "#f");
    assert_eq!(rlisp.execute("(> 2 1)"), "#t");
    assert_eq!(rlisp.execute("(> 2 2)"), "#f");

    assert_eq!(rlisp.execute("(< 1 2)"), "#t");
    assert_eq!(rlisp.execute("(< 2 1)"), "#f");
    assert_eq!(rlisp.execute("(< 2 2)"), "#f");

    assert_eq!(rlisp.execute("(>= 1 2)"), "#f");
    assert_eq!(rlisp.execute("(>= 2 1)"), "#t");
    assert_eq!(rlisp.execute("(>= 2 2)"), "#t");

    assert_eq!(rlisp.execute("(<= 1 2)"), "#t");
    assert_eq!(rlisp.execute("(<= 2 1)"), "#f");
    assert_eq!(rlisp.execute("(<= 2 2)"), "#t");

    assert_eq!(rlisp.execute("(and #f #f)"), "#f");
    assert_eq!(rlisp.execute("(and #t #f)"), "#f");
    assert_eq!(rlisp.execute("(and #t #t)"), "#t");
    assert_eq!(rlisp.execute("(and #t #t #f)"), "#f");
    assert_eq!(rlisp.execute("(and #t #t #t)"), "#t");

    assert_eq!(rlisp.execute("(or #f #f)"), "#f");
    assert_eq!(rlisp.execute("(or #t #f)"), "#t");
    assert_eq!(rlisp.execute("(or #t #t)"), "#t");
    assert_eq!(rlisp.execute("(or #t #t #f)"), "#t");
    assert_eq!(rlisp.execute("(or #f #f #f)"), "#f");

    assert_eq!(rlisp.execute("(not #t)"), "#f");
    assert_eq!(rlisp.execute("(not #f)"), "#t");

    assert_eq!(rlisp.execute("(== {1 2 3 {4 5}} {1 2 3 {4 5}})"), "#t");
    assert_eq!(rlisp.execute("(== {1 2 3 {4 5}} {1 2 3 {4 6}})"), "#f");

    assert_eq!(rlisp.execute("(!= {1 2 3 {4 5}} {1 2 3 {4 5}})"), "#f");
    assert_eq!(rlisp.execute("(!= {1 2 3 {4 5}} {1 2 3 {4 6}})"), "#t");
}

#[test]
fn test_if() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(if #t 1 2)"), "1");
    assert_eq!(rlisp.execute("(if #f 1 2)"), "2");

    assert_eq!(rlisp.execute("(if (== 1 1) {+ 1 1} {+ 2 2})"), "2");
    assert_eq!(rlisp.execute("(if (== 1 2) {+ 1 1} {+ 2 2})"), "4");

    assert_eq!(rlisp.execute("(def {x} 100)"), "()");
    assert_eq!(rlisp.execute("(def {y} 200)"), "()");

    assert_eq!(rlisp.execute("(if (== x y) {+ x y} {- x y})"), "-100");
}

#[test]
fn test_recursion() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {rev l} {if (== l {}) {list} {join (rev (tail l)) (list (head l))}})"), "()");
    assert_eq!(rlisp.execute("(rev {1 2 3})"), "{3 2 1}");
}