
enum Hello {
    H1(u8),
    E1(u8),
    L1(u8),
    L2(u8),
    O1(u8),
    Space(u8),
    W1(u8),
    O2(u8),
    R1(u8),
    L3(u8),
    D1(u8),
    ExclamationMark(u8)
}

struct HelloWorld {
    h1: Hello,
    e1: Hello,
    l1: Hello,
    l2: Hello,
    o1: Hello,
    space: Hello,
    w1: Hello,
    o2: Hello,
    r1: Hello,
    l3: Hello,
    d1: Hello,
    exclamation_mark: Hello
}

impl HelloWorld {

    pub fn new(h1: Hello, e1: Hello, l1: Hello, l2: Hello, o1: Hello, space: Hello, w1: Hello, o2: Hello, r1: Hello, l3: Hello, d1: Hello, exclamation_mark: Hello) -> Self {
        Self {
            h1,
            e1,
            l1,
            l2,
            o1,
            space,
            w1,
            o2,
            r1,
            l3,
            d1,
            exclamation_mark
        }
    }

    pub fn create_hello_world(&self) -> String {
        let h1: u8 = self.world(&self.h1);
        let e1: u8 = self.world(&self.e1);
        let l1: u8 = self.world(&self.l1);
        let l2: u8 = self.world(&self.l2);
        let o1: u8 = self.world(&self.o1);
        let space: u8 = self.world(&self.space);
        let w1: u8 = self.world(&self.w1);
        let o2: u8 = self.world(&self.o2);
        let r1: u8 = self.world(&self.r1);
        let l3: u8 = self.world(&self.l3);
        let d1: u8 = self.world(&self.d1);
        let exclamation_mark: u8 = self.world(&self.exclamation_mark);

        let mut hello = vec![];
        hello.push(h1);
        hello.push(e1);
        hello.push(l1);
        hello.push(l2);
        hello.push(o1);
        hello.push(space);
        hello.push(w1);
        hello.push(o2);
        hello.push(r1);
        hello.push(l3);
        hello.push(d1);
        hello.push(exclamation_mark);

        convert_hello_world!(hello)
    }
    
    fn world(&self, hello_: &Hello) -> u8 {
        match hello_ {
            Hello::H1(i) => *i,
            Hello::E1(i) => *i,
            Hello::L1(i) => *i,
            Hello::L2(i) => *i,
            Hello::O1(i) => *i,
            Hello::Space(i) => *i,
            Hello::W1(i) => *i,
            Hello::O2(i) => *i,
            Hello::R1(i) => *i,
            Hello::L3(i) => *i,
            Hello::D1(i) => *i,
            Hello::ExclamationMark(i) => *i,
        }
    }
}

#[macro_export]
macro_rules! convert_hello_world {
    ($byte:expr) => {
        {
            let mut text: String = "".to_string();
            for ch in $byte {
                match ch {
                    0b1001000 => text.push('H'),
                    0b1100101 => text.push('e'),
                    0b1101100 => text.push('l'),
                    0b1101111 => text.push('o'),
                    0b100000  => text.push(' '),
                    0b1110111 => text.push('w'),
                    0b1110010 => text.push('r'),
                    0b1100100 => text.push('d'),
                    0b100001  => text.push('!'),
                    _ => ()
                }
            }
            text
        }
    };
}

#[macro_export]
macro_rules! print_hello_world {
    ($hello:expr) => {
        for sym in $hello.chars() {
            print!("{}", sym);
        }
        print!("\n");
    };
}

#[test]
fn test_hello_world() {
    let hello_world = HelloWorld::new(
        Hello::H1(0b1001000),
        Hello::E1(0b1100101),
        Hello::L1(0b1101100),
        Hello::L2(0b1101100),
        Hello::O1(0b1101111),
        Hello::Space(0b100000),
        Hello::W1(0b1110111),
        Hello::O2(0b1101111),
        Hello::R1(0b1110010),
        Hello::L3(0b1101100),
        Hello::D1(0b1100100),
        Hello::ExclamationMark(0b100001)
    );

    let hello = hello_world.create_hello_world();
    assert_eq!(hello, "Hello world!");
}

fn main() {
    let hello_world = HelloWorld::new(
        Hello::H1(0b1001000),
        Hello::E1(0b1100101),
        Hello::L1(0b1101100),
        Hello::L2(0b1101100),
        Hello::O1(0b1101111),
        Hello::Space(0b100000),
        Hello::W1(0b1110111),
        Hello::O2(0b1101111),
        Hello::R1(0b1110010),
        Hello::L3(0b1101100),
        Hello::D1(0b1100100),
        Hello::ExclamationMark(0b100001)
    );

    print_hello_world!(hello_world.create_hello_world());
}
