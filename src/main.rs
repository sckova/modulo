use clap::{Arg, command};

fn clapper() -> (i32, i32) {
    let matches = command!()
        .arg(Arg::new("x")
            .required(true)
            .value_parser(clap::value_parser!(i32)))
        .arg(Arg::new("y")
            .required(true)
            .value_parser(clap::value_parser!(i32)))
        .get_matches();

    println!(
        "x: {:?}",
        matches.get_one::<i32>("x").expect("required")
    );
    println!(
        "y: {:?}",
        matches.get_one::<i32>("y").expect("required")
    );

    let x: i32 = *matches.get_one::<i32>("x").expect("required");
    let y: i32 = *matches.get_one::<i32>("y").expect("required");
    (x, y)
}




// use euclidean gcd algorithm
fn gcd(a: i32, b: i32) -> i32{
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

// get all positive divisors of n
fn divisors(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }
    divs.sort_unstable();
    divs
}

// main
fn valid_ratios(x: i32, y: i32) -> Vec<(i32, i32)> {
    let g: i32 = gcd(x, y);
    let base_w: i32 = x / g;
    let base_h: i32 = y / g;
    let ks = divisors(g);
    ks.into_iter()
        .map(|k| (base_w * k, base_h * k))
        .collect()
}

fn main() {
    let (x, y) = clapper();
    let res = valid_ratios(x, y);
    println!("{:#?}", res);
}
