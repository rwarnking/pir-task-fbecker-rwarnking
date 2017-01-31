use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct RecDrop {
    child: RefCell<Option<Rc<RecDrop>>>
}

impl std::ops::Drop for RecDrop {
    fn drop(&mut self) {
        println!("I was dropped!");
    }
}

/// Creates an RC cycle
fn main() {
    let one = Rc::new(RecDrop { child: RefCell::new(None) });
    let two = one.clone();
    *one.child.borrow_mut() = Some(two);
}
