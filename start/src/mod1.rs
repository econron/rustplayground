mod mod2;

use std::rc::Rc;

pub struct Node {
    pub data: i32,
    pub child: Option<Rc<Node>>
}

pub fn print_link(start_node: Rc<Node>){
    let mut p = start_node;
    loop {
        println!("{}", p.data);
        if p.child.is_none() {
            break;
        }
        p = Rc::clone(p.child.as_ref().unwrap());
    }
}

pub fn print_static_name(){
    let node2 = Rc::new(mod2::Node2 {
        name: "TESTMAN".to_string()
    });
    println!("the name is {}", &node2.name)
}

