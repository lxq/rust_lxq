
//! my exame for trait.
//! lxq@weetgo.com
//! 2016/11/20

use std::io::Write;
use std::time;
use std::collections::HashMap;
use std::num::ParseIntError;

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", *self)
    }
}

// x 是一个trait对象
// 用指针的目的是因为，x可以是任意实现Foo的类型实例，内存大小并不确定，但指针的大小是固定的。
fn do_something(x: &Foo) {
    println!("{}",x.method());
}

fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
    some_closure(1)
}

fn call_with_two<F>(closure: &F) -> i32
    where F: Fn(i32) -> i32{
    closure(2)
}

fn push_1m(v: &mut Vec<usize>, num: usize) {
    let st = time::SystemTime::now();
    for i in 1..num {
        v.push(i);
    }
    let et = time::SystemTime::now();
    println!("花费时间:{:?}", et.duration_since(st).unwrap());
}

fn test_iterator () {
    let v = vec![1,2,3,4,5];
    
    let v_take = v.iter()
        .cloned()
        .take(3)
        .collect::<Vec<_>>();
    assert_eq!(v_take, vec![1,2,3]);

    let v_skip = v.iter()
        .cloned()
        .skip(3)
        .collect::<Vec<_>>();
    assert_eq!(v_skip, vec![4,5]);

    let names = vec!["lxq", "mike", "tom"];
    let scores = vec![60, 80, 100];
    let zip_map: HashMap<_,_> = names.iter()
        .zip(scores.iter())
        .collect();
    println!("zip map: {:?}", zip_map);
}

fn find_char(ss: &str, cc: char) -> Option<usize> {
    for (i, c) in ss.char_indices() {
        if (cc == c) {
            return Some(i)
        }
    }
    None
}
fn test_handle_error() {
    let file_name = "hello.rs";
    match find_char(&file_name, '.') {
        None => println!("没有文件推展名."),
        Some(i) => println!("扩展名：{}", &file_name[(i+1)..]),
    }
}

fn double_num(num_str: &str) -> Result<i32, ParseIntError> {
    num_str.parse::<i32>().map(|x| x * 2)
}

macro_rules! create_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("调用函数：{:?}", stringify!($fn_name))
        }
    }
}

macro_rules! vector {
    ($($x: expr),*) => {
        {
            let mut tmpv = Vec::new();
            $(tmpv.push($x);)*
            tmpv
        }
    };
}

fn main() {
    let c = Circle {x: 0.0, y: 0.0, r: 1.0};
    println!("Circle c has area: {}", c.area());

    {
        let curPath = std::env::current_dir().unwrap();
        println!("current path: {:?}", curPath);
        let mut f = std::fs::File::open("foo.txt").expect("无法打开文件 foo.txt. ");
        let buf = b"WeetGo.com";
        let res = f.write(buf);
    }

    {
        let x = 8u8;
        do_something(&x);
        let s = "WEETGO.COM".to_string();
        do_something(&s);
    }

    // closure test
    {
        // 静态分发——没理解
        let r = call_with_one(|x| x + 1);
        println!("answer is : {}", r);

        // 动态分发——没理解
        let r = call_with_two(&|x| x+ 2);
        println!("result is {}", r);
    }

    // 测试Vec预分配时的效率
    {
        let num: usize = 5_000_000;
        let mut v: Vec<usize> = vec![];
        push_1m(&mut v, num);
        
        let mut v: Vec<usize> = vec![];
        v.reserve(num);
        push_1m(&mut v, num);
    }

    // iterator
    test_iterator();

    test_handle_error();

    // test Result
    {
        match double_num("123") {
            Ok(n) => println!("result is {}", n),
            Err(err) => println!("error is {:?}", err),
        }
    }

    // test macro
    {
        create_fn!(foobar);
        foobar();

        let v = vector![1,2,3,4,5,57];
        println!("my vector: {:?}", v);
    }
}
