fn main() {
    println!("Hello, world!");
    let rectangle1 = Rectangle { width: 30, height: 50 };
    let rectangle2 = Rectangle { width: 40, height: 60 };
    println!("{}, {}, {}",rectangle1.area(), rectangle2.area(), add(rectangle1.area(), rectangle2.area()));
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