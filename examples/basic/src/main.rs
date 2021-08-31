fn main() {
    variable();

    data_type();

    control_flow();
}

fn variable() {
    // 赋值类型需要保持一致
    let x = 5;
    println!("The Value is {}", x);
    let x = 6; // x = 6; 会报错
    println!("The Value is {}", x);

    {
        let x = x * 2;
        println!("The Value of x in the inner scope is {}", x);
    }

    println!("The Value is {}", x);

    let mut y = "five";
    println!("The Value is {}", y);
    y = "six";
    println!("The Value is {}", y);
}

fn data_type() {
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _floored = 2 / 3; // 结果为0

    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false;

    let _c = 'z';

    let _z = 'Z';

    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    println!("The value of y is {}", y);

    let a = [1, 2, 3, 4, 5, 6, 7];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);

    let _b:[i32; 5] = [1, 2 , 3, 4, 5];

    let _c = [3; 3]; // [3, 3, 3]

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);
}   

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true; //
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is {}", number);

    loops();
}

fn loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}


