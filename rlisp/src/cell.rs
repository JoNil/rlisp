use std::fmt;
#[cfg(test)]
use std::mem;

use environment::Environment;
use types::Type::*;
use types::Type;

use self::Cell::*;

pub type BuiltinFunction = fn (env: Environment, args: &[Cell]) -> Cell;

pub struct BuiltinFunctionSpec {
    pub func: BuiltinFunction,
    pub name: &'static str,
    pub argument_types: &'static [Type],
}

#[deriving(Clone)]
pub struct CurriedBuiltinSpec {
    pub builtin: &'static BuiltinFunctionSpec,
    pub bound_args: Vec<Cell>,
}

#[deriving(Clone)]
pub struct LambdaSpec {
    pub arguments: Vec<Cell>,
    pub body: Vec<Cell>,
    pub environment: Environment,
}

#[deriving(Clone)]
pub enum Cell {
    Nil,
    Symbol(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    Str(String),
    Sexpr(Vec<Cell>),
    Qexpr(Vec<Cell>),
    Error(String),
    Builtin(&'static BuiltinFunctionSpec),
    CurriedBuiltin(Box<CurriedBuiltinSpec>),
    Lambda(Box<LambdaSpec>),
}

#[test]
fn test_cell_size() {
    assert_eq!(mem::size_of::<Cell>(), 32);
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        match (self, other) {
            (&Nil, &Nil)                       => true,
            (&Symbol(ref a), &Symbol(ref b))   => *a == *b,
            (&Integer(ref a), &Integer(ref b)) => *a == *b,
            (&Float(ref a), &Float(ref b))     => *a == *b,
            (&Char(ref a), &Char(ref b))       => *a == *b,
            (&Bool(ref a), &Bool(ref b))       => *a == *b,
            (&Str(ref a), &Str(ref b))         => *a == *b,
            (&Sexpr(ref a), &Sexpr(ref b))     => *a == *b,
            (&Qexpr(ref a), &Qexpr(ref b))     => *a == *b,
            (&Error(ref a), &Error(ref b))     => *a == *b,
            (&Builtin(ref a), &Builtin(ref b)) => a.func as *const u8 == b.func as *const u8,
            (&CurriedBuiltin(ref a),
             &CurriedBuiltin(ref b))           => a.builtin.func as *const u8 == b.builtin.func as *const u8,
            (&Lambda(ref a), &Lambda(ref b))   => a.arguments == b.arguments && a.body == b.body,
            _ => false,
        }
    }
}

impl Cell {
    pub fn get_type(&self) -> Type {
        match *self {
            Nil               => NilT,
            Symbol(_)         => SymbolT,
            Integer(_)        => IntegerT,
            Float(_)          => FloatT,
            Char(_)           => CharT,
            Bool(_)           => BoolT,
            Str(_)            => StringT,
            Sexpr(ref v)      => RSexprT(v.iter().map(|e| e.get_type()).collect()),
            Qexpr(ref v)      => RQexprT(v.iter().map(|e| e.get_type()).collect()),
            Error(_)          => ErrorT,
            Builtin(_)        => BuiltinT,
            CurriedBuiltin(_) => BuiltinT,
            Lambda(_)         => LambdaT,
        }
    }

