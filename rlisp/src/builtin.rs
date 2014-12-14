use cell::Cell::*;
use cell::{Cell, LambdaSpec};
use environment::Environment;

fn internal_error() -> Cell {
    panic!("Internal type error");
}

pub fn add(_: Environment, args: &[Cell]) -> Cell {
    let mut is_float: bool = false;
    let mut isum: i64 = 0;
    let mut fsum: f64 = 0.0;

    for arg in args.iter() {
         match arg {
            &Integer(a) => {
                if is_float {
                    fsum += a as f64;
                } else {
                    isum += a;
                }
            },
            &Float(a) => {
                if is_float {
                    fsum += a;
                } else {
                    is_float = true;
                    fsum += isum as f64 + a;
                }
            },
            _ => return internal_error(),
         }
    }

    if is_float {
        Float(fsum)
    } else {
        Integer(isum)
    }
}

#[test]
fn test_add() {
    let e = Environment::new();
    assert_eq!(add(e.clone(), &[Integer(2), Integer(3), Integer(2)]), Integer(7));
    assert_eq!(add(e.clone(), &[Integer(2), Integer(3), Float(5.0)]), Float(10.0));
    assert_eq!(add(e.clone(), &[Integer(2), Integer(3)]), Integer(5));
    assert_eq!(add(e.clone(), &[Integer(3), Float(3.0)]), Float(6.0));
    assert_eq!(add(e.clone(), &[Float(3.0), Integer(3)]), Float(6.0));
    assert_eq!(add(e.clone(), &[Float(3.0), Float(5.0)]), Float(8.0));
    assert_eq!(add(e.clone(), &[Float(3.0), Float(5.0)]), Float(8.0));
}

pub fn sub(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Integer(a)]             => Integer(-a),
        [Float(a)]               => Float(-a),
        [Integer(a), Integer(b)] => Integer(a - b),
        [Float(a),   Integer(b)] => Float(a - b as f64),
        [Integer(a), Float(b)]   => Float(a as f64 - b),
        [Float(a),   Float(b)]   => Float(a - b),
        _                        => internal_error(),
    }
}

#[test]
fn test_sub() {
    let e = Environment::new();
    assert_eq!(sub(e.clone(), &[Integer(2)]), Integer(-2));
    assert_eq!(sub(e.clone(), &[Float(3.0)]), Float(-3.0));
    assert_eq!(sub(e.clone(), &[Integer(2), Integer(3)]), Integer(-1));
    assert_eq!(sub(e.clone(), &[Integer(3), Float(3.0)]), Float(0.0));
    assert_eq!(sub(e.clone(), &[Float(3.0), Integer(3)]), Float(0.0));
    assert_eq!(sub(e.clone(), &[Float(3.0), Float(5.0)]), Float(-2.0));
}

pub fn mul(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Integer(a), Integer(b)] => Integer(a * b),
        [Float(a),   Integer(b)] => Float(a * b as f64),
        [Integer(a), Float(b)]   => Float(a as f64 * b),
        [Float(a),   Float(b)]   => Float(a * b),
        _                        => internal_error(),
    }
}

#[test]
fn test_mul() {
    let e = Environment::new();
    assert_eq!(mul(e.clone(), &[Integer(2), Integer(3)]), Integer(6));
    assert_eq!(mul(e.clone(), &[Integer(3), Float(3.0)]), Float(9.0));
    assert_eq!(mul(e.clone(), &[Float(3.0), Integer(3)]), Float(9.0));
    assert_eq!(mul(e.clone(), &[Float(3.0), Float(5.0)]), Float(15.0));
}

pub fn div(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Integer(_), Integer(0)] |
        [Integer(_), Float(0.0)] |
        [Float(_), Integer(0)]   |
        [Float(_), Float(0.0)]   => Error("/, 2, can't divide by zero".to_string()),
        [Integer(a), Integer(b)] => Integer(a / b),
        [Float(a),   Integer(b)] => Float(a / b as f64),
        [Integer(a), Float(b)]   => Float(a as f64 / b),
        [Float(a),   Float(b)]   => Float(a / b),
        _                        => internal_error(),
    }
}

