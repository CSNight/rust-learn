use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

extern crate rand;

fn main() {
    let num = 2..=20;
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
    println!("vector test");
    vec_test();
    println!("------------------------------------");
    println!("file read test");
    read_file();
    println!("------------------------------------");
    println!("file write test");
    write_file();
    println!("------------------------------------");
    println!("args test");
    args_test();
    println!("------------------------------------");
    println!("traits test");
    traits_test();
    println!("------------------------------------");
    println!("match test");
    match_test();
    println!("------------------------------------");
    println!("hashmap test");
    hashmap_test();
    println!("------------------------------------");
    println!("number guess game");
    guess_num();
    println!("------------------------------------");
}

fn print_num(range: u32) {
    for i in 1..range {
        println!("{}", i)
    }
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

fn slice_test() {
    let s = "hello world";
    println!("slice of hello world -> {}", &s[0..5]);
    let s = "沉思";
    println!("slice of 沉思 -> {}", &s[0..3]);
    let mut s: [i32; 3] = [1, 2, 3]; //
    //another array define type :-> let mut s = [2; 400];
    s[2] = 2;
    println!("slice length of s -> {}", &s[..].len());
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

fn vec_test() {
    let mut vec_new: Vec<i32> = Vec::new();
    vec_new.push(2);
    let mut vec = vec!["a", "b", "c"];
    vec.push("s");
    vec.remove(2);
    for (index, item) in vec.iter().enumerate() {
        println!("index {} of vec is {}", index, item);
    }
    println!("vec new {}", vec_new[0]);
}

fn read_file() {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(".gitignore").expect("can't open");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("can't read file");
    println!("{}", contents);
}

fn write_file() {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("test.txt").expect("could not create");
    let mut content = String::new();
    content.push_str("test write file");
    file.write_all(content.as_bytes()).expect("can't write file");
}

fn args_test() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    for item in args.iter() {
        println!("{}", item);
    }
}

fn traits_test() {
    struct Person {
        name: String,
        age: i32,
    }
    trait Speak {
        fn speak(&self);
        fn can_speak(&self) -> bool;
    }
    impl Speak for Person {
        fn speak(&self) {
            println!("my name is {}", self.name);
        }

        fn can_speak(&self) -> bool {
            if self.age > 2 {
                return true;
            }
            return false;
        }
    }
    let bob = Person {
        name: String::from("bob"),
        age: 20,
    };
    let baby = Person {
        name: String::from("boby"),
        age: 1,
    };
    println!("Is {} can speak: {}", baby.name, baby.can_speak());
    bob.speak();
}

fn match_test() {
    let number = 20;
    match number {
        1 => println!("equal one"),
        //重点 三个点
        2...10 => println!("greater than one"),
        11 | 12 => println!("equal eleven or twelve"),
        _ => println!("unknown")
    }
}

fn hashmap_test() {
    let mut map: HashMap<&str, &str> = HashMap::with_capacity(20);
    map.insert("a", "b");
    println!("key {} equal {}", "a", map.get("a").expect(""));
    map.insert("a", "c");
    println!("key {} equal {}", "a", map.get("a").expect(""));
    map.insert("b", "b");
    map.insert("c", "b");
    println!("map size is {} and capacity is {}", map.len(), map.capacity());
    map.shrink_to_fit();
    println!("map size is {} and capacity is {}", map.len(), map.capacity());
    match map.get("b") {
        Some(s) => println!("b value is {}", s),
        None => println!("can't find key in map")
    }
    println!("print by reference");
    for (k, v) in &map {
        println!("key {} equal {}", k, v);
    }
    for (_, v) in map.iter_mut() {
        *v = "s"
    }
    println!("print by iter after values mut");
    for (k, v) in map.iter() {
        println!("key {} equal {}", k, v);
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