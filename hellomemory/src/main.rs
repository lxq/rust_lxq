
//! my example for Rust memory operation.
//! lxq@weetgo.com
//! 2016/12/07

use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::sync::RwLock;

const N: usize = 10;

/// my simple test for Arc
fn test_arc() {
    let nums: Vec<_> = (0..100u32).collect();
    let shared_nums = Arc::new(nums);

    for _ in 0..10 {
        // TODO: 通过clone()来增加引用计数，减少引用计数使用drop().
        let children = shared_nums.clone();

        thread::spawn(move || {
            let locals = &children[..];
            // work with locals
            println!("in thread: {:?}", locals);
        });
    }
}

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn test_rc() {
    let g_owner: Rc<Owner> = Rc::new(Owner{name: String::from("Owner People")});

    let g1 = Gadget{id: 1, owner: g_owner.clone()};
    let g2 = Gadget{id: 2, owner: g_owner.clone()};

    // TODO: 这个起什么作用？
    drop(g_owner);

    println!("gadget {} owned by {}.", g1.id, g1.owner.name);
    println!("gadget {} owner by {}.", g2.id, g2.owner.name);
}

fn test_mutex() {
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            println!("data:{}", *data);
            if N == *data {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
}

fn test_rwlock() {
    let lock = RwLock::new(5);

    // read
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1 = {}, r2 = {}", *r1, *r2);
    }

    // write
    {
        let mut w = lock.write().unwrap();
        *w += 12;
        println!("write: {}", *w);
    }
}

fn main() {
    println!("Hello, Rust memory!");

    test_arc();

    test_rc();

    test_mutex();

    test_rwlock();
}
