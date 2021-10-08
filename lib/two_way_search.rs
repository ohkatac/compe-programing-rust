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
fn binary_search_upper<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: T) -> usize{
  let target_idx = binary_search(data, &target);
  if target_idx == data.len()-1 { return target_idx; }
  if data[target_idx] <= target { return target_idx+1; }
  
  return target_idx;
}

fn binary_search_lower_eq<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: T) -> usize{
  let target_idx = binary_search(data, &target);
  if target_idx == 0 { return target_idx; }
  if data[target_idx] > target { return target_idx-1; }
  
  return target_idx;
}
fn binary_search_lower<T: Eq+PartialEq+Ord+PartialOrd>(data: &[T], target: T) -> usize{
  let target_idx = binary_search(data, &target);
  if target_idx == 0 { return target_idx; }
  if data[target_idx] >= target { return target_idx-1; }
  
  return target_idx;
}