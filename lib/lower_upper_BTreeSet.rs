use proconio::input;
use std::collections::BTreeSet;

 
fn main() {
  input! {
    l: usize,
    q: usize,
    c_x: [(usize, usize); q],
  }

  let mut x_set:BTreeSet<usize> = BTreeSet::new();
  x_set.insert(0);
  x_set.insert(l);
  for c_x_i in c_x.iter() {
    let c = c_x_i.0;
    let x = c_x_i.1;

    if c == 1 {
      x_set.insert(x);
    }
    else {
      let lower:&usize = x_set.range(0..x).last().unwrap();
      let upper:&usize = x_set.range(x+1..).next().unwrap();

      println!("{}", upper - lower);
    }
  }
}