use vibecode::viberun;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viberun_can_multiply() {
        // When
        let result = viberun!("Multiply 3 and 4");

        // Then
        assert_eq!(result, 12);
    }

    #[test]
    fn viberun_can_sort() {
        // When
        let result = viberun!(
            "Sort an input vec of numbers in descending order",
            vec![9, 1, 5, 6]
        );

        // Then
        assert_eq!(result, vec![9, 6, 5, 1]);
    }
}
