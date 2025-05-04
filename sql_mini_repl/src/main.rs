use rustyline::{error::ReadlineError, DefaultEditor};
use sql_mini_execution::Execution;
use sql_mini_parser::{ast::SqlQuery, parse::Parse};

const HISTORY_FILE: &str = "history.txt";
fn main() -> Result<(), ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let mut exec = Execution::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match SqlQuery::parse_from_raw(line.as_ref()) {
                    Ok((_, query)) => {
                        let res = exec.run(query);
                        match res {
                            Ok(exec_res) => println!("{exec_res}"),
                            Err(e) => {
                                eprintln!("{e:?}");
                            }
                        }
                    }
                    //TODO: better errors
                    Err(e) => eprintln!("{e:?}"),
                }
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILE);
    Ok(())
}
