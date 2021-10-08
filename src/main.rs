use std::io;  
use proconio::input;
use std::cmp;

fn calc_a_b(a_b: (usize, usize)) -> (usize, usize) {
  return (a_b.0, a_b.0 + a_b.1 - 1);
}

fn binary_search<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: &T) -> usize{
  let mut left  = 0;
  let mut right = data.len()-1;

  while left < right {
    let middle = (left + right)/2;
    if data[middle] < *target { left = middle+1; continue; }
    else { right = middle; continue; }
  }
  left
}

fn binary_search_upper_eq<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: T) -> usize{
  let target_idx = binary_search(data, &target);
  if target_idx == data.len()-1 { return target_idx; }
  if data[target_idx] < target { return target_idx+1; }
  
  return target_idx;
}

fn binary_search_lower_eq<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: T) -> usize{
  let target_idx = binary_search(data, &target);
  if target_idx == 0 { return target_idx; }
  if data[target_idx] > target { return target_idx-1; }
  
  return target_idx;
}

fn main() {
  input! {
    n: usize,
    a_b_arr: [(usize, usize); n],
  }

  let mut areas:Vec<usize> = Vec::new();
  let mut score:Vec<usize> = Vec::new();

  areas.push(a_b_arr[0].0);
  areas.push(a_b_arr[0].1);
  score.push(1);
  
  for i in 1..a_b_arr.len() {
    let (a, b) = calc_a_b(a_b_arr[i]);

    let s_i = binary_search_lower_eq(&areas, a);
    let e_i = binary_search_upper_eq(&areas, b);

    if s_i % 2 != 0 {
      
    }
    else {}

    if e_i % 2 != 1 {
    }
    else {}
    
    let s_i = binary_search_lower_eq(&areas, a);
    let e_i = binary_search_upper_eq(&areas, b);
    for j in s_i/2..e_i/2 {
      score[j] += 1;
    }
  }

}