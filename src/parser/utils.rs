pub fn specify_position(target: &str, position: usize) -> (usize, usize) {
  let lines = target.split("\n");
  let mut current_row = 0;
  let mut current_col = 0;
  let mut cursor = 0;
  for line in lines {
    let line_len = line.len();
    if cursor < position && position < cursor + line_len {
      current_row = position - cursor - 1;
      break;
    }
    cursor += line_len;
    current_col += 1;
  }

  (current_col + 1, current_row + 1) // for hume
}
