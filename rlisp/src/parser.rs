use cell;
use mpc;

pub struct Parser {
    float:   mpc::Parser,
    integer: mpc::Parser,
    string:  mpc::Parser,
    char:    mpc::Parser,
    bool:    mpc::Parser,
    symbol:  mpc::Parser,
    expr:    mpc::Parser,
    sexpr:   mpc::Parser,
    qexpr:   mpc::Parser,
    rlisp:   mpc::Parser,
}

impl Parser {
    pub fn new() -> Parser {
        let mut parser = Parser {
            float:   mpc::Parser::new("float"),
            integer: mpc::Parser::new("integer"),
            string:  mpc::Parser::new("string"),
            char:    mpc::Parser::new("char"),
            bool:    mpc::Parser::new("bool"),
            symbol:  mpc::Parser::new("symbol"),
            expr:    mpc::Parser::new("expr"),
            sexpr:   mpc::Parser::new("sexpr"),
            qexpr:   mpc::Parser::new("qexpr"),
            rlisp:   mpc::Parser::new("rlisp"),
        };

        if let Some(e) = mpc::lang(mpc::DEFAULT,
                "float   : /[-+]?[0-9]*\\.?[0-9]+([eE][-+]?[0-9]+)?/;\
                 integer : /[-+]?[0-9]+/;\
                 string  : /\"[^\"]*\"/;\
                 char    : /\'[^\']\'/;\
                 bool    : /#t/ | /#f/;\
                 symbol  : /[a-zA-Z!$%&\\*\\+\\-\\.\\/:<=>\\?@^_~\\\\][0-9a-zA-Z!$%&\\*\\+\\-\\.\\/:<=>\\?@^_~\\\\]*/;\
                 expr    : <float> | <integer> | <string> | <char> | <bool> | <symbol> | <sexpr> | <qexpr>;\
                 sexpr   : '(' <expr>* ')';\
                 qexpr   : '{' <expr>* '}';\
                 rlisp   : /^/ <expr>* /$/;",
                &[&mut parser.float,
                  &mut parser.integer,
                  &mut parser.string,
                  &mut parser.char,
                  &mut parser.bool,
                  &mut parser.symbol,
                  &mut parser.expr,
                  &mut parser.sexpr,
                  &mut parser.qexpr,
                  &mut parser.rlisp]) {

            let error = e.to_string();
            panic!("{}", error[]);
        }

        parser
    }

    pub fn parse(&self, input: &str) -> cell::Cell {
        let ast = match self.rlisp.parse(input) {
            Some(mpc::Result::Ast(a))   => a,
            Some(mpc::Result::Error(e)) => { return cell::Cell::Error(e.to_string()); },
            None                        => { panic!("Internal parsing error") },
        };
        parse_ast(&ast)
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
          mpc::cleanup(&[&mut self.float,
                         &mut self.integer,
                         &mut self.string,
                         &mut self.char,
                         &mut self.bool,
                         &mut self.symbol,
                         &mut self.expr,
                         &mut self.sexpr,
                         &mut self.qexpr,
                         &mut self.rlisp]);
    }
}

fn parse_ast(ast: &mpc::Ast) -> cell::Cell {

    let tag: String = ast.get_tag();

    if tag[].find_str("float").is_some() {
        return match from_str(ast.get_contents()[].trim()) {
            Some(f) => cell::Cell::Float(f), None => cell::Cell::Float(0.0)
        };
    }

    if tag[].find_str("integer").is_some() {
        return match from_str(ast.get_contents()[].trim()) {
            Some(i) => cell::Cell::Integer(i), None => cell::Cell::Integer(0)
        };
    }

    if tag[].find_str("string").is_some() {
        let s = ast.get_contents();
        return cell::Cell::Str(s[].slice(1, s.len() - 1).to_string());
    }

    if tag[].find_str("char").is_some() {
        let s = ast.get_contents();
        return cell::Cell::Char(s[].slice(1, s.len() - 1).char_at(0));
    }

    if tag[].find_str("bool").is_some() {
        let s = ast.get_contents();

        if s[] == "#t" {
            return cell::Cell::Bool(true);
        } else {
            return cell::Cell::Bool(false);
        }
    }

    if tag[].find_str("symbol").is_some() {
        let s = ast.get_contents();
        return cell::Cell::Symbol(s);
    }

    if tag[].find_str("qexpr").is_some() {    
        let mut res: Vec<cell::Cell> = Vec::new();

        for c in ast.child_iter() {

            let ctag: String = c.get_tag();
            let ccontent: String = c.get_contents();

            if ctag[] == "regex" ||
               ccontent[] == "{" ||
               ccontent[] == "}" {
                continue;
            }

            res.push(parse_ast(&c));
       }

       return cell::Cell::Qexpr(res);
    }

    if tag[].find_str("sexpr").is_some() {
        let mut res: Vec<cell::Cell> = Vec::new();

        for c in ast.child_iter() {

            let ctag: String = c.get_tag();
            let ccontent: String = c.get_contents();

            if ctag[] == "regex" ||
               ccontent[] == "(" ||
               ccontent[] == ")" {
                continue;
            }

           res.push(parse_ast(&c));
        }

       return cell::Cell::Sexpr(res);
    }

    if tag[] == ">" {
        return parse_ast(&ast.get_child(1).expect("Internal grammer error"));
    }

    panic!("Internal ast parse error");
}