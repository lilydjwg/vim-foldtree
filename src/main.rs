use std::io::{BufRead, Write, stdin, stdout, BufWriter};
use std::cmp::Ordering;
use std::fmt::Write as FmtWrite;

use smartstring::alias::String;
use regex::Regex;

#[derive(Debug)]
enum LineKind {
  NotInTree,
  IndentedText(u32),
  PrevAt(u32),
}

fn main() {
  let mut stdin = stdin().lock();
  let mut line = std::string::String::new();
  let mut line_kinds = Vec::new();
  let regex = Regex::new(r"^\s*([├│└  ]*[├└─])").unwrap();

  while let Ok(n) = stdin.read_line(&mut line) {
    if n == 0 {
      break;
    }
    if let Some(captures) = regex.captures(&line) {
      let tree_match = captures.get(1).unwrap();
      let end = tree_match.end() as u32;
      let tree_str = tree_match.as_str();
      let point_up = tree_str.trim_end_matches('─');
      let distance = (tree_str.len() - point_up.len()) as u32;
      line_kinds.push(LineKind::PrevAt(end - distance));
    } else {
      let n = line.len() - line.trim_start_matches([' ', ' ', '│']).len();
      if n > 0 {
        line_kinds.push(LineKind::IndentedText(n as u32));
      } else {
        line_kinds.push(LineKind::NotInTree);
      }
    }
    line.clear();
  }

  let mut folds = Vec::with_capacity(line_kinds.len());
  let mut level_positions = Vec::new();
  for w in line_kinds.windows(2) {
    let fold = match w {
      [LineKind::NotInTree, LineKind::NotInTree | LineKind::IndentedText(_)] => String::from("0\n"),
      [LineKind::NotInTree, LineKind::PrevAt(b)] => {
        assert!(level_positions.is_empty());
        level_positions.push(b);
        String::from(">1\n")
      },
      [LineKind::PrevAt(_), LineKind::NotInTree] => {
        let mut s = String::new();
        writeln!(s, "<{}", level_positions.len()).unwrap();
        level_positions.clear();
        s
      },
      [LineKind::PrevAt(a), LineKind::PrevAt(b)] => {
        let mut s = String::new();
        match a.cmp(b) {
          Ordering::Less => {
            // opening a substree
            level_positions.push(b);
            writeln!(s, ">{}", level_positions.len()).unwrap();
          },
          Ordering::Greater => {
            // closing a substree
            writeln!(s, "<{}", level_positions.len()).unwrap();
            let pos = match level_positions.binary_search(&b) {
              Ok(p) => p,
              Err(p) => p,
            };
            level_positions.truncate(pos + 1);
          },
          Ordering::Equal => {
            // keep same
            writeln!(s, "{}", level_positions.len()).unwrap();
          }
        }
        s
      },
      [LineKind::PrevAt(_), LineKind::IndentedText(_)] => {
        let mut s = String::new();
        writeln!(s, "{}", level_positions.len()).unwrap();
        s
      },
      [LineKind::IndentedText(_), LineKind::PrevAt(b)] => {
        let mut s = String::new();
        // opening a substree
        level_positions.push(b);
        writeln!(s, ">{}", level_positions.len()).unwrap();
        s
      },
      [LineKind::IndentedText(_), _] => {
        let mut s = String::new();
        writeln!(s, "{}", level_positions.len()).unwrap();
        s
      },
      [] | [_] | [_, _, _, ..] => unreachable!(),
    };
    folds.push(fold);
  }
  {
    let mut s = String::new();
    writeln!(s, "{}", level_positions.len()).unwrap();
    folds.push(s)
  }

  let mut stdout = BufWriter::new(stdout().lock());
  for f in folds {
    if stdout.write_all(f.as_bytes()).is_err() {
      break;
    }
  }
}
