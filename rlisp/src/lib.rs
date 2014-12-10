#![feature(globs)]
#![feature(phase)]
#![feature(slicing_syntax)]

extern crate mpc;
extern crate phf;

#[phase(plugin)]
extern crate phf_mac;

#[cfg(test)]
extern crate test;

use cell::{Cell, CurriedBuiltinSpec, LambdaSpec};
use environment::Environment;
use parser::Parser;

#[cfg(test)]
use self::test::Bencher;

mod types;
mod cell;
mod globals;
mod environment;
mod builtin;
mod parser;

pub struct Rlisp<'a> {
    parser: Parser,
    environment: Environment,
}

impl<'a> Rlisp<'a> {
    pub fn new() -> Rlisp<'a> {
        Rlisp {
            parser: Parser::new(),
            environment: Environment::new(),
        }
    }

    pub fn execute(&mut self, input: &str) -> String {
        format!("{}", eval(self.environment.clone(), &self.parser.parse(input)))
    }
}

fn apply(env: Environment, procedure: &Cell, args: &[Cell]) -> Cell {
    if let Some(e) = first_error(args) {
         return e.clone();
    }
    
    let evaled_args = args.iter().map(|a| eval(env.clone(), a)).collect::<Vec<Cell>>();

    if let Some(e) = first_error(evaled_args[]) {
         return e.clone();
    }

    match procedure {
        &Cell::Lambda(box ref lambda) => {
            let sub_env = lambda.environment.make_sub_environment();

            let mut found_elipsis = false;

            for (i, arg) in evaled_args.iter().enumerate() {
                if let Some(&Cell::Symbol(ref s)) = lambda.arguments.get(i) {
                    match (s[] == "...", i == lambda.arguments.len() - 1) {
                        (true, true)  => { sub_env.insert(s, &Cell::Qexpr(evaled_args[i..].to_vec())); found_elipsis = true; break; },
                        (true, false) => return Cell::Error("Lambda can only have ... as it's last argument".to_string()),
                        (false, _)    => sub_env.insert(s, arg),
                    }
                }
            }

            if evaled_args.len() == 0 {            
                if let Some(&Cell::Symbol(ref s)) = lambda.arguments.get(0) {
                    if s[] == "..." {
                        sub_env.insert(s, &Cell::Qexpr(Vec::new()));
                        found_elipsis = true;
                    }
                }
            }

            if evaled_args.len() == lambda.arguments.len() || found_elipsis {
                eval(sub_env.clone(), &Cell::Sexpr(lambda.body.clone()))
            } else if evaled_args.len() < lambda.arguments.len() {
                if evaled_args.len() == 0 {
                    Cell::Error(format!("{} got no arguments", procedure))
                } else {
                    Cell::Lambda(box LambdaSpec {
                        arguments:   lambda.arguments[evaled_args.len()..].to_vec(),
                        body:        lambda.body.clone(),
                        environment: sub_env.clone(),
                    })
                }
            } else {
                Cell::Error(format!("{} got to many arguments expected {} got {}",
                                    procedure, lambda.arguments.len(), evaled_args.len()))
            }
        },
        &Cell::Builtin(builtin) => {
            let arity = types::get_arity(builtin.argument_types);

            if evaled_args.len() as i32 >= arity.requierd {
                if let Some(e) = types::validate(builtin, evaled_args[]) {
                    return Cell::Error(e);
                }

                (builtin.func)(env, evaled_args[])
            } else {
                Cell::CurriedBuiltin(box CurriedBuiltinSpec {
                    builtin: builtin,
                    bound_args: evaled_args,
                })
            }            
        },
        &Cell::CurriedBuiltin(box ref cb) => {
            let arity = types::get_arity(cb.builtin.argument_types);
            let mut evaled_and_bound_args = Vec::new();
            evaled_and_bound_args.push_all(cb.bound_args[]);
            evaled_and_bound_args.push_all(evaled_args[]);

            if evaled_and_bound_args.len() as i32 >= arity.requierd {
                if let Some(e) = types::validate(cb.builtin, evaled_and_bound_args[]) {
                    return Cell::Error(e);
                }

                (cb.builtin.func)(env, evaled_and_bound_args[])
            } else {
                Cell::CurriedBuiltin(box CurriedBuiltinSpec {
                    builtin: cb.builtin,
                    bound_args: evaled_and_bound_args,
                })
            }            
        },
        _ => Cell::Error(format!("{} is not a procedure, is {}", *procedure, procedure.get_type().to_string())),
    }
}

fn eval(env: Environment, c: &Cell) -> Cell {
    match c {
        &Cell::Sexpr(ref v) => match v[] {
            [ref procedure, args..] => {
                let evaled_procedure = eval(env.clone(), procedure);

                if let Cell::Error(_) = evaled_procedure {
                    return evaled_procedure.clone();
                }

                apply(env.clone(), &evaled_procedure, args)
            },
            _ => c.clone(),
        },
        &Cell::Symbol(ref s) => env.lookup(s),
        _ => c.clone(),
    }
}

fn first_error(c: &[Cell]) -> Option<&Cell> {
    for e in c.iter() {
        if let &Cell::Error(_) = e {
            return Some(e);
        }
    }
    None
}

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