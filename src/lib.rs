enum Color {
    Blue,
    Green,
    Red,
}

pub fn test() {
    let a = Color::Blue;
    match a{
        Color::Blue => println!("blue"),
        Color::Green => todo!(),
        Color::Red => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test;
    #[test]
    fn it_works() {
        test();
    }
}
