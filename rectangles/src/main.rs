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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/* fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} */

fn main() {
    let mut scale = 2;

    let rect1 = Rectangle::square(dbg!(30 * scale));

    dbg!(&rect1);

    println!("Rectangle is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    scale = 1;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 * scale),
    };

    dbg!(&rect2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    scale = 3;

    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 * scale),
    };

    dbg!(&rect3);

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /* let rect_dimensions = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_dimensions(rect_dimensions)
    ); */
}
