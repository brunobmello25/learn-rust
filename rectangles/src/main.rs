struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let result = 1 + 4;

    match result {
        1 => println!("deu 1"),
        2 => println!("deu 2"),
        banana => println!("deu banana"),
        rest => println!("Poxa, deu {}", rest),
    }

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );

    println!("Rect 2 can hold rect 1: {}", rect2.can_hold(&rect1));
}
