use std::fmt;
use std::str::Str as StrTrait;
use std::string::CowString;

use cell;
use self::Type::*;

#[cfg(test)]
use cell::Cell::*;
#[cfg(test)]
use globals;

#[derive(Clone)]
pub enum Type {
    NilT,
    SymbolT,
    IntegerT,
    FloatT,
    CharT,
    BoolT,
    StringT,
    SexprT(&'static [Type]),
    QexprT(&'static [Type]),
    RSexprT(Vec<Type>),
    RQexprT(Vec<Type>),
    ErrorT,
    BuiltinT,
    LambdaT,
    AnyT,
    ElipsisT(&'static Type),
    OptionalT(&'static Type),
    OrT(&'static Type, &'static Type),
}

pub struct Arity {
    pub requierd: i32,
    pub optional: i32,
    pub elipsis: bool,
}

impl Type {
    pub fn to_string(&self) -> CowString {
        use std::borrow::Cow::{Borrowed, Owned};

        match *self {
            NilT     => Borrowed("Nil"),
            SymbolT  => Borrowed("Symbol"),
            IntegerT => Borrowed("Integer"),
            FloatT   => Borrowed("Float"),
            CharT    => Borrowed("Char"),
            BoolT    => Borrowed("Bool"),
            StringT  => Borrowed("String"),
            SexprT(ref v) => {
                let mut temp: String = "(".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i]).as_slice());
                    } else {
                        temp.push_str(format!("{} ", v[i]).as_slice());
                    }
                }
                temp.push_str(")");
                Owned(temp)
            },
            QexprT(ref v) => {
                let mut temp: String = "{".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i]).as_slice());
                    } else {
                        temp.push_str(format!("{} ", v[i]).as_slice());
                    }
                }
                temp.push_str("}");
                Owned(temp)
            },
            RSexprT(ref v) => {
                let mut temp: String = "(".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i]).as_slice());
                    } else {
                        temp.push_str(format!("{} ", v[i]).as_slice());
                    }
                }
                temp.push_str(")");
                Owned(temp)
            },
            RQexprT(ref v) => {
                let mut temp: String = "{".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i]).as_slice());
                    } else {
                        temp.push_str(format!("{} ", v[i]).as_slice());
                    }
                }
                temp.push_str("}");
                Owned(temp)
            },
            ErrorT               => Borrowed("Error"),
            BuiltinT             => Borrowed("Builtin"),
            LambdaT              => Borrowed("Lambda"),
            AnyT                 => Borrowed("Any"),
            ElipsisT(ref inner)  => Owned(format!("{}...", inner)),
            OptionalT(ref inner) => Owned(format!("[{}]", inner)),
            OrT(ref i1, ref i2)  => Owned(format!("{}|{}", i1, i2)),
        }
    }
}

impl fmt::String for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Show for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn get_arity(argument_types: &[Type]) -> Arity {
    let mut arity = Arity {
        requierd: 0,
        optional: 0,
        elipsis: false,
    };

    let mut found_optional = false;
    let mut found_elipsis = false;

    for t in argument_types.iter() {
        match t {
            &ElipsisT(_) => {
                arity.elipsis = true;
                found_elipsis = true;
            },
            &OptionalT(_) => {
                if found_elipsis {
                    panic!("Internal type error");
                }
                found_optional = true;
                arity.optional += 1;
            },
            _ => {
                if found_optional {
                    panic!("Internal type error");
                }
                if found_elipsis {
                    panic!("Internal type error");
                }
                arity.requierd += 1;
            }
        }
    }

    arity
}

