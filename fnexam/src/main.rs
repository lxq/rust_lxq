/// 函数练习.
/// <<Rust Primer>>
/// lxq@weetgo.com
/// 2016/11/14


fn say_hi(name: &str) {
    println!("Hi, {}.", name);
}

fn say_hello(name: &str) {
    println!("Hello, {}.", name);
}

fn say_what(name: &str, func: fn(&str)) {
    func(name);
}

// 函数参数的模式匹配
fn print_person((name, age):(&str, i32)) {
    println!("My name is {}, I'm {} years old.", name, age);
}
fn print_name((name, _):(&str, i32)) {
    println!("My name is {}.", name);
}
fn print_age((_, age):(&str, i32)) {
    println!("My age is {}.", age);
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}
fn test_match_person() {
    let name = "WEETGO".to_string();
    let x: Option<Person> = Some(Person{name: Some(name)});
    match x {
        Some(Person{name: ref a @ Some(_), ..}) => println!("{:?}", a),
        _ => {},
    }
}

fn main() {
    let xm = "小明";
    let xh = "小红";

    say_what(xm, say_hi);
    say_what(xh, say_hello);

    let zs = ("张三", 53);
    print_person(zs);
    print_age(zs);
    print_name(zs);

    let mut x = 5;
    match x {
        ref mut mr => println!("mut ref : {}", mr),
    };
    let y = false;
    match x {
        // match 后置条件
        3 | 5 if y => println!("yes"),
        _ => println!("No"),
    }

    // match 变量绑定
    let y = 3;
    match y {
        // ... 用于匹配范围， | 用于匹配多个条件
        e @ 1...5 | e@ 10...20 => println!("get: {}", e),
        _ => (),
    };

    test_match_person();
}
