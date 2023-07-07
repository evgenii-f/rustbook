pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32, 
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn cause_panic(&self) {
        panic!("Rectangle is panicking here, bro");
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
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
        assert!(
            !smaller.can_hold(&larger),
            "Smaller {:?} holds somehow larger one {:?}", smaller, larger
        );
    }

    #[test]
    #[should_panic]
    fn check_rectangular_panic() {
        let some = Rectangle{width: 1.0, height: 2.0};
        some.cause_panic();
    }
}
