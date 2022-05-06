use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //
    // move によるスレッド間の所有権の移動
    //
    let v = vec![1, 2, 3];

    // NOTE: move がないとエラーになる。
    // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    //
    // Channel
    //
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // tx.send に所有権が移っていることの確認
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //
    // Mutex
    //
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // NOTE: ここで drop しないと num で m が lock されたままとなり n が編集権限を取得できず、ブロックされたままプログラムが止まる
        drop(num);
        let mut n = m.lock().unwrap();
        *n = 12;
    }
    println!("m = {:?}", m);

    //
    // Parallel
    //

    // Arc は Rc のマルチスレッド版
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    println!("count: {:?}", counter);
}
