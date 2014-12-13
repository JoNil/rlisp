use super::Rlisp;

pub fn inject(rlisp: &mut Rlisp) {
    rlisp.execute("(def {unpack f l} {eval (join (list f) l)})");
}