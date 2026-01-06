use vibecode_macros::add;

#[add]
pub fn macro_add() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = macro_add(2, 2);
        assert_eq!(result, 4);
    }
}
