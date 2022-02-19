use std::rc::Rc;
mod mod1;

struct TestA {
    data_i32: i32,
    data_string: String,
}

struct Node {
    data: i32,
    child: Option<Rc<Node>>
}

fn main() {
    let x = Rc::new(TestA {
        data_i32: 1,
        data_string: String::from("Hello"),
    });

    // Rc型の中の値はイミュータブルであり、所有権も移動できない
    // 下記2行は所有権が移動するのでコンパイルエラー
    // let data_i32 = x.data_i32;
    // let data_string = x.data_string;

    // リファレンスを利用して所有権の移動が起こらないようにする
    // 考えたことメモ：初期化したら関数に当てはめて借用を解除するのが良い・・？
    let data_i32 = &x.data_i32;
    let data_string = &x.data_string;

    println!("{}, {}", data_i32, data_string);

    // mod1Cellの学習用
    let node3 = Rc::new(mod1::Node {
        data: 1,
        child: None
    });
    let node2 = Rc::new(mod1::Node {
        data: 2,
        child: Some(Rc::clone(&node3))
    });
    let node1 = Rc::new(mod1::Node {
        data: 3,
        child: Some(Rc::clone(&node3))
    });

    println!("node1");
    mod1::print_link(Rc::clone(&node1));

    println!("node2");
    mod1::print_link(Rc::clone(&node2));

    mod1::print_static_name();
}