fn validate_inner(argument_types: &[Type], args: &[cell::Cell]) -> Option<String> {
    let mut arg_type_iter = argument_types.iter();
    let mut arg_type = arg_type_iter.next().expect("Internal error");

    for (i, arg) in args.iter().enumerate() {

        match (arg, arg_type) {
            (&cell::Cell::Sexpr(ref v), &SexprT(ref vt)) => {
                if let Some(e) = validate_inner(vt.as_slice(), v.as_slice()) {
                    return Some(format!("argument list at {}: {}", i+1, e));
                }
            },
            (&cell::Cell::Sexpr(ref v), &RSexprT(ref vt)) => {
                if let Some(e) = validate_inner(vt.as_slice(), v.as_slice()) {
                    return Some(format!("argument list at {}: {}", i+1, e));
                }
            },
            (&cell::Cell::Qexpr(ref v), &QexprT(ref vt)) => {
                if let Some(e) = validate_inner(vt.as_slice(), v.as_slice()) {
                    return Some(format!("argument list at {}: {}", i+1, e));
                }
            },
            (&cell::Cell::Qexpr(ref v), &RQexprT(ref vt)) => {
                if let Some(e) = validate_inner(vt.as_slice(), v.as_slice()) {
                    return Some(format!("argument list at {}: {}", i+1, e));
                }
            },
            (_, _) => (),
        }

        if !arg.is_type(arg_type) {
            return Some(format!("argument {} is of type {} expected {}",
                                i+1, arg.get_type(), arg_type));
        }

        if let &ElipsisT(_) = arg_type {} else {
            arg_type = match arg_type_iter.next() {
                Some(t) => t,
                None    => break,
            }; 
        }
    }

    let arity = get_arity(argument_types);

    if arity.optional == 0 {
        if arity.elipsis && ((args.len() as i32) < arity.requierd) {
            return Some(format!("requiers {}+ arguments, got {}",
                                arity.requierd, args.len()));
        } else if !arity.elipsis && arity.requierd != args.len() as i32 {
            return Some(format!("requiers {} arguments, got {}",
                                arity.requierd, args.len()));
        }
    } else {
        if arity.elipsis && ((args.len() as i32) < arity.requierd) {
            return Some(format!("requiers {}+ arguments, got {}",
                                arity.requierd, args.len()));
        } else if !arity.elipsis && ((args.len() as i32) < arity.requierd ||
                                     (args.len() as i32) > arity.requierd + arity.optional)  {
            return Some(format!("requiers {} to {} arguments, got {}",
                                arity.requierd, arity.requierd + arity.optional, args.len()));
        }
    }

    return None;
}

pub fn validate(f: &cell::BuiltinFunctionSpec, args: &[cell::Cell]) -> Option<String> {
    match validate_inner(f.argument_types.as_slice(), args) {
        Some(e) => Some(format!("{}, {}", f.name, e)),
        None    => None,
    }
}

#[test]
fn test_validate() {
    let stub_sub = globals::GLOBAL_ENVIROMENT.get("-").unwrap();

    assert_eq!(validate(stub_sub, &[Integer(1), Integer(2)]), None);
    assert_eq!(validate(stub_sub, &[Integer(1), Integer(2), Integer(3)]), Some("-, requiers 1 to 2 arguments, got 3".to_string()));
    assert_eq!(validate(stub_sub, &[Integer(1)]), None);
    assert_eq!(validate(stub_sub, &[]), Some("-, requiers 1 to 2 arguments, got 0".to_string()));
    assert_eq!(validate(stub_sub, &[Char('a')]), Some("-, argument 1 is of type Char expected Integer|Float".to_string()));
    assert_eq!(validate(stub_sub, &[Integer(1), Char('a')]), Some("-, argument 2 is of type Char expected [Integer|Float]".to_string()));

    let stub_mul = globals::GLOBAL_ENVIROMENT.get("*").unwrap();

    assert_eq!(validate(stub_mul, &[Integer(1), Integer(2)]), None);
    assert_eq!(validate(stub_mul, &[Integer(1), Integer(2), Integer(3)]), Some("*, requiers 2 arguments, got 3".to_string()));
    assert_eq!(validate(stub_mul, &[Integer(1)]), Some("*, requiers 2 arguments, got 1".to_string()));
    assert_eq!(validate(stub_mul, &[]), Some("*, requiers 2 arguments, got 0".to_string()));
    assert_eq!(validate(stub_mul, &[Char('a')]), Some("*, argument 1 is of type Char expected Integer|Float".to_string()));
    assert_eq!(validate(stub_mul, &[Integer(1), Char('a')]), Some("*, argument 2 is of type Char expected Integer|Float".to_string()));

    let stub_add = globals::GLOBAL_ENVIROMENT.get("+").unwrap();

    assert_eq!(validate(stub_add, &[]), Some("+, requiers 1+ arguments, got 0".to_string()));
    assert_eq!(validate(stub_add, &[Integer(1)]), None);
    assert_eq!(validate(stub_add, &[Char('a')]), Some("+, argument 1 is of type Char expected Integer|Float".to_string()));
    assert_eq!(validate(stub_add, &[Integer(1), Char('a')]), Some("+, argument 2 is of type Char expected Integer|Float...".to_string()));

    let stub_def = globals::GLOBAL_ENVIROMENT.get("def").unwrap();

    assert_eq!(validate(stub_def, &[]), Some("def, requiers 2 arguments, got 0".to_string()));
    assert_eq!(validate(stub_def, &[Qexpr(vec![Symbol("hej".to_string())]), Integer(1)]), None);
    assert_eq!(validate(stub_def, &[Qexpr(vec![Integer(1)]), Integer(1)]), Some("def, argument list at 1: argument 1 is of type Integer expected Symbol".to_string()));
    assert_eq!(validate(stub_def, &[Qexpr(vec![Symbol("hej".to_string()), Integer(1)]), Integer(1)]), Some("def, argument list at 1: argument 2 is of type Integer expected Symbol...".to_string()));
}