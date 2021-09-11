
pub trait InspectableResult {
    fn inspect_ok(self) -> Self;
    fn inspect_err(self) -> Self;
    fn inspect_both(self) -> Self;
}

impl<T, E> InspectableResult for Result<T, E> {
    fn inspect_ok(self) -> Self {
        if self.is_ok() {
            println!("It's OK!");
        }
        self
    }

    fn inspect_err(self) -> Self {
        if self.is_err() {
            print!("It's an error");
        }
        self
    }

    fn inspect_both(self) -> Self {
        match self {
            Ok(_) => self.inspect_ok(),
            Err(_) => self.inspect_err(),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::InspectableResult;

    fn test_success() -> Result<usize, String> {
        Ok(0)
    }
    
    fn test_error() -> Result<usize, String> {
        Err("I'm broken".to_string())
    }

    #[test]
    fn err_returns_both() {
        assert!(test_success().inspect_err().is_ok());
        assert!(test_error().inspect_err().is_err());
    }

    #[test]
    fn ok_returns_both() {
        assert!(test_success().inspect_ok().is_ok());
        assert!(test_error().inspect_ok().is_err());
    }

    #[test]
    fn both_returns_both() {
        assert!(test_success().inspect_both().is_ok());
        assert!(test_error().inspect_both().is_err());
    }
}
