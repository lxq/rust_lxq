//! rust base
//! 2016-4-27
//! lxq

fn show(slice:&[u8]) {
    for e in slice {
        print!("{} ", e);
    }
    println!("");
}

// struct 
struct Persion {
    name: String, // 这里不是分号
    age: i32
}

impl Persion {
    fn new(n: &str, a: i32) -> Persion {
        // without "return"
        Persion{
            name:n.to_string(),
            age: a
        }
    }
    
    fn print(&self) {
        println!("My name is {}, I'm {} years old.", self.name, self.age);
    }
}

fn main() {
    println!("Hello, world!");
    
    // char
    let c = '林';
    println!("{}", c);
    
    //array: [T; N]
    let arr : [u8; 4] = [1,2,3,4];
    println!("arr[0]:{}", arr[0]);
    
    //slice: &[T]
    let s1 = &arr[..];
    show(s1);
    let s2 = &arr[1..3];
    show(s2);
    
    //vec
    let mut v1:Vec<i32> = vec![1,2,3,4];
    let v2 = vec![0; 10];
    println!("{}, {}", v1[2], v2[0]);
    for v in &v2 {
        print!("{} ", v);
    }
    println!("");
    // 下面mut的写法很让人费解
    for v in &mut v1 {
        *v += 1;
        print!("{} ", v);
    }
    println!("");
    
    // tuple
    let t1 = (123, "rust");
    let t2: (i32, &str) = (234, "hello world");
    let (x, y) = t2;
    println!("tuple 1: {}, {}", t1.0, t1.1);
    println!("tuple 2: {}, {}", x, y);
    
    // 可以看作是一个有名字的元组，具体使用方法和一般的元组基本类似。
    //struct struct1(i32, String, bool);
    // empty struct 
    //struct struct2;
    
    let lxq = Persion::new("lxq", 37);
    lxq.print();
    
    // 2016.5.3
    let fun: IncType = inc;
    println!("{}", fun(3));
    println!("高阶函数{}", process(5, inc));
    
    let ss = [1,2,3,4,5,6,7];
    let mut ve = Vec::<i32>::new();
    for i in &ss {
        ve.push(get_func(*i)(*i));
    }
    println!("{:?}", ve);
}


type IncType = fn(i32) -> i32;

fn inc(n: i32) -> i32 {
    n+1
}

fn process(n: i32, func: IncType) -> i32 {
    func(n)
}

// fn process2<F>(n: i32, func: F) -> i32
//     where F: fn(i32) -> i32 {
//         func(n)
//     }


// return  function
fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }
    fn dec(n: i32) -> i32 {
        n -1
    }
    
    if 0 == n%2 {
        inc
    } else {
        dec
    }    
}