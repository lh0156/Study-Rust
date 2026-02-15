#[derive(Debug)]
struct UserId(u32);

impl From<u32> for UserId {
    fn from(value: u32) -> Self {
        UserId(value)
    }
}

fn main() {
    let a = {
        1 + 2
    }; // a = 3
    let b = {
        1 + 2;
    }; // b = ()
}