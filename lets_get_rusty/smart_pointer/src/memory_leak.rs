use self::List::{Cons, Nil};
use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn run() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}\n", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());
    println!("b item = {:?}", b);
    println!("b next item = {:?}", b.tail().unwrap());
    println!(
        "b next and next item = {:?}\n",
        b.tail().unwrap().borrow().tail().unwrap()
    );

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    /*
     * cause memory leak
     */
    // println!("\nb next item = {:?}\n", b.tail());
}

/*
 * Tree
 */

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn tree_run() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "count of leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("leaf: {}", leaf.value);
    println!("branch: {}", branch.value);

    println!("branch to leaf : {}", branch.children.borrow()[0].value);

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("after connect branch to leaf");

    println!("leaf: {:?}", leaf);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("\nTree Scenario 2\n");

    let leaf_01 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "count of leaf_01 strong = {}, weak = {}",
        Rc::strong_count(&leaf_01),
        Rc::weak_count(&leaf_01)
    );

    {
        let branch_01 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf_01)]),
        });

        println!(
            "after clonning count of leaf_01 strong = {}, weak = {}",
            Rc::strong_count(&leaf_01),
            Rc::weak_count(&leaf_01)
        );

        println!(
            "before connecting count of branch_01 strong = {}, weak = {}",
            Rc::strong_count(&branch_01),
            Rc::weak_count(&branch_01)
        );

        *leaf_01.parent.borrow_mut() = Rc::downgrade(&branch_01);

        println!(
            "after connecting count of branch_01 strong = {}, weak = {}",
            Rc::strong_count(&branch_01),
            Rc::weak_count(&branch_01)
        );

        println!(
            "after connecting branch_01 count of leaf_01 strong = {}, weak = {}",
            Rc::strong_count(&leaf_01),
            Rc::weak_count(&leaf_01)
        );
        println!(
            "leaf_01 parent = {:?}",
            leaf_01.parent.borrow().upgrade().unwrap()
        );
    }

    println!(
        "after block of branch_01 count of leaf_01 strong = {}, weak = {}",
        Rc::strong_count(&leaf_01),
        Rc::weak_count(&leaf_01)
    );

    println!("leaf_01 parent = {:?}", leaf_01.parent.borrow().upgrade());
}
