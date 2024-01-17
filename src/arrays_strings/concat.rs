// Sample function to test Rust modules

fn concat(left: &str, right: &str) -> String {
    println!("Add two strings");
    let mut concat = left.to_owned();
    concat.push_str(right);
    concat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = concat("Gordon", " Freeman");
        assert_eq!(result, "Gordon Freeman");
    }
}
