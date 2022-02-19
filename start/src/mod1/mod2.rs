use std::rc::Rc;

pub struct Node2 {
    pub name: String
}

pub fn getName(node: Rc<Node2>) {
    println!("name is {}.", node.name);
}