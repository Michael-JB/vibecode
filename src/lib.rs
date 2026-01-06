#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use vibecode_macros::vibecode;
pub use vibecode_macros::viberun;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vibecode_can_add() {
        // Given
        #[vibecode]
        fn add(a: u64, b: u64) -> u64 {}

        // When
        let result = add(2, 2);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn vibecode_accepts_prompt() {
        // Given
        #[vibecode(prompt = "Sort in descending order")]
        fn sort(values: Vec<u64>) -> Vec<u64> {}

        // When
        let result = sort(vec![3, 1, 4, 2]);

        // Then
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

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
