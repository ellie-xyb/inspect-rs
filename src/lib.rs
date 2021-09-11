struct Job {
    name: String,
}

enum Status<T> {
    Pending(T),
    Accepted(T),
    Rejected(T),
}

fn print_status<T: ToString>(status: Status<T>) {
    match status {
        Status::Pending(t) => println!("p: {}", t.to_string()),
        Status::Accepted(t) => println!("a: {}", t.to_string()),
        Status::Rejected(t) => println!("r: {}", t.to_string()),
    }
}

pub fn test() {
    let a = Status::Pending("damn bear!");
    print_status(a);
}

#[cfg(test)]
mod tests {
    use crate::test;
    #[test]
    fn it_works() {
        test();
    }
}
