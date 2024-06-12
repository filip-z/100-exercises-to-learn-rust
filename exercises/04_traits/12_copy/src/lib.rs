use std::ops::Add;
// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Debug, Clone, Copy)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}
impl Add<u32> for WrappingU32 {
    type Output = u32;
    fn add(self, rhs: u32) -> u32 {
        self.value + rhs
    }
}
impl Add<WrappingU32> for u32 {
    type Output = u32;
    fn add(self, rhs: WrappingU32) -> u32 {
        self + rhs.value
    }
}

impl Add<WrappingU32> for WrappingU32 {
    type Output = u32;

    fn add(self, rhs: WrappingU32) -> u32 {
        self.value + rhs.value
    }
}

impl PartialEq<WrappingU32> for u32 {
    fn eq(&self, rhs: &WrappingU32) -> bool {
        *self == rhs.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
