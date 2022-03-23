// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter(|item| item.0 % 2 == 0).map(|x| x.1)
}

fn abs_num(num: i16) -> i16 {
    if num > 0 {
        num
    } else {
        -num
    }
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        abs_num(self.0) + abs_num(self.1)
    }
}
