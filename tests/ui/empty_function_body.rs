use vibecode_macros::add;

#[add]
pub fn macro_add() {
    println!("This function has a body, which is not allowed.");
}

fn main() {}
