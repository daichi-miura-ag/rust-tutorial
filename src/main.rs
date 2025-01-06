fn main() {
    println!("Hello, world!");

    let rectangle1 = Rectangle { width: 30, height: 50 };
    let rectangle2 = Rectangle { width: 40, height: 60 };
    println!("{}, {}, {}",rectangle1.area(), rectangle2.area(), add(rectangle1.area(), rectangle2.area()));

    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("{:?}", numbers);
    for num in numbers.iter() {
        println!("{}", num);
    }

    let add_closure = |x: i32, y: i32| x + y;
    println!("{}", add_closure(5, 7));

    let boxed_n = Box::new(5);
    println!("{}", boxed_n);
}

// structure; 構造体
struct Rectangle {
    width: i32,
    height: i32,
}

// implementation; 実装
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

// function; 関数
fn add(x: i32, y: i32) -> i32 {
    x + y
}