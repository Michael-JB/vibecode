use vibecode::{vibecode, viberun};

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
    fn vibecode_accepts_complexity() {
        // Given
        #[vibecode(
            prompt = "Compute all prime numbers up to the given limit",
            complexity = "medium"
        )]
        fn primes(limit: u64) -> Vec<u64> {}

        // When
        let result = primes(20);

        // Then
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
