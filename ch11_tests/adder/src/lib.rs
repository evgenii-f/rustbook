pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: f32,
    height: f32, 
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const W_LARGE: f32 = 10.;
    const H_LARGE: f32 = 8.;
    const W_SMALL: f32 = 4.;
    const H_SMALL: f32 = 4.;

    #[test]
    fn if_larger_holds_smaller() {
        let larger = Rectangle{width: W_LARGE, height: H_LARGE};
        let smaller = Rectangle{width: W_SMALL, height: H_SMALL};
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn if_smaller_holds_larger() {
        let larger = Rectangle{width: W_LARGE, height: H_LARGE};
        let smaller = Rectangle{width: W_SMALL, height: H_SMALL};
        assert!(!smaller.can_hold(&larger));
    }
}
