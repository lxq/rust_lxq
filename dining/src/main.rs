
// 哲学家就餐问题
// lxq@weetgo.com
// 2016-10-28


struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    // 关联函数(类似于其他语言的静态方法)，用于创建对象
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {name: name.to_string(), left: left, right:right}
    }

    // 方法(类似其他的对象方法)，使用内置参数 &selfB
    fn eat(&self, table: &Table) {
        // _ 开始的命名为了明确告诉Rust是要忽略的
        let _left = table.forks[self.left].lock().unwrap();
        // 模拟拿起叉子的动作时间
        std::time::Duration::from_millis(150);
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} 正在吃。", self.name);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("{} 吃完了。", self.name);
        
    }
}

// table 对象用于实现互斥
struct Table {
    forks: Vec<std::sync::Mutex<()>>,
}

fn main() {
    println!("哲学家就餐问题的案例");

    //定义原子操作的互斥对象
    let table = std::sync::Arc::new(Table { forks:vec![
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
        std::sync::Mutex::new(()),
    ]});
    // 5个哲学家
    let ps = vec![
        Philosopher::new("p1", 0, 1),
        Philosopher::new("p2", 1, 2),
        Philosopher::new("p3", 2, 3),
        Philosopher::new("p4", 3, 4),
        // 模拟左撇子
        Philosopher::new("p5", 0, 4),
    ];

    // 并行实现方式
    /**
    * 下面一句涉及内容很多，有：
    * 1. Vec<_> 是种什么类型，什么情况使用
    * 2. 关于容器的操作：into_iter(), map(), collect()
    * 3. 闭包，这里出现了2重闭包函数
    * 4. std::thread模块，需要深入函数线程、进程的相关知识，如 spawn(), join()等
    */
    let handles: Vec<_> = ps.into_iter().map(|p| {
        let table = table.clone();
        std::thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
