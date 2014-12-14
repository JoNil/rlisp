extern crate rlisp;

use self::rlisp::Rlisp;

#[test]
fn test_rlisp() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a} 12)"), "()".to_string());

    assert_eq!(rlisp.execute("(+ a a)"), "24".to_string());

    assert_eq!(rlisp.execute("(- a 4)"), "8".to_string());

    assert_eq!(rlisp.execute("(eval (head {(+ 1 2) (+ 10 20)}))"), "3".to_string());

    assert_eq!(rlisp.execute("((lambda {a} {+ a 1}) 2)"), "3".to_string());

    assert_eq!(rlisp.execute("((lambda {a} {+ a 1}) a)"), "13".to_string());

    assert_eq!(rlisp.execute("(def {addgen} (lambda {a} {lambda {} {+ a 1}}))"), "()".to_string());
    assert_eq!(rlisp.execute("(def {adder} (addgen 1))"), "()".to_string());
    assert_eq!(rlisp.execute("(adder)"), "2".to_string());

    assert_eq!(rlisp.execute("(def {defgen} (lambda {a} {lambda {} {def {o} a}}))"), "()".to_string());
    assert_eq!(rlisp.execute("(def {defer} (defgen 10))"), "()".to_string());
    assert_eq!(rlisp.execute("(defer)"), "()".to_string());
    assert_eq!(rlisp.execute("o"), "10".to_string());

    assert_eq!(rlisp.execute("((lambda {x y} {+ x y}) 10 20)"), "30".to_string());

    assert_eq!(rlisp.execute("(def {add-mul} (lambda {x y} {+ x (* x y)}))"), "()".to_string());
    assert_eq!(rlisp.execute("(add-mul 10 20)"), "210".to_string());

    assert_eq!(rlisp.execute("(def {add-mul-10} (add-mul 10))"), "()".to_string());
    assert_eq!(rlisp.execute("(add-mul-10 50)"), "510".to_string());

    assert_eq!(rlisp.execute("(def {f x y} {+ x (/ (+ x x) y)})"), "()".to_string());
    assert_eq!(rlisp.execute("(f 10 20)"), "11".to_string());

    assert_eq!(rlisp.execute("(def {hof f} {f 12 12})"), "()".to_string());
    assert_eq!(rlisp.execute("(hof +)"), "24".to_string());
    assert_eq!(rlisp.execute("(hof (lambda {a b} {+ a (/ b 2)}))"), "18".to_string());

    assert_eq!(rlisp.execute("(def {elipse ...} {head (list ...)})"), "()".to_string());
    assert_eq!(rlisp.execute("(elipse 1 2 3)"), "{1 2 3}".to_string());
    assert_eq!(rlisp.execute("(elipse 1 2 3 4 5)"), "{1 2 3 4 5}".to_string());
 }

#[test]
fn test_elipsis_empty_arg_list() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a ...} {head (list ...)})"), "()".to_string());

    assert_eq!(rlisp.execute("(a)"), "{}".to_string());
}

#[test]
fn test_curried_builtin() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(def {a} (* 5))"), "()".to_string());

    assert_eq!(rlisp.execute("(a 10)"), "50".to_string());
}

#[test]
fn test_no_input() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute(""), "()".to_string());
}

#[test]
fn test_utf8_string() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("\"ߚ\""), "\"ߚ\"".to_string());
}

#[test]
fn test_unpack() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(unpack + {1 2 3 4 5 6})"), "21".to_string());
}

#[test]
fn test_pack() {
    let mut rlisp = Rlisp::new();

    assert_eq!(rlisp.execute("(pack head 5 6 7)"), "5".to_string());
}