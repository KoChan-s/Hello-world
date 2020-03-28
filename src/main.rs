
fn hello_world() {
    let h1: u8 = 104;
    let e1: u8 = 101;
    let l1: u8 = 108;
    let l2: u8 = 108;
    let o1: u8 = 111;
    let space: u8 = 32;
    let w1: u8 = 119;
    let o2: u8 = 111;
    let r1: u8 = 114;
    let l3: u8 = 108;
    let d1: u8 = 100;
    let exclamation_mark: u8 = 33;

    hello_world!(vec![h1, e1, l1, l2, o1, space, w1, o2, r1, l3, d1, exclamation_mark]);
}

#[macro_export]
macro_rules! hello_world {
    ($byte:expr) => {
        println!("{}", String::from_utf8($byte).unwrap());
    };
}

fn main() {
    hello_world();
}
