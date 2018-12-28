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

pub fn specify_lines(target: &str, position: (usize, usize)) -> String {
  let lines = target.split("\n");
  let mut res = "".to_string();

  let (col, row) = position;
  let digits = f32::log(col as f32, 10.0) as i32 + 1;

  let mut indent = "".to_string();
  for _ in 0..digits {
    indent.push_str(" ");
  }
  res.push_str(format!("{} |\n", indent).as_str());

  let line = lines.collect::<Vec<&str>>()[col - 1];
  res.push_str(format!("{}", col).as_str());
  res.push_str(" | ");
  res.push_str(line);

  let mut checkers = "".to_string();
  for _ in 0..(row - 1) {
    checkers.push_str(" ");
  }
  for _ in (row - 1)..line.len() {
    checkers.push_str("^");
  }
  res.push_str(format!("\n{} |{}", indent, checkers).as_str());

  res
}
