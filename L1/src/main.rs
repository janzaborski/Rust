use std::io;
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut input = String::new();
    let mut x:u64;
    let mut tab:[u64; 10];
    let mut checked_tab:[bool; 10];

    let mut file = File::create("xyz.txt").expect("Unable to create file");

    let exited_by_error = 'main_loop: loop {
        input.clear();
        println!("Podaj liczbę:");
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        x = match input.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => break true
        };
        if x == 0 {
            break false;
        }
        x += rand::rng().random_range(0..=5);
        println!("Nowa wartość: {x}");
        tab = potegi_x(x);
        checked_tab = check_collatz_array(tab);

        for i in 0..10 {
            match writeln!(&mut file, "{} : {}", tab[i], checked_tab[i]) {
                Ok(_) => (),
                Err(_) => break 'main_loop true
            }
        }

        let cos : (u64, bool) = funkcja(x);
        if cos.1 {
            println!("spełnione");
        }
    };
    if exited_by_error {
        println!("Gra przerwana z powodu błędu.");
    }
    else {
        println!("Gra przerwana przez użytkownika.");
    }
}

fn potegi_x(x:u64) -> [u64; 10]{
    let mut ret: [u64; 10] = [x; 10];

    for i in 1..10 {
        ret[i] = ret[i-1] * x;
    }

    ret
}

fn collatz_function(n:u64) -> u64{
    if n % 2 == 0 {
        return n/2;
    }
    3 * n + 1
}

fn check_collatz_array(arr: [u64; 10]) -> [bool; 10] {
    let mut ret: [bool; 10] = [false; 10];

    for i in 0..10 {
        let mut n = arr[i];
        'inner: for j in 1..=100 {
            if n == 1 {
                ret[i] = true;
                break 'inner;
            }
            if j == 100 {
                ret[i] = false;
                break 'inner;
            }
            n = collatz_function(n);
        }
    }

    ret
}

fn funkcja(x: u64) -> (u64, bool) {
    let mut ret: (u64, bool) = (0, false);
    'outer: for i in 1..10 {
        ret.0 = i;
        for j in 1..=i {
            if j * i == x {
                ret.1 = true;
                break 'outer;
            }
        }
    }

    ret
}