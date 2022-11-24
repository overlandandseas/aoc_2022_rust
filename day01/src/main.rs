fn hello_world() -> &'static str {
    &"Hello, world!"
}
fn main() {
    let msg = hello_world();
    println!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        let answer = hello_world();
        assert_eq!(answer, "Hello, world!");
    }
}
