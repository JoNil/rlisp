use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

use cell::Cell;
use globals;

struct EnvironmentImpl {
    table:     HashMap<String, Cell>,
    enclosing: Option<Weak<RefCell<EnvironmentImpl>>>,
}

#[derive(Clone)]
pub struct Environment(Rc<RefCell<EnvironmentImpl>>);

impl Environment {
    pub fn new() -> Environment {
        Environment(Rc::new(RefCell::new(EnvironmentImpl {
            table:     HashMap::new(),
            enclosing: None,
        })))
    }

    pub fn make_sub_environment(&self) -> Environment {
        let &Environment(ref env) = self;
        Environment(Rc::new(RefCell::new(EnvironmentImpl {
            table:     HashMap::new(),
            enclosing: Some(env.downgrade()),
        })))
    }

    pub fn lookup(&self, key: &String) -> Cell {
        let &Environment(ref env) = self;
        match (env.borrow().table.get(key), &env.borrow().enclosing) {
            (Some(c), _)         => c.clone(),
            (None, &Some(ref e)) => Environment(e.upgrade().expect("Internal error")).lookup(key),
            (None, &None)        => {
                match globals::GLOBAL_ENVIROMENT.get(key[]) {
                    Some(bfs) => Cell::Builtin(bfs),
                    None      => Cell::Error(format!("Undefined symbol: {}", key))
                }
            }
        }
    }

    pub fn insert(&self, key: &String, c: &Cell) {
        let &Environment(ref env) = self;
        env.borrow_mut().table.insert(key.clone(), c.clone());
    }

    pub fn insert_top(&self, key: &String, c: &Cell) {
        let &Environment(ref env) = self;
        let enclosing = env.borrow().enclosing.clone();
        match enclosing {
            Some(ref e) => Environment(e.upgrade().expect("Internal error")).insert_top(key, c),
            None        => { env.borrow_mut().table.insert(key.clone(), c.clone()); },
        };
    }
}