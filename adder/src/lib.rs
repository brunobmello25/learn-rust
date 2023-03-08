#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_the_logger() { /* ... */
    }
    #[test]
    fn test_the_database() { /* ... */
    }
    #[test]
    fn test_logger_and_database() { /* ... */
    }
}
