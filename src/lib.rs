#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use vibecode_macros::vibecode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add() {
        // Given
        #[vibecode]
        fn add(a: u64, b: u64) -> u64 {}

        // When
        let result = add(2, 2);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn it_can_sort() {
        // Given
        #[vibecode]
        fn sort(values: Vec<u64>) -> Vec<u64> {}

        // When
        let result = sort(vec![3, 1, 4, 2]);

        // Then
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_can_be_prompted() {
        // Given
        #[vibecode(prompt = "Sort in descending order")]
        fn sort(values: Vec<u64>) -> Vec<u64> {}

        // When
        let result = sort(vec![3, 1, 4, 2]);

        // Then
        assert_eq!(result, vec![4, 3, 2, 1]);
    }
}
