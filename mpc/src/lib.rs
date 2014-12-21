#![allow(dead_code)]

extern crate libc;

use self::libc::c_int;

use std::c_str::CString;
use std::mem;
use std::ptr;
use std::raw::Slice;
use std::str;
use std::string::String;
use std::sync::{StaticMutex, MUTEX_INIT};

mod ext_mpc {

    use super::libc::c_char;
    use super::libc::c_int;
    use super::libc::c_void;
    use super::libc::FILE;
    
    // State Type

    #[repr(C)]
    pub struct MpcState {
      pub pos: c_int,
      pub row: c_int,
      pub col: c_int,
    }

    // Error Type

    #[repr(C)]
    pub struct MpcErr {
      pub state: MpcState,
      pub expected_num: c_int,
      pub filename: *mut c_char,
      pub failure: *mut c_char,
      pub expected: *mut *mut c_char,
      pub recieved: c_char,
    }

    // Parsing Types

    pub type MpcVal = c_void;
    pub type MpcResult = *mut MpcVal;

    #[repr(C)]
    pub struct MpcParser;

    // Function Types

    type MpcDtorFn = extern "C" fn(v: *mut MpcVal);
    type MpcCtorFn = extern "C" fn() -> *mut MpcVal;

    type MpcApplyFn = extern "C" fn(*mut MpcVal) -> *mut MpcVal;
    type MpcApplyToFn = extern "C" fn(*mut MpcVal, *mut c_void) -> *mut MpcVal;
    type MpcFoldFn = extern "C" fn(c_int, *mut MpcVal) -> *mut MpcVal;

    // AST Types

    #[repr(C)]
    pub struct MpcAst {
        pub tag: *mut c_char,
        pub contents: *mut c_char,
        pub state: MpcState,
        pub children_num: c_int,
        pub children: *mut *mut MpcAst,
    }
    
