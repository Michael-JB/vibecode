use vibecode::{vibecode, viberun};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viberun_accepts_prompt() {
        let result = viberun!("Multiply 3 and 4");
        assert_eq!(result, 12);
    }

    #[test]
    fn viberun_accepts_args() {
        let result = viberun!("Multiply two inputs", 3, 4);
        assert_eq!(result, 12);
    }
}
