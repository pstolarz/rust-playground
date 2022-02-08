mod inner {
    pub fn f() -> i32 { 0 }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_f0() {
            assert!(f() == 0);
        }
    }
}

pub fn test() -> i32 {
    inner::f();
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert!(test() == 1);
    }
}
