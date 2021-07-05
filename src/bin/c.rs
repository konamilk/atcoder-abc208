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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        k: usize,
        a: [usize; n]
    };

    let mut ans = k / n;
    let kp = k % n;

    let c = a.compress();

    for i in 0..n{
        if c[i] < kp {
            println!("{}", ans + 1)
        }
        else {
            println!("{}", ans)
        }
    }

}

trait Compress<T> {
    fn compress(&self) -> Vec::<usize>;
}

impl<T:Ord + Copy> Compress<T> for [T] {
    fn compress(&self) -> Vec::<usize> {
        let mut v1 = self.to_vec();
        v1.sort();
        v1.dedup();

        let v2 = self.iter()
            .map(|&x| v1.binary_search(&x).unwrap())
            .collect::<Vec::<usize>>();
        v2
    }
}

#[test]
fn compress_test() {
    assert_eq!(vec![5, 2, 3, 6, 7].compress(), vec![2, 0, 1, 3, 4]);
    assert_eq!(vec![8, 8, 8, 7, 7].compress(), vec![1, 1, 1, 0, 0]);

    assert_eq!(vec![-5i64, -3, -4, 1, 0].compress(), vec![0, 2, 1, 4, 3]);
    assert_eq!(vec![5usize, 2, 3, 6, 7].compress(), vec![2, 0, 1, 3, 4]);

    // sort, binary_search()がOrd要求のため浮動小数点には実装しない
    // assert_eq!(vec![10.2, 4.3, 2.8, 5.5, 9.1].compress(), vec![4, 1, 0, 2, 3]);
}