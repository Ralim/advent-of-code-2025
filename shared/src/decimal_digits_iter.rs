/// Decimal digits iterator, yields out the digits of a number in left to right order
/// i.e. 123 -> 1,2,3
#[derive(Debug, Clone)]
pub struct DecimalDigits {
    number: u64,
    pos: u32,
}
impl From<u64> for DecimalDigits {
    fn from(number: u64) -> Self {
        DecimalDigits {
            number,
            pos: number.ilog10() + 1,
        }
    }
}
impl From<&str> for DecimalDigits {
    fn from(number: &str) -> Self {
        let num = number.parse().unwrap();
        DecimalDigits {
            number: num,
            pos: num.ilog10() + 1,
        }
    }
}

impl DecimalDigits {
    /// Returns the number of remaining digits
    pub fn len(&self) -> usize {
        self.pos as usize
    }
}
impl Iterator for DecimalDigits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            None
        } else {
            self.pos -= 1;
            let div = 10_u64.pow(self.pos);
            Some((self.number / div) % 10)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        let digits = DecimalDigits::from(123u64);
        assert_eq!(digits.number, 123);
        assert_eq!(digits.pos, 3);
    }

    #[test]
    fn test_from_str() {
        let digits = DecimalDigits::from("456");
        assert_eq!(digits.number, 456);
        assert_eq!(digits.pos, 3);
    }

    #[test]
    fn test_len() {
        let digits = DecimalDigits::from(12345u64);
        assert_eq!(digits.len(), 5);

        let single_digit = DecimalDigits::from(7u64);
        assert_eq!(single_digit.len(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut digits = DecimalDigits::from(123u64);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_iterator_single_digit() {
        let mut digits = DecimalDigits::from(5u64);
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_iterator_collect() {
        let digits = DecimalDigits::from(9876u64);
        let collected: Vec<u64> = digits.collect();
        assert_eq!(collected, vec![9, 8, 7, 6]);
    }

    #[test]
    fn test_clone() {
        let original = DecimalDigits::from(123u64);
        let cloned = original.clone();
        assert_eq!(original.number, cloned.number);
        assert_eq!(original.pos, cloned.pos);
    }
}
