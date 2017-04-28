
// 猜字游戏练习
// rust book
// 216.10.27

use std::cmp;

//相当于 use rand;
extern crate rand;
use rand::Rng;

fn main() {
    println!("猜字游戏!");

    //产生随机数
    let num = rand::thread_rng().gen_range(1, 101);
    //println!("随机数是：{}", num);

    loop {
        println!("请输入你猜的数字：");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .expect("读取输入失败！");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => { println!("输入的不是数字！"); continue; }
        };
        
        println!("你猜的数字是：{}", guess);

        //compare numbers
        match guess.cmp(&num) {
            cmp::Ordering::Greater  => 
                println!("大了！"),
            cmp::Ordering::Less     => 
                println!("小了！"),
            cmp::Ordering::Equal    => 
                {
                    println!("恭喜你，猜对了！");
                    break;
                }
        }
    }
}
