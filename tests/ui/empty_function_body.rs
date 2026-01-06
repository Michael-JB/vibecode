use vibecode::vibecode;

#[vibecode]
pub fn my_vibecoded_function() {
    println!("This function has a body, which is not allowed.");
}

fn main() {}