    pub fn is_type(&self, t: &Type) -> bool {
        match (self, t) {
            (&Nil, &NilT)                     => true,
            (&Symbol(_), &SymbolT)            => true,
            (&Integer(_), &IntegerT)          => true,
            (&Float(_), &FloatT)              => true,
            (&Char(_), &CharT)                => true,
            (&Bool(_), &BoolT)                => true,
            (&Str(_), &StringT)               => true,
            (&Sexpr(ref v), &SexprT(ref vt))  => v.iter().zip(vt.iter()).all(|(e, t)| e.is_type(t)),
            (&Qexpr(ref v), &QexprT(ref vt))  => v.iter().zip(vt.iter()).all(|(e, t)| e.is_type(t)),
            (&Sexpr(ref v), &RSexprT(ref vt)) => v.iter().zip(vt.iter()).all(|(e, t)| e.is_type(t)),
            (&Qexpr(ref v), &RQexprT(ref vt)) => v.iter().zip(vt.iter()).all(|(e, t)| e.is_type(t)),
            (&Error(_), &ErrorT)              => true,
            (&Builtin(_), &BuiltinT)          => true,
            (&CurriedBuiltin(_), &BuiltinT)   => true,
            (&Lambda(_), &LambdaT)            => true,
            (_, &AnyT)                        => true,
            (_, &ElipsisT(inner))             => self.is_type(inner),
            (_, &OptionalT(inner))            => self.is_type(inner),
            (_, &OrT(i1, i2))                 => self.is_type(i1) || self.is_type(i2),
            _                                 => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            &Nil             => "()".to_string(),
            &Symbol(ref sym) => format!("{}", sym),
            &Integer(ref i)  => format!("{}", i),
            &Float(ref fl)   => format!("{:.2}", fl),
            &Char(ref c)     => format!("'{}'", c),
            &Bool(ref b)     => if *b { "#t".to_string() } else { "#f".to_string() },
            &Str(ref s)      => format!("\"{}\"", s),
            &Sexpr(ref v)    => {
                let mut temp: String = "(".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i])[]);
                    } else {
                        temp.push_str(format!("{} ", v[i])[]);
                    }
                }
                temp.push_str(")");
                temp
            },
            &Qexpr(ref v) => {
                let mut temp: String = "{".to_string();
                for i in range(0, v.len()) {
                    if i == v.len() - 1 {
                        temp.push_str(format!("{}", v[i])[]);
                    } else {
                        temp.push_str(format!("{} ", v[i])[]);
                    }
                }
                temp.push_str("}");
                temp
            },
            &Error(ref e)    => format!("Error: {}", e),
            &Builtin(ref f)  => {
                let mut temp: String = String::new();
                for (i, t) in f.argument_types.iter().enumerate() {
                    if i == f.argument_types.len() - 1 {
                        temp.push_str(format!("{}", t)[]);
                    } else {
                        temp.push_str(format!("{} ", t)[]);
                    }
                }
                format!("func: ({} {})", f.name, temp)
            },
            &CurriedBuiltin(box ref cb) => {
                let mut temp: String = String::new();
                for (i, t) in cb.builtin.argument_types.iter().enumerate() {
                    if i >= cb.bound_args.len() {
                        if i == cb.builtin.argument_types.len() - 1 {
                            temp.push_str(format!("{}", t)[]);
                        } else {
                            temp.push_str(format!("{} ", t)[]);
                        }
                    }
                }
                format!("func: ({} {})", cb.builtin.name, temp)
            },
            &Lambda(ref l)          => format!("(lambda {} {})", Qexpr(l.arguments.clone()), Qexpr(l.body.clone())),
        }
    }
}

impl fmt::Show for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_fmt() {
    assert_eq!(format!("{}", &Nil), "()".to_string());
    assert_eq!(format!("{}", &Symbol("Hej".to_string())), "Hej".to_string());
    assert_eq!(format!("{}", &Integer(-1)), "-1".to_string());
    assert_eq!(format!("{}", &Float(5.54321012)), "5.54".to_string());
    assert_eq!(format!("{}", &Char('a')), "'a'".to_string());
    assert_eq!(format!("{}", &Bool(true)), "#t".to_string());
    assert_eq!(format!("{}", &Bool(false)), "#f".to_string());
    assert_eq!(format!("{}", &Str("Hej".to_string())), "\"Hej\"".to_string());
    assert_eq!(format!("{}", &Sexpr(vec![Nil, Integer(1)])), "(() 1)".to_string());
    assert_eq!(format!("{}", &Qexpr(vec![Nil, Integer(1)])), "{() 1}".to_string());
    assert_eq!(format!("{}", &Sexpr(Vec::new())), "()".to_string());
    assert_eq!(format!("{}", &Qexpr(Vec::new())), "{}".to_string());
    assert_eq!(format!("{}", &Error("Error".to_string())), "Error: Error".to_string());
    assert_eq!(format!("{}", &Float(5.0)), "5.00".to_string());
}
