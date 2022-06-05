use std::{cell::RefCell, rc::Rc};
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

pub enum ListRefCellRc {
    Cons(i32, RefCell<Rc<ListRefCellRc>>),
    Nil,
}
