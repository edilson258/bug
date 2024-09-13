const RED_UNDERSCORE: &str = "\x1b[4m\x1b[31m";
const YELLOW_UNDERSCORE: &str = "\x1b[4m\x1b[33m";
const RESET: &str = "\x1b[0m";

pub fn highlight_error(haystack: &str, slice_start: usize, slice_end: usize) -> String {
  highlight(haystack, slice_start, slice_end, RED_UNDERSCORE)
}

pub fn highlight_warning(haystack: &str, slice_start: usize, slice_end: usize) -> String {
  highlight(haystack, slice_start, slice_end, YELLOW_UNDERSCORE)
}

pub fn highlight(haystack: &str, slice_start: usize, slice_end: usize, color: &str) -> String {
  // This 👇 operation is very expensive @TODO: make it less expensive
  let haystack = &haystack.chars().collect::<Vec<char>>();
  let haystack_len = haystack.len();
  let slice_end = if haystack_len > slice_end { slice_end } else { haystack_len };

  // absolute position in the `haystack`
  let mut cursor = 0;
  // keeps track of how many new lines we already found
  let mut line_counter = 1;
  // points to the first char of the current line
  let mut begin_of_line_index = 0;
  // points to the first line that the slice occupies
  let mut line_number_where_slice_begin = 0;
  // points to the first char of the line where the slice occupies
  let mut begin_of_line_where_the_slice_start = 0;
  // points to the last line that the slice occupies
  let mut line_number_where_slice_end = 0;

  while cursor < haystack_len {
    if cursor == slice_start {
      line_number_where_slice_begin = line_counter;
      begin_of_line_where_the_slice_start = begin_of_line_index;
    }

    if haystack[cursor] == '\n' {
      line_counter += 1;
      begin_of_line_index = cursor + 1;
      if cursor >= slice_end {
        line_number_where_slice_end = line_counter;
        break;
      }
    }
    cursor += 1;
  }

  let mut out = String::new();
  cursor = begin_of_line_where_the_slice_start;

  out.push_str("    |\n");
  for line_number in line_number_where_slice_begin..line_number_where_slice_end {
    out.push_str(&format!("  {} | ", line_number));
    while haystack[cursor] != '\n' {
      if cursor >= slice_start && cursor <= slice_end {
        out.push_str(&format!("{}{}{}", color, haystack[cursor], RESET));
      } else {
        out.push(haystack[cursor]);
      }
      cursor += 1;
    }
    out.push('\n');
    cursor += 1;
  }
  out.push_str("    |\n");

  out
}