    extern "C" {
        // Error

        pub fn mpc_err_delete(e: *mut MpcErr);
        pub fn mpc_err_string(e: *mut MpcErr) -> *mut c_char;
        pub fn mpc_err_print(e: *mut MpcErr);
        pub fn mpc_err_print_to(e: *mut MpcErr, f: *mut FILE);
        
        // Parsing

        pub fn mpc_parse(filename: *const c_char, string: *const c_char, p: *mut MpcParser, r: *mut MpcResult) -> c_int;
        pub fn mpc_parse_file(filename: *const c_char, file: *mut FILE, p: *mut MpcParser, r: *mut MpcResult) -> c_int;
        pub fn mpc_parse_pipe(filename: *const c_char, pipe: *mut FILE, p: *mut MpcParser, r: *mut MpcResult) -> c_int;
        pub fn mpc_parse_contents(filename: *const c_char, p: *mut MpcParser, r: *mut MpcResult) -> c_int;

        // Building a Parser

        pub fn mpc_new(name: *const c_char) -> *mut MpcParser;
        pub fn mpc_define(p: *mut MpcParser, a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpc_undefine(p: *mut MpcParser) -> *mut MpcParser;

        pub fn mpc_delete(p: *mut MpcParser);
        pub fn mpc_cleanup(n: c_int, ...);
   
        // Basic Parsers

        pub fn mpc_any() -> *mut MpcParser;
        pub fn mpc_char(c: c_char) -> *mut MpcParser;
        pub fn mpc_range(s: c_char, e: c_char) -> *mut MpcParser;
        pub fn mpc_oneof(s: *const c_char) -> *mut MpcParser;
        pub fn mpc_noneof(s: *const c_char) -> *mut MpcParser;
        pub fn mpc_satisfy(f: extern "C" fn(c_char) -> c_int) -> *mut MpcParser;
        pub fn mpc_string(s: *const c_char) -> *mut MpcParser;
    
        // Other Parsers

        pub fn mpc_pass() -> *mut MpcParser;
        pub fn mpc_fail(m: *const c_char) -> *mut MpcParser;
        pub fn mpc_failf(fmt: *const c_char, ...) -> *mut MpcParser;
        pub fn mpc_lift(f: MpcCtorFn) -> *mut MpcParser;
        pub fn mpc_lift_val(x: *mut MpcVal) -> *mut MpcParser;
        pub fn mpc_anchor(f: extern "C" fn(c_char, c_char) -> c_int) -> *mut MpcParser;
        pub fn mpc_state() -> *mut MpcParser;
    
        // Combinator Parsers

        pub fn mpc_expect(a: *mut MpcParser, e: *const c_char) -> *mut MpcParser;
        pub fn mpc_expectf(a: *mut MpcParser, format: *const c_char, ...) -> *mut MpcParser;
        pub fn mpc_apply(a: *mut MpcParser, f: MpcApplyFn) -> *mut MpcParser;
        pub fn mpc_apply_to(a: *mut MpcParser, f: MpcApplyToFn, x: *mut c_void) -> *mut MpcParser;

        pub fn mpc_not(a: *mut MpcParser, da: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_not_lift(a: *mut MpcParser, da: MpcDtorFn, lf: MpcCtorFn) -> *mut MpcParser;
        pub fn mpc_maybe(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpc_maybe_lift(a: *mut MpcParser, lf: MpcCtorFn) -> *mut MpcParser;

        pub fn mpc_many(f: MpcFoldFn, a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpc_many1(f: MpcFoldFn, a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpc_count(n: c_int, f: MpcFoldFn, a: *mut MpcParser, da: MpcDtorFn) -> *mut MpcParser;

        pub fn mpc_or(n: c_int, ...) -> *mut MpcParser;
        pub fn mpc_and(n: c_int, f: MpcFoldFn, ...) -> *mut MpcParser;

        pub fn mpc_predictive(a: *mut MpcParser) -> *mut MpcParser;
   
        // Common Parsers

        pub fn mpc_eoi() -> *mut MpcParser;
        pub fn mpc_soi() -> *mut MpcParser;

        pub fn mpc_boundary() -> *mut MpcParser;

        pub fn mpc_whitespace() -> *mut MpcParser;
        pub fn mpc_whitespaces() -> *mut MpcParser;
        pub fn mpc_blank() -> *mut MpcParser;

        pub fn mpc_newline() -> *mut MpcParser;
        pub fn mpc_tab() -> *mut MpcParser;
        pub fn mpc_escape() -> *mut MpcParser;

        pub fn mpc_digit() -> *mut MpcParser;
        pub fn mpc_hexdigit() -> *mut MpcParser;
        pub fn mpc_octdigit() -> *mut MpcParser;
        pub fn mpc_digits() -> *mut MpcParser;
        pub fn mpc_hexdigits() -> *mut MpcParser;
        pub fn mpc_octdigits() -> *mut MpcParser;

        pub fn mpc_lower() -> *mut MpcParser;
        pub fn mpc_upper() -> *mut MpcParser;
        pub fn mpc_alpha() -> *mut MpcParser;
        pub fn mpc_underscore() -> *mut MpcParser;
        pub fn mpc_alphanum() -> *mut MpcParser;

        pub fn mpc_int() -> *mut MpcParser;
        pub fn mpc_hex() -> *mut MpcParser;
        pub fn mpc_oct() -> *mut MpcParser;
        pub fn mpc_number() -> *mut MpcParser;

        pub fn mpc_real() -> *mut MpcParser;
        pub fn mpc_float() -> *mut MpcParser;

        pub fn mpc_char_lit() -> *mut MpcParser;
        pub fn mpc_string_lit() -> *mut MpcParser;
        pub fn mpc_regex_lit() -> *mut MpcParser;

        pub fn mpc_ident() -> *mut MpcParser;

        // Useful Parsers

        pub fn mpc_startwith(a: MpcParser) -> *mut MpcParser;
        pub fn mpc_endwith(a: MpcParser, da: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_whole(a: MpcParser, da: MpcDtorFn) -> *mut MpcParser;

        pub fn mpc_stripl(a: MpcParser) -> *mut MpcParser;
        pub fn mpc_stripr(a: MpcParser) -> *mut MpcParser;
        pub fn mpc_strip(a: MpcParser) -> *mut MpcParser;
        pub fn mpc_tok(a: MpcParser) -> *mut MpcParser; 
        pub fn mpc_sym(s: *const c_char) -> *mut MpcParser;
        pub fn mpc_total(a: MpcParser, da: MpcDtorFn) -> *mut MpcParser;

        pub fn mpc_between(a: MpcParser, ad: MpcDtorFn, o: *const c_char, c: *const c_char) -> *mut MpcParser;
        pub fn mpc_parens(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_braces(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_brackets(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_squares(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;

        pub fn mpc_tok_between(a: MpcParser, ad: MpcDtorFn, o: *const c_char, c: *const c_char) -> *mut MpcParser;
        pub fn mpc_tok_parens(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_tok_braces(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_tok_brackets(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;
        pub fn mpc_tok_squares(a: MpcParser, ad: MpcDtorFn) -> *mut MpcParser;

        // Common Function Parameters

        pub fn mpcf_dtor_null(x: *mut MpcVal);

        pub fn mpcf_ctor_null() -> *mut MpcVal;
        pub fn mpcf_ctor_str() -> *mut MpcVal;

        pub fn mpcf_free(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_int(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_hex(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_oct(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_float(x: *mut MpcVal) -> *mut MpcVal;

        pub fn mpcf_escape(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_escape_regex(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_escape_string_raw(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_escape_char_raw(x: *mut MpcVal) -> *mut MpcVal;

        pub fn mpcf_unescape(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_unescape_regex(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_unescape_string_raw(x: *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_unescape_char_raw(x: *mut MpcVal) -> *mut MpcVal;

        pub fn mpcf_null(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_fst(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_snd(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_trd(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;

        pub fn mpcf_fst_free(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_snd_free(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_trd_free(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;

        pub fn mpcf_strfold(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_maths(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;

        // Regular Expression Parsers

        pub fn mpc_re(re: *const c_char) -> *mut MpcParser;

        // AST

        pub fn mpc_ast_new(tag: *const c_char, contents: *const c_char) -> *mut MpcAst;
        pub fn mpc_ast_build(n: c_int, tag: *const c_char, ...) -> *mut MpcAst;
        pub fn mpc_ast_add_root(a: *mut MpcAst) -> *mut MpcAst;
        pub fn mpc_ast_add_child(r: *mut MpcAst, a: *mut MpcAst) -> *mut MpcAst;
        pub fn mpc_ast_add_tag(a: *mut MpcAst, t: *const c_char) -> *mut MpcAst;
        pub fn mpc_ast_tag(a: *mut MpcAst, t: *const c_char) -> *mut MpcAst;
        pub fn mpc_ast_state(a: *mut MpcAst, s: MpcState) -> *mut MpcAst;

        pub fn mpc_ast_delete(a: *mut MpcAst);
        pub fn mpc_ast_print(a: *mut MpcAst);
        pub fn mpc_ast_print_to(a: *mut MpcAst, fp: *mut FILE);

        // Warning: This function currently doesn't test for equality of the `state` member!
        pub fn mpc_ast_eq(a: *mut MpcAst, b: *mut MpcAst) -> c_int;

        pub fn mpcf_fold_ast(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;
        pub fn mpcf_str_ast(c: MpcVal) -> *mut MpcVal;
        pub fn mpcf_state_ast(n: c_int, xs: *mut *mut MpcVal) -> *mut MpcVal;

        pub fn mpca_tag(a: *mut MpcParser, t: *const c_char) -> *mut MpcParser;
        pub fn mpca_add_tag(a: *mut MpcParser, t: *const c_char) -> *mut MpcParser;
        pub fn mpca_root(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpca_state(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpca_total(a: *mut MpcParser) -> *mut MpcParser;

        pub fn mpca_not(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpca_maybe(a: *mut MpcParser) -> *mut MpcParser;

        pub fn mpca_many(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpca_many1(a: *mut MpcParser) -> *mut MpcParser;
        pub fn mpca_count(n: c_int, a: *mut MpcParser) -> *mut MpcParser;

        pub fn mpca_or(n: c_int, ...) -> *mut MpcParser;
        pub fn mpca_and(n: c_int, ...) -> *mut MpcParser;

        pub fn mpca_grammar(flags: c_int, grammar: *const c_char, ...) -> *mut MpcParser;

        pub fn mpca_lang(flags: c_int, language: *const c_char, ...) -> *mut MpcErr;
        pub fn mpca_lang_file(flags: c_int, f: *mut FILE, ...) -> *mut MpcErr;
        pub fn mpca_lang_pipe(flags: c_int, f: *mut FILE, ...) -> *mut MpcErr;
        pub fn mpca_lang_contents(flags: c_int, filename: *const c_char, ...) -> *mut MpcErr;

        // Debug & Testing

        pub fn mpc_print(p: *mut MpcParser);

        pub fn mpc_test_pass(p: *mut MpcParser, s: *const c_char, d: *mut c_void,
                             tester: extern "C" fn(*mut c_void, *mut c_void) -> c_int,
                             destructor: MpcDtorFn,
                             printer: extern "C" fn(*mut c_void)) -> c_int;

        pub fn mpc_test_fail(p: *mut MpcParser, s: *const c_char, d: *mut c_void,
                             tester: extern "C" fn(*mut c_void, *mut c_void) -> c_int,
                             destructor: MpcDtorFn,
                             printer: extern "C" fn(*mut c_void)) -> c_int;
    }
}

static MPC_GLOBAL_PARSER_LOCK: StaticMutex = MUTEX_INIT;

unsafe fn from_c_str_with_lifetime<'a>(s: *const i8) -> &'a str {
    let s = s as *const u8;
    let mut len = 0u;
    while *s.offset(len as int) != 0 {
        len += 1u;
    }
    let v: &'a [u8] = mem::transmute(Slice { data: s, len: len });
    str::from_utf8(v).expect("from_c_str_with_lifetime passed invalid utf-8 data")
}

pub struct Error {
    error: *mut ext_mpc::MpcErr,
}

impl Error {
    #[allow(unused_variables)]
    pub fn to_string(&self) -> String {
        unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            let s = ext_mpc::mpc_err_string(self.error);
            String::from_utf8_lossy(CString::new(s as *const i8, true).as_bytes()).into_owned()
        }
    }

    #[allow(unused_variables)]
    pub fn print(&self) {
        unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            ext_mpc::mpc_err_print(self.error);
        }
    }
}

impl Drop for Error {
    #[allow(unused_variables)]
    fn drop(&mut self) {
        unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            ext_mpc::mpc_err_delete(self.error);
        }
    }
}

pub struct Ast {
    ast: *mut ext_mpc::MpcAst,
    owning: bool,
}

impl Ast {
    #[allow(unused_variables)]
    pub fn print(&self) {
        unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            ext_mpc::mpc_ast_print(self.ast);
        }
    }

    pub fn get_tag<'a>(&'a self) -> &'a str {
        unsafe {
            let a = self.ast.as_ref().expect("Internal error");
            if a.tag.is_null() { panic!("Internal error"); }
            from_c_str_with_lifetime(a.tag as *const i8)
        }
    }

    pub fn get_contents<'a>(&'a self) -> &'a str {
        unsafe {
            let a = self.ast.as_ref().expect("Internal error");
            if a.contents.is_null() { panic!("Internal error"); }
            from_c_str_with_lifetime(a.contents as *const i8)
        }
    }

    pub fn get_no_children(&self) -> int {
        unsafe {
            let a = self.ast.as_ref().expect("Internal error");
            a.children_num as int
        }
    }

    pub fn get_child(&self, index: int) -> Option<Ast> {
        let a = unsafe { match self.ast.as_ref() { Some(a) => a, None => return None } };

        if index >= a.children_num as int {
            return None
        }

         Some(Ast { ast: unsafe { *a.children.offset(index) }, owning: false })
    }

    pub fn child_iter(&self) -> AstChildIterator {
        AstChildIterator { ast: self, index: 0 }
    }
}

impl Drop for Ast {
    #[allow(unused_variables)]
    fn drop(&mut self) {
        if self.owning {
            unsafe {
                let guard = MPC_GLOBAL_PARSER_LOCK.lock();
                ext_mpc::mpc_ast_delete(self.ast);
            }
        }
    }
}

pub struct AstChildIterator<'a> {
    ast: &'a Ast,
    index: int,
}

impl<'a> Iterator<Ast> for AstChildIterator<'a> {
    fn next(&mut self) -> Option<Ast> {
        self.index += 1;
        self.ast.get_child(self.index-1)
    }
}

pub enum Result {
    Ast(Ast),
    Error(Error),
}

#[allow(missing_copy_implementations)]
pub struct Parser {
    parser: *mut ext_mpc::MpcParser,
}

impl Parser {
    #[allow(unused_variables)]
    pub fn new(name: &str) -> Parser {
        let c_name = name.to_c_str();
        unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            let ret = ext_mpc::mpc_new(c_name.as_ptr());
            if ret.is_null() {
                panic!("Failed to create parser, {}", name);
            } else {
                Parser{ parser: ret }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn parse(&self, input: &str) -> Option<Result> {

        let mut res: ext_mpc::MpcResult = ptr::null_mut();

        let ret_code = unsafe {
            let guard = MPC_GLOBAL_PARSER_LOCK.lock();
            ext_mpc::mpc_parse("stdin>".to_c_str().as_ptr(), input.to_c_str().as_ptr(), self.parser, &mut res)
        };
        
        if res == ptr::null_mut() {
            return None;
        }

        if ret_code == 0 {
            return Some(Result::Error(Error { error: unsafe { mem::transmute(res) } }));
        } else if ret_code == 1 {
            return Some(Result::Ast(Ast { ast: unsafe { mem::transmute(res) }, owning: true }));
        }

        None
    }
}

bitflags! {
    flags LangFlags: c_int {
        const DEFAULT               = 0,
        const PREDICTIVE            = 1,
        const WHITE_SPACE_SENSITIVE = 2,
    }
}

#[allow(unused_variables)]
pub fn lang(flags: LangFlags, language: &str, parsers: &[&mut Parser]) -> Option<Error> {
    let c_language = language.to_c_str();

    let err: *mut ext_mpc::MpcErr = unsafe {
        let guard = MPC_GLOBAL_PARSER_LOCK.lock();
        match parsers.len() {
            1  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser),
            2  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser),
            3  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser),
            4  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser),
            5  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser),
            6  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser),
            7  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser),
            8  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser),
            9  => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser),
            10 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser),
            11 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser),
            12 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser),
            13 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser),
            14 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser),
            15 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser),
            16 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser),
            17 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser, parsers[16].parser),
            18 => ext_mpc::mpca_lang(flags.bits(), c_language.as_ptr(), parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser, parsers[16].parser, parsers[17].parser),
            _  => panic!("You must provide between 1 and 18 parsers"),
        }
    };

    if err.is_null() {
        return None;
    }

    Some(Error { error: err })
}

#[allow(unused_variables)]
pub fn cleanup(parsers: &[&mut Parser]) {
    unsafe {
        let guard = MPC_GLOBAL_PARSER_LOCK.lock();
        match parsers.len() {
            1  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser),
            2  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser),
            3  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser),
            4  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser),
            5  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser),
            6  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser),
            7  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser),
            8  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser),
            9  => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser),
            10 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser),
            11 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser),
            12 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser),
            13 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser),
            14 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser),
            15 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser),
            16 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser),
            17 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser, parsers[16].parser),
            18 => ext_mpc::mpc_cleanup(parsers.len() as i32, parsers[0].parser, parsers[1].parser, parsers[2].parser, parsers[3].parser, parsers[4].parser, parsers[5].parser, parsers[6].parser, parsers[7].parser, parsers[8].parser, parsers[9].parser, parsers[10].parser, parsers[11].parser, parsers[12].parser, parsers[13].parser, parsers[14].parser, parsers[15].parser, parsers[16].parser, parsers[17].parser),
            _  => panic!("You must provide between 1 and 18 parsers"),
        }
    }
}