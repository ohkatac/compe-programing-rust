fn rotate_90<T: Clone+Copy>(table: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let mut ret_val = table.clone();
  let n = table.len();

  for i in 0..n {
    for j in 0..n {
      ret_val[(n-1)-j][i] = table[i][j];
    }
  }
  return ret_val.to_vec();
}

fn rotate_180<T: Clone+Copy>(table: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let mut ret_val = table.clone();
  let n = table.len();

  for i in 0..n {
    for j in 0..n {
      ret_val[(n-1)-i][(n-1)-j] = table[i][j];
    }
  }
  return ret_val.to_vec();
}

fn rotate_270<T: Clone+Copy>(table: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let mut ret_val = table.clone();
  let n = table.len();

  for i in 0..n {
    for j in 0..n {
      ret_val[j][(n-1)-i] = table[i][j];
    }
  }
  return ret_val.to_vec();
}

fn move_point<T: Copy+Clone>(table: &Vec<Vec<T>>, m_i: usize, m_j: usize) -> Vec<Vec<T>> {
  let n = table.len();
  let mut ret_val:Vec<Vec<T>> = Vec::new();

  for i in m_i..n {
    for j in m_j..n {
      // if table[i][j] == '#' { 
      ret_val[i-m_i][j-m_j] = table[i][j];
      // }
    }
  }
  return ret_val;
}