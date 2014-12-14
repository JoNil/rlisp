#![feature(globs)]
#![feature(phase)]
#![feature(slicing_syntax)]

extern crate mpc;
extern crate phf;

#[phase(plugin)]
extern crate phf_mac;

use cell::{Cell, CurriedBuiltinSpec, LambdaSpec};
use environment::Environment;
use parser::Parser;

mod builtin;
mod cell;
mod environment;
mod globals;
mod parser;
mod stdlib;
mod types;

pub struct Rlisp<'a> {
    parser: Parser,
    environment: Environment,
}

impl<'a> Rlisp<'a> {
    pub fn new() -> Rlisp<'a> {
        let mut rlisp = Rlisp {
            parser: Parser::new(),
            environment: Environment::new(),
        };

        stdlib::inject(&mut rlisp);

        rlisp
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
            } else if evaled_args.len() == 0 {
                Cell::Error(format!("{} got no arguments", procedure))
            } else {
                Cell::CurriedBuiltin(box CurriedBuiltinSpec {
                    builtin: cb.builtin,
                    bound_args: evaled_and_bound_args,
                })
            }            
        },
        _ => Cell::Error(format!("{} is not a procedure, is {}", *procedure, procedure.get_type())),
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