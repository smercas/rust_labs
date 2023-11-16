#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid line format: {}\nNumber of elements:{}\n", .0, .1)]
  LineFormatError(String, usize),
  #[error("{}", .0)]
  IoError(#[from] std::io::Error),
}

trait Command {
  fn get_name(&self) -> &str;
  fn execute(&mut self, arguments: std::str::SplitWhitespace);
}

struct Ping {}
impl Command for Ping {
  fn get_name(&self) -> &str {
    "ping"
  }
  fn execute(&mut self, _arguments: std::str::SplitWhitespace) {
    println!("pong");
  }
}

struct Count {}
impl Command for Count {
  fn get_name(&self) -> &str {
    "count"
  }
  fn execute(&mut self, arguments: std::str::SplitWhitespace) {
    println!("counted {} args", arguments.count());
  }
}

struct Times {
  count: usize,
}
impl Command for Times {
  fn get_name(&self) -> &str {
    "times"
  }
  fn execute(&mut self, _arguments: std::str::SplitWhitespace) {
    println!("increased the counter from {} to {}", self.count, self.count + 1);
    self.count += 1;
  }
}

struct Terminal {
  commands: Vec<Box<dyn Command>>,
  file_name: &'static str,
}
impl Terminal {
  fn new() -> Terminal {
    Terminal {
      commands: Vec::new(),
      file_name: "commands.txt",
    }
  }
  fn register(&mut self, command: Box<dyn Command>) {
    self.commands.push(command);
  }
  fn run(mut self) -> Result<(), Error> {
    // loop {

    // }
    let file: String = std::fs::read_to_string(self.file_name)?;
    'main_loop: for line in file.lines() {
      let (raw_command_name, arguments): (&str, &str) = line.trim_start().split_once(' ').unwrap_or((line, ""));
      let command_name: String = raw_command_name.to_ascii_lowercase();
      for command in &mut self.commands {
        // println!("{}\tcompared to\t{}", command_name, command.get_name());
        if command_name == command.get_name() {
          command.execute(arguments.split_whitespace());
          continue 'main_loop;
        }
      }
      match command_name.as_str() {
        "stop" => {
          println!("terminal stoped");
          break 'main_loop;
        },
        _ => {
          println!("\"{}\" is not a known command", command_name);
        },
      }
    }
    Ok(())
  }
}

pub fn main() -> Result<(), Error> {
  let mut terminal = Terminal::new();

  terminal.register(Box::new(Ping {}));
  terminal.register(Box::new(Times { count: 0 }));
  terminal.register(Box::new(Count {}));

  terminal.run()?;

  Ok(())
}