#[test]
fn test_div() {
    let e = Environment::new();
    assert_eq!(div(e.clone(), &[Integer(8), Integer(4)]), Integer(2));
    assert_eq!(div(e.clone(), &[Integer(4), Float(2.0)]), Float(2.0));
    assert_eq!(div(e.clone(), &[Float(2.0), Integer(4)]), Float(0.5));
    assert_eq!(div(e.clone(), &[Float(2.0), Float(0.5)]), Float(4.0));
    assert_eq!(div(e.clone(), &[Integer(8), Integer(0)]), Error("/, 2, can't divide by zero".to_string()));
    assert_eq!(div(e.clone(), &[Integer(8), Float(0.0)]), Error("/, 2, can't divide by zero".to_string()));
    assert_eq!(div(e.clone(), &[Float(2.0), Integer(0)]), Error("/, 2, can't divide by zero".to_string()));
    assert_eq!(div(e.clone(), &[Float(2.0), Float(0.0)]), Error("/, 2, can't divide by zero".to_string()));
}

pub fn print(_: Environment, args: &[Cell]) -> Cell {
    for arg in args.iter() {
        println!("{}", arg);    
    }
    Nil
}

pub fn list(_: Environment, args: &[Cell]) -> Cell {
    Qexpr(args.to_vec())
}

#[test]
fn test_list() {
    let e = Environment::new();
    assert_eq!(list(e.clone(), &[Integer(1), Integer(2), Integer(3)]), Qexpr(vec![Integer(1), Integer(2), Integer(3)]));
}

pub fn head(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v)] => v[0].clone(),
        _              => internal_error(),
    }
}

#[test]
fn test_head() {
    let e = Environment::new();
    assert_eq!(head(e.clone(), &[Qexpr(vec![Integer(1), Integer(2), Integer(3)])]), Integer(1));
}

pub fn tail(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v)] => Qexpr(v[1..].to_vec()),
        _              => internal_error(),
    }
}

#[test]
fn test_tail() {
    let e = Environment::new();
    assert_eq!(tail(e.clone(), &[Qexpr(vec![Integer(1), Integer(2), Integer(3)])]), Qexpr(vec![Integer(2), Integer(3)]));
}

pub fn init(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v)] => Qexpr(v[..v.len() - 1].to_vec()),
        _              => internal_error(),
    }
}

#[test]
fn test_init() {
    let e = Environment::new();
    assert_eq!(init(e.clone(), &[Qexpr(vec![Integer(1), Integer(2), Integer(3)])]), Qexpr(vec![Integer(1), Integer(2)]));
}

pub fn join(_: Environment, args: &[Cell]) -> Cell {
    let mut res: Vec<Cell> = Vec::new();

    for arg in args.iter() {
         match arg {
            &Qexpr(ref v) => res.push_all(v[]),
            _             => return internal_error(),
         }
    }

    Qexpr(res)
}

#[test]
fn test_join() {
    let e = Environment::new();
    assert_eq!(join(e.clone(), &[Qexpr(vec![Integer(1), Integer(2), Integer(3)]),
                                 Qexpr(vec![Integer(4), Integer(5), Integer(6)]),
                                 Qexpr(vec![Integer(1), Integer(2), Integer(3)])]),
               Qexpr(vec![Integer(1), Integer(2), Integer(3), Integer(4), Integer(5), Integer(6), Integer(1), Integer(2), Integer(3)]));
}

pub fn len(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v)] => Integer(v.len() as i64),
        _              => internal_error(),
    }
}

#[test]
fn test_len() {
    let e = Environment::new();
    assert_eq!(len(e.clone(), &[Qexpr(vec![Integer(1), Integer(2), Integer(3)])]), Integer(3));
}

pub fn eval(env: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v)] => super::eval(env, &Sexpr(v.clone())),
        [expr]         => super::eval(env, expr),
        _              => internal_error(),
    }
}

#[test]
fn test_eval() {
    let e = Environment::new();
    let add_f = e.lookup(&"+".to_string());
    assert_eq!(eval(e.clone(), &[Qexpr(vec![add_f, Integer(1), Integer(2), Integer(3)])]), Integer(6));
}

pub fn eq(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [ref a, ref b] => Bool(*a == *b),
        _              => internal_error(),
    }
}

#[test]
fn test_eq() {
    let e = Environment::new();
    let eq_f = e.lookup(&"==".to_string());
    let ne_f = e.lookup(&"!=".to_string());
    assert_eq!(eq(e.clone(), &[Integer(1), Integer(1)]), Bool(true));
    assert_eq!(eq(e.clone(), &[Integer(1), Integer(2)]), Bool(false));
    assert_eq!(eq(e.clone(), &[Integer(1), Float(1.0)]), Bool(false));
    assert_eq!(eq(e.clone(), &[eq_f.clone(), eq_f.clone()]), Bool(true));
    assert_eq!(eq(e.clone(), &[eq_f.clone(), ne_f.clone()]), Bool(false));
}

pub fn ne(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [ref a, ref b] => Bool(*a != *b),
        _              => internal_error(),
    }
}

#[test]
fn test_ne() {
    let e = Environment::new();
    let eq_f = e.lookup(&"==".to_string());
    let ne_f = e.lookup(&"!=".to_string());
    assert_eq!(ne(e.clone(), &[Integer(1), Integer(1)]), Bool(false));
    assert_eq!(ne(e.clone(), &[Integer(1), Integer(2)]), Bool(true));
    assert_eq!(ne(e.clone(), &[Integer(1), Float(1.0)]), Bool(true));
    assert_eq!(ne(e.clone(), &[eq_f.clone(), eq_f.clone()]), Bool(false));
    assert_eq!(ne(e.clone(), &[eq_f.clone(), ne_f.clone()]), Bool(true));
}

pub fn lt(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [&Integer(ref a), &Integer(ref b)] => *a < *b,
        [&Float(ref a), &Float(ref b)]     => *a < *b,
        [&Char(ref a), &Char(ref b)]       => *a < *b,
        [&Str(ref a), &Str(ref b)]         => *a < *b,
        _ => false,
    }
}

pub fn lte(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [&Integer(ref a), &Integer(ref b)] => *a <= *b,
        [&Float(ref a), &Float(ref b)]     => *a <= *b,
        [&Char(ref a), &Char(ref b)]       => *a <= *b,
        [&Str(ref a), &Str(ref b)]         => *a <= *b,
        _ => false,
    }
}

pub fn gt(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [&Integer(ref a), &Integer(ref b)] => *a > *b,
        [&Float(ref a), &Float(ref b)]     => *a > *b,
        [&Char(ref a), &Char(ref b)]       => *a > *b,
        [&Str(ref a), &Str(ref b)]         => *a > *b,
        _ => false,
    }
}

pub fn gte(_: Environment, args: &[Cell]) -> Cell {
    match args {
        [&Integer(ref a), &Integer(ref b)] => *a >= *b,
        [&Float(ref a), &Float(ref b)]     => *a >= *b,
        [&Char(ref a), &Char(ref b)]       => *a >= *b,
        [&Str(ref a), &Str(ref b)]         => *a >= *b,
        _ => false,
    }
}

pub fn and(_: Environment, args: &[Cell]) -> Cell {
    for arg in args {
        match arg {
            Bool(ref a) => { if !a { return Bool(false); } }
            _           => return Bool(false),
        }
    }
    Bool(true)
}

pub fn or(_: Environment, args: &[Cell]) -> Cell {
    for arg in args {
        match arg {
            Bool(ref a) => { if a { return Bool(true); } }
            _           => return Bool(false),
        }
    }
    Bool(false)
}

pub fn if_func(env: Environment, args: &[Cell]) -> Cell {
     match args {
        [&Bool(ref cond), _, _] => {
            if cond {
                eval(env.clone(), args[1..1])
            } else {
                eval(env.clone(), args[2..2])
            }
        },
        _ => internal_error(),
    }
}

pub fn def(env: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v), ref b] => match (v[], b) {
            ([Symbol(ref s)], b) => {
                env.insert_top(s, b);
                Nil
            },
            ([Symbol(ref s), args..], &Qexpr(ref body)) => {
                let lambda = Lambda(box LambdaSpec {
                    arguments:   args.to_vec(),
                    body:        body.clone(),
                    environment: env.clone(),
                });
                env.insert_top(s, &lambda);
                Nil
            },
            ([Symbol(_), ..], _) => {
                Error("def, when defining a function argument 2 has to be a List".to_string())
            }
            _  => internal_error(),
        },
        _ => internal_error(),
    }
}

pub fn set(env: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref v), ref b] => match v[] {
            [Symbol(ref s)] => {
                env.insert(s, b);
                Nil
            },
            _  => internal_error(),
        },
        _ => internal_error(),
    }
}

pub fn lambda(env: Environment, args: &[Cell]) -> Cell {
    match args {
        [Qexpr(ref args), Qexpr(ref body)] => {
            Lambda(box LambdaSpec {
                arguments:   args.clone(),
                body:        body.clone(),
                environment: env,
            })
        },
        _ => internal_error(),
    }
}