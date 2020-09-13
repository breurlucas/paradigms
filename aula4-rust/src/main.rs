struct Cli {
    command: String,
    path: std::path::PathBuf,
}

fn main() {
  let command = std::env::args().nth(1).expect("no command given");
  let path = std::env::args().nth(2).expect("no path given"); 

  let args = Cli {
    command: command,
    path: std::path::PathBuf::from(path),
  };

  let content = std::fs::read_to_string(&args.path)
    .expect("could not read file");

  // * Counter *
  // Enter the '-l' command in order to retrieve the line count
  // Enter the '-w' command in order to retrieve the word count
  // Enter the '-lw' command in order to retrieve the line and word count

  let mut line_count: i8 = 0;
  let mut word_count: i8 = 0;
  let cmd = &args.command;

  for line in content.lines() {
    line_count += 1;

    if cmd == "-w" || cmd == "-lw" {
        let words: Vec<&str> = line.split(' ').collect();
        for word in words {
            word_count += 1;
        };
    }
  };

  if cmd != "-w" {
      println!("{}", line_count);
  }
  
  if cmd == "-w" || cmd == "-lw" {
      println!("{}", word_count);
  }

}