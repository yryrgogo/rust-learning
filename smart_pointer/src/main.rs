use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{ Rc, Weak };
use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// 不変参照の構造体について、内部可変参照への変更を許容する値を持たせつつ、複数の所有者も許容する（ Rc<RefCell<>> により実装可能）
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match &self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Deref trait

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // hello("Rust");
    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);
    // hello(&(*m)[..]);

    // // Drop trait

    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");
    // drop(c);
    // println!("CustomSmartPointers dropped before the end of main.");

    // Rc (use enum List)

    // let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating e = {}", Rc::strong_count(&e));
    // let f = Cons(3, Rc::clone(&e));
    // println!("count after creating f = {}", Rc::strong_count(&e));
    // {
    //     let g = Cons(4, Rc::clone(&e));
    //     println!("count after creating g = {}", Rc::strong_count(&e));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&e));

    // NOTE: Rc<RefCell<>> 内部可変参照への変更を許容する構造体を複数の所有者で共有する

    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // NOTE: RefCell<Rc<>> 循環参照

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // // a の次の要素は = {:?}
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // // b 作成後の a の参照カウント = {}
    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // // b の最初の参照カウント = {}
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // // b の次の要素 = {:?}
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // // a を変更後の b の参照カウント = {}
    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // // a を変更後の a の参照カウント = {}
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // // Stack Overflow
    // // a は末尾に b を持ち、b は 末尾に a を持ち、、循環参照
    // println!("a next item = {:?}", a.tail());


    // Weak 木構造により弱い参照を確認する
    // 親ノードが子ノードへの参照を持つことは可能であり、子ノードへの参照を複数の親ノードが持つことも Rc により可能だが、子ノードが自分を参照する親ノードを知るために親ノードへの参照を持つと、循環参照になってしまう
    // また、親ノードがドロップされたら子ノードもドロップされるべきだが、子ノードがドロップされても親ノードはドロップされるべきでない
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

	// leaf の親 = {:?}
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
