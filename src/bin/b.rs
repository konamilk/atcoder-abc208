use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    let source = AutoSource::from("9");
    input!{
        // from source,
        mut n: i64
    };

    let mut ans = 0;

    for i in (1..=10).rev(){
        // println!("{}", i)
        let coin = f(i);
        // println!("{}", coin);
        if coin > n {
            continue
        }

        ans += n / coin;
        n = n % coin;
    }

    println!("{}", ans)
}

fn f(x: i64) -> i64{
    if x == 0{
        return 1i64
    }

    return x * f(x-1)
}

