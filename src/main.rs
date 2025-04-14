fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn variable() {
    let name = "Riski Drian Pratama";
    println!("Hello {}", name);
}

#[test]
fn mutable() {
    let mut name = "Riski Drian Pratama";
    println!("Hello {}", name);

    name = "Budi";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Riski Drian Pratama";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Riski Drian Pratama";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn comment() {
    // let name = "Riski Drian Pratama";
    // println!("Hello {}", name);

    /* let name = 10;
    println!("Hello {}", name); */
}

#[test]
fn explicit() {
    let age: i32 = 20;

    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1_000_000_000;
    let e: i8 = d as i8; // integer overflow
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;

    let c = a * b;
    println!("{}", c);

    let d = a + b;
    println!("{}", d);

    let e = a - b;
    println!("{}", e);
    
    let f = a / b;
    println!("{}", f);
    
    let g = a % b;
    println!("{}", g);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);

    a *= 10;
    println!("{}", a);

    a /= 10;
    println!("{}", a);

    a %= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {a}", b);
}

#[test]
fn comparison() {
    let a = 20;
    let b = 20;

    let result: bool = a >= b;
    println!("{result}");
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type () {
    let char1: char = 'a';
    let char2: char = '.';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple () {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{a}, {b}, {c}");
    
    // destructuring tuple
    let (a, b, c) = data;
    println!("{a}, {b}, {c}");

    // mutable tuple
    let mut data2: (i32, f64, bool) = (10, 10.5, true);

    data2.0 = 234;
    println!("{:?}", data2);
}

// unit atau tuple kosong
fn unit () {
    println!("hello");
}
#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
} 

#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[3];
    println!("{a} {b}");

    array[0] = 12;
    array[4] = 23;
    println!("{:?}", array);

    // cek panjang array
    let length_array: usize = array.len();
    println!("{}", length_array);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{}", matrix[0][1]);
}

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    let num: i32 = 1; // variable scope

    { // inner scope
        println!("inner num: {num}");
        let num2: i32 = 2;
        println!("inner num2: {num2}");
    }

    // println!("inner num2: {}", num2); // error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Drian");

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Pratama");

    println!("{} {}", a, b);
}

#[test]
fn string_slice() {
    let name: &str = " Hi Bro ";
    let trim: &str = name.trim();

    println!("{name}");
    println!("{trim}");

    let mut username: &str = "Drian";
    username = "Test";

    println!("{username}");
}

#[test]
fn string_type() {
    let mut name: String = String::from("Drian P");
    name.push_str(" Riski");
    println!("{name}");

    let drian: String = name.replace("P", "Pratama");
    println!("{drian}");
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai disini

    { // b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai disini
        println!("{b}");
    } // scope b selesai, b dihapus, b tidak bisa di akses lagi

    println!("{a}");
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // a copy ke b karena stack

    println!("{b} {a}");
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Drian");

    // ownership dari name1 dipindahkan ke name2 karena heap
    let name2 = name1;

    println!("{name2}");
    // println!("{name1}"); // akan error
}

#[test]
fn clone() {
    let name1 = String::from("Drian");
    let name2 = name1.clone();

    println!("{name1} {name2}");
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 10 {
        println!("Good");
    } else if value >= 6 {
        println!("Not Bad");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Very Bad");
    }
}

#[test]
fn let_statement() {
    let value = 3;
    let result = if value >= 10 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{result}");
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {counter}");
        }

        counter += 1;
    }
}

#[test]
fn array_iteration_while() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value {}", value);
    }
}