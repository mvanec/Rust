use macros::*;

pub fn add(left: usize, right: usize) -> usize {
    println!("\n=========Running {}", function!());
    left + right
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("\n=========Running {}", function!());
        // code to actually draw a button
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("\n=========Running {}", function!());
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
