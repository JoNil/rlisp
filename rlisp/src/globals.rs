use phf;

use builtin;
use cell::BuiltinFunctionSpec;
use types::Type::*;

pub static GLOBAL_ENVIROMENT: phf::Map<&'static str, BuiltinFunctionSpec> = phf_map! {
    // Math
    "+" => BuiltinFunctionSpec {
        func: builtin::add,
        name: "+",
        argument_types: &[OrT(&IntegerT, &FloatT), ElipsisT(&OrT(&IntegerT, &FloatT))],
    },
    "-" => BuiltinFunctionSpec {
        func: builtin::sub,
        name: "-",
        argument_types: &[OrT(&IntegerT, &FloatT), OptionalT(&OrT(&IntegerT, &FloatT))],
    },
    "*" => BuiltinFunctionSpec {
        func: builtin::mul,
        name: "*",
        argument_types: &[OrT(&IntegerT, &FloatT), OrT(&IntegerT, &FloatT)],
    },
    "/" => BuiltinFunctionSpec {
        func: builtin::div,
        name: "/",
        argument_types: &[OrT(&IntegerT, &FloatT), OrT(&IntegerT, &FloatT)],
    },
    // sqrt
    // ^

    // Compare
    "==" => BuiltinFunctionSpec {
        func: builtin::eq,
        name: "==",
        argument_types: &[AnyT, AnyT],
    },
    "!=" => BuiltinFunctionSpec {
        func: builtin::ne,
        name: "!=",
        argument_types: &[AnyT, AnyT],
    },
    "<" => BuiltinFunctionSpec {
        func: builtin::lt,
        name: "<",
        argument_types: &[AnyT, AnyT],
    },
    "<=" => BuiltinFunctionSpec {
        func: builtin::lte,
        name: "<=",
        argument_types: &[AnyT, AnyT],
    },
    ">" => BuiltinFunctionSpec {
        func: builtin::gt,
        name: ">",
        argument_types: &[AnyT, AnyT],
    },
    ">=" => BuiltinFunctionSpec {
        func: builtin::gte,
        name: ">=",
        argument_types: &[AnyT, AnyT],
    },
    "and" => BuiltinFunctionSpec {
        func: builtin::and,
        name: "and",
        argument_types: &[BoolT, ElipsisT(&BoolT)],
    },
    "or" => BuiltinFunctionSpec {
        func: builtin::or,
        name: "or",
        argument_types: &[BoolT, ElipsisT(&BoolT)],
    },
    "not" => BuiltinFunctionSpec {
        func: builtin::not,
        name: "not",
        argument_types: &[BoolT],
    },
    "if" => BuiltinFunctionSpec {
        func: builtin::if_func,
        name: "if",
        argument_types: &[BoolT, AnyT, AnyT],
    },

    // IO
    "print" => BuiltinFunctionSpec {
        func: builtin::print,
        name: "print",
        argument_types: &[ElipsisT(&AnyT)],
    },
    // open
    // read
    // write

    // List
    "list" => BuiltinFunctionSpec {
        func: builtin::list,
        name: "list",
        argument_types: &[ElipsisT(&AnyT)],
    },
    "head" => BuiltinFunctionSpec {
        func: builtin::head,
        name: "head",
        argument_types: &[QexprT(&[AnyT, ElipsisT(&AnyT)])],
    },
    "tail" => BuiltinFunctionSpec {
        func: builtin::tail,
        name: "tail",
        argument_types: &[QexprT(&[AnyT, ElipsisT(&AnyT)])],
    },
    "init" => BuiltinFunctionSpec {
        func: builtin::init,
        name: "init",
        argument_types: &[QexprT(&[AnyT, ElipsisT(&AnyT)])],
    },
    "join" => BuiltinFunctionSpec {
        func: builtin::join,
        name: "join",
        argument_types: &[QexprT(&[ElipsisT(&AnyT)]), ElipsisT(&QexprT(&[ElipsisT(&AnyT)]))],
    },
    "len" => BuiltinFunctionSpec {
        func: builtin::len,
        name: "len",
        argument_types:  &[QexprT(&[ElipsisT(&AnyT)])],
    },
    // nth

    // String
    // slice
    // nth
    // append

    // Char
    // ??

    // Type
    // type-of
    // is-type

    // Functional
    // map
    // fold
    // filter

    // Language
    "eval" => BuiltinFunctionSpec {
        func: builtin::eval,
        name: "eval",
        argument_types: &[AnyT],
    },
    "def" => BuiltinFunctionSpec {
        func: builtin::def,
        name: "def",
        argument_types: &[QexprT(&[SymbolT, ElipsisT(&SymbolT)]), AnyT],
    },
    "set!" => BuiltinFunctionSpec {
        func: builtin::set,
        name: "set!",
        argument_types: &[QexprT(&[SymbolT]), AnyT],
    },
    "lambda" => BuiltinFunctionSpec {
        func: builtin::lambda,
        name: "lambda",
        argument_types: &[QexprT(&[ElipsisT(&SymbolT)]), QexprT(&[ElipsisT(&AnyT)])],
    },

    // begin
    // let
    // for
    // cond
    // continuations?
};