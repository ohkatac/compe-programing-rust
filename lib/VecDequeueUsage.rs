use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn solve() {
  let q:usize = read_value().parse().unwrap();
  
  let mut queue:VecDeque<usize> = VecDeque::new();
  let mut sorted_q:BinaryHeap<Reverse<usize>> = BinaryHeap::new();
  for _ in 0..q {
    let line:Vec<usize> = read_values().iter().map(|s| s.parse().unwrap()).collect();

    match line[0] {
      1 => {
        let x = line[1];
        queue.push_back(x);
      },
      2 => {
        let front_x:usize = if sorted_q.len() > 0 {
          let Reverse(v) = sorted_q.pop().unwrap(); v
        } else { queue.pop_front().unwrap() };

        println!("{}", front_x);
      },
      3 => {
        while let Some(v) = queue.pop_front() {
          sorted_q.push(Reverse(v));
        }
      },
      _ => {},
    }
  }
}