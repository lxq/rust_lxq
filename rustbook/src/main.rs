// rust book 学习的练习项目
// rust book :https://kaisery.gitbooks.io/rust-book-chinese/content/
// 2016.10.25
// lxq@weetgo.com


// struct 的生命周期
struct Foo<'a> {
    x: &'a i32,
}
// lifetime in impl
impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {
        self.x
    }
}

// test borrowing
fn test_borrowing () {
    let mut x: Vec<i32> = vec!(1i32, 2, 3);
    println!("x: {:?}", x);
    x.push(123);
    println!("x: {:?}", x);

    {
        let mut y = &mut x;
        y.push(234);
        println!("y: {:?}", y);

        // not &mut x
        let mut z = &mut y;
        z.push(456);
        println!("z:{:?}", z);
    }

    println!("x: {:?}", x);

    // borrowing 2
    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a + b);
    }
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        s1 + s2
    }
    let v1 = vec![1,2,3];
    let v2 = vec![10,11, 12];
    println!("sum : {}", foo(&v1, &v2));
}


fn main() {
    println!("Hello, Rust Book!");

    // enumerate()
    for (i, j) in (5..10).enumerate() {
        println!("索引{}的值{}", i, j);
    }
    {
        let lines = "你好\nRust".lines();
        for (num, line) in lines.enumerate() {
            println! ("行{}:{}", num, line);
        }
    }

    // loop labels
    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            if 0 == x%2 {
                continue 'outer;
            }
            if 0 == y %2 {
                continue 'inner;
            }
            println!("x:{}, y:{}", x, y);
        }
    }

    // scope
    {
        let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
        // 在下面使用x的引用前必须不能与上面&mut 的y引用在同一个作用域。
        println!("x: {}",x );
    }

    // lifetime
    {
        let y = &5; // let _y = 5; let y = &_y;
        let f = Foo{x: y};
        println!("x in struct: {}, f.x(): {}", f.x, f.x()); 
    }

    test_borrowing();
}
