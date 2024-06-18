#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, Copy, Clone)]
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

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
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
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //============================================================
    let sq = Rectangle::square(3);
    dbg!(sq);
    //============================================================
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
    dbg!(r);

    //============================================================
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("Area: {}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other_rect);
    dbg!(max_rect);

    //============================================================
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    dbg!("Before", rect);
    rect.set_to_max(other_rect);
    dbg!("After", rect);

    //============================================================
    let mut p = Point { x: 1, y: 2 };
    let y = p.y;
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, y);
}
