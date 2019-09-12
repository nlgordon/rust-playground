
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn debug_print(input: &String) -> &str {
    println!("Debug: {}", input);

    &input[0..1]
}

#[derive(Debug)]
pub struct Rectangle<Size> {
    width: Size,
    height: Size,
}

impl Rectangle<u32> {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn new(height: u32, width: u32) -> Rectangle<u32> {
        Rectangle {width, height}
    }

    pub fn square(size: u32) -> Rectangle<u32> {
        Rectangle { width: size, height: size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn area_of_a_1_by_1_rectangle_is_1() {
        let subject = Rectangle::new(1, 1);
        assert_eq!(subject.area(), 1);
    }
}
