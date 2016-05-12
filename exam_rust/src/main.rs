

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl HasArea for Circle {
    fn area(&self)  -> f64{
        std::f64::consts::PI * (self.r * self.r)
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

fn do_something(x: &Foo) {
    println!("{}", x.method());
}

fn main() {
    println!("trait example.");
    
    let c = Circle{x:0.0f64, y: 0.0f64, r: 1.0f64};
    println!("the area of circle is : {}", c.area());
    
    let s = "林秀全".to_string();
    do_something(&s);
    let u = 8u8;
    do_something(&u);
    
}
