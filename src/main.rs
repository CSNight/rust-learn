use std::io;
use rand::Rng;
use std::cmp::Ordering;

extern crate rand;

fn main() {
    println!("------------------------------------");
    println!("enum test");
    enum_test();
    println!("------------------------------------");
    println!("function test");
    let a = 10;
    print_num(a);
    println!("------------------------------------");
    println!("reference test");
    mut_ref_test();
    println!("------------------------------------");
    println!("slice test");
    slice_test();
    println!("------------------------------------");
    println!("struct test");
    struct_test();
    println!("------------------------------------");
    println!("number guess game");
    guess_num();
    println!("------------------------------------");
}

fn slice_test() {
    let s = "hello world";
    println!("slice of hello world -> {}", &s[0..5]);
    let s = "沉思";
    println!("slice of 沉思 -> {}", &s[0..3]);
    let mut s: [i32; 3] = [1, 2, 3];
    s[2] = 2;
    println!("slice length of s -> {}", &s[..].len());
}

fn enum_test() {
    //可为不同类型
    enum Direction {
        //enum members type can be different
        Up(String),
        Down(String),
        Left(String),
        Right(String),
    }
    let direct: Direction = Direction::Down(String::from("Down"));
    match direct {
        Direction::Down(val) => println!("Direction is {}", val),
        Direction::Up(val) => println!("Direction is {}", val),
        Direction::Left(val) => println!("Direction is {}", val),
        Direction::Right(val) => println!("Direction is {}", val),
    }
}

fn mut_ref_test() {
    let mut a = 10;
    //immutable ref
    let ar1 = &a;
    println!("{}", ar1);
    //mutable ref1
    let amr = &mut a;
    *amr += 1;
    //amr will destroy after execute if it not used in code below;
    //mutable ref2
    let amr2 = &mut a;
    *amr2 += 1;
    println!("{}", amr2);
    //call println!("{} and {}", amr, amr2); will show error to avoid "data races" exception
    //mutable ref can only call once at same time
    //immutable ref
    let ar2 = &a;
    println!("{}", ar2);
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct RGB(u8, u8, u8);

impl Color {
    fn print_color(&self) {
        println!("Color - R:{} G:{} B:{}", self.red, self.green, self.blue);
    }
    fn is_red(&self) -> bool {
        return self.red == 255 && self.blue == 0 && self.green == 0;
    }
}

impl ToString for RGB {
    fn to_string(&self) -> String {
        format!("Red - R:{} G:{} B:{}", self.0, self.1, self.2)
    }
}

fn struct_test() {
    let mut bg = Color { red: 200, green: 100, blue: 10 };
    let red = RGB(255, 0, 0);
    bg.blue = 20;
    print_struct(&bg);
    //reference can used many times and does not change the variable's ownership and scope
    print_struct(&bg);
    //impl with struct
    bg.print_color();
    println!("Impl struct print Color is red? {}", bg.is_red());
    //impl traits for struct
    println!("Original Print Red - R:{} G:{} B:{}", red.0, red.1, red.2);
    println!("Impl traits print {}", red.to_string());
}

fn print_struct(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}

fn print_num(range: u32) {
    for i in 1..range {
        println!("{}", i)
    }
}

fn guess_num() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}