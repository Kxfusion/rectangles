#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> &str {
        if self.width > other.width && self.height > other.height {
            "can"
        } else {
            "can't"
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle { 
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(5);

    dbg!(&rect1);
    println!("rect1 {} hold rect2", rect1.can_hold(&rect2));
    println!("{} pixels^2", square.area());
}
