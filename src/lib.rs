pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn subtract(a: u32, b: u32) -> u32 {
    a - b
}

#[cfg(test)]
mod tests {
    use crate::{add, subtract};

    #[test]
    fn test_add() {
        let c = add(4, 4);
        assert_eq!(c, 8);
    }

    #[test]
    fn test_subtract() {
        let c = subtract(10, 2);
        assert_eq!(c, 8);
    }
}
