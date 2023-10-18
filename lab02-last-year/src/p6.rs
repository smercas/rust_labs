// Implement the following esoteric language named RainDuck, with the following specifications:
// A program written in this language is a sequence of commands. Each command consists of a single character with a certain semantic.
// Most commands are run sequentially; this means that the next command is executed after it.
// The application has an array of bytes that it uses as memory.
// There's a command offset, which says the index of the current command, and a data offset, which says the index of the current data byte.
//
// The commands are the following:
// > increments the data offset (moves it to the byte to the right)
// < decrements the data offset (moves it to the byte to the left)
// + increments the byte at the data offset
// - decrements the byte at the data offset
// . prints the byte at the data offset as a character
// [ if the byte at the data offset is 0, moves the command offset to the command after the _matching_ ] command to the right
// ] if the byte at the data offset is not 0, moves the command offset to the command after the _matching_ [ command to the left
//
// Example:
// 0 1 2 3 4 5 6 7 8 9
// [ + + [ + + ] ] -
// ^ command offset
//
// In this situation, if the byte is 0, the data offset will move to the 8th index.
// Notice that it's not the following closing bracket, but the closing bracket that matches the opened one.
// 0 1 2 3 4 5 6 7 8 9
// [ + + [ + + ] ] -
//                 ^ command offset
//
// The commands that are hardcoded in main are expected to print "Hello world!\n".
// For more information see https://en.wikipedia.org/wiki/Brainfuck#Language_design

fn do_command(commands: &[char], command_offset: &mut usize, data: &mut [u8], data_offset: &mut usize) {
  let next: char = commands[*command_offset];
  match next {
    '>' => {
      if *data_offset >= data.len() - 1 {
        panic!("Data offset overflow");
      }
      *data_offset += 1;
    },
    '<' => {
      if *data_offset <= usize::MIN {
        panic!("Data offset underflow");
      }
      *data_offset -= 1;
    },
    '+' => {
      if data[*data_offset] >= u8::MAX {
        panic!("Data member overflow");
      }
      data[*data_offset] += 1;
    },
    '-' => {
      if data[*data_offset] <= u8::MIN {
        panic!("Data member underflow");
      }
      data[*data_offset] -= 1;
    },
    '.' => {
      print!("{}", data[*data_offset] as char);
    },
    '[' => {
      if data[*data_offset] == 0 {
        let mut counter: usize = 1;
        if *command_offset < commands.len() {
          *command_offset += 1;
        }
        while counter > 0 && *command_offset < commands.len() {
          if commands[*command_offset] == '[' {
            counter += 1;
          } else if commands[*command_offset] == ']' {
            counter -= 1;
          }
          *command_offset += 1;
        }
        if counter != 0 || *command_offset >= commands.len() {
          panic!("bad input: unmached '['");
        }
        *command_offset -= 1;
      }
    },
    ']' => {
      if data[*data_offset] != 0 {
        let mut counter: usize = 1;
        if *command_offset != usize::MAX {
          *command_offset -= 1;
        }
        while counter > 0 && *command_offset != usize::MAX {
          if commands[*command_offset] == ']' {
            counter += 1;
          } else if commands[*command_offset] == '[' {
            counter -= 1;
          }
          *command_offset -= 1;
        }
        if counter != 0 {
          panic!("bad input: unmatched ']'");
        }
        *command_offset += 1;
      }
    },
    _ => {
      panic!("bad input: unexpected character passed");
    }
  };
  /*
  if next == '>' {
    if *data_offset >= data.len() - 1 {
      panic!("Data offset overflow");
    }
    *data_offset += 1;
  } else if next == '<' {
    if *data_offset <= usize::MIN {
      panic!("Data offset underflow");
    }
    *data_offset -= 1;
  } else if next == '+' {
    if data[*data_offset] >= u8::MAX {
      panic!("Data member overflow");
    }
    data[*data_offset] += 1;
  } else if next == '-' {
    if data[*data_offset] <= u8::MIN {
      panic!("Data member underflow");
    }
    data[*data_offset] -= 1;
  } else if next == '.' {
    print!("{}", data[*data_offset] as char);
  } else if next == '[' {
    if data[*data_offset] == 0 {
      let mut counter: usize = 1;
      if *command_offset < commands.len() {
        *command_offset += 1;
      }
      while counter > 0 && *command_offset < commands.len() {
        if commands[*command_offset] == '[' {
          counter += 1;
        } else if commands[*command_offset] == ']' {
          counter -= 1;
        }
        *command_offset += 1;
      }
      if counter != 0 || *command_offset >= commands.len() {
        panic!("bad input: unmached '['");
      }
      *command_offset -= 1;
    }
  } else if next == ']' {
    if data[*data_offset] != 0 {
      let mut counter: usize = 1;
      if *command_offset != usize::MAX {
        *command_offset -= 1;
      }
      while counter > 0 && *command_offset != usize::MAX {
        if commands[*command_offset] == ']' {
          counter += 1;
        } else if commands[*command_offset] == '[' {
          counter -= 1;
        }
        *command_offset -= 1;
      }
      if counter != 0 {
        panic!("bad input: unmatched ']'");
      }
      *command_offset += 1;
    }
  } else {
    panic!("bad input: unexpected character passed");
  }
  */
  *command_offset += 1;
}

pub fn main() {
  let commands: [char; 106] = [
      '+', '+', '+', '+', '+', '+', '+', '+', '[', '>', '+', '+', '+', '+', '[', '>', '+', '+',
      '>', '+', '+', '+', '>', '+', '+', '+', '>', '+', '<', '<', '<', '<', '-', ']', '>', '+',
      '>', '+', '>', '-', '>', '>', '+', '[', '<', ']', '<', '-', ']', '>', '>', '.', '>', '-',
      '-', '-', '.', '+', '+', '+', '+', '+', '+', '+', '.', '.', '+', '+', '+', '.', '>', '>',
      '.', '<', '-', '.', '<', '.', '+', '+', '+', '.', '-', '-', '-', '-', '-', '-', '.', '-',
      '-', '-', '-', '-', '-', '-', '-', '.', '>', '>', '+', '.', '>', '+', '+', '.',
  ];
  let mut data: [u8; 16] = [0; 16];
  let mut command_offset: usize = 0;
  let mut data_offset: usize = 0;

  while command_offset < commands.len() {
      do_command(&commands, &mut command_offset, &mut data, &mut data_offset);
  }
}
