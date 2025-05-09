use rustyline::{error::ReadlineError, DefaultEditor, Result};
use sql_mini_execution::{ExecResponse, Execution};
use sql_mini_parser::{ast::SqlQuery, parse::Parse};
use tabled::builder::Builder;

const HISTORY_FILE: &str = "history.txt";
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let mut exec = Execution::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match SqlQuery::parse_from_raw(line.as_ref()) {
                    Ok((_, query)) => {
                        let res = exec.run(query);
                        match res {
                            Ok(exec_res) => display_response(exec_res),
                            Err(e) => {
                                eprintln!("{e:?}");
                            }
                        }
                    }
                    //TODO: better errors
                    Err(e) => eprintln!("{e:?}"),
                }
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

    let _ = rl.save_history(HISTORY_FILE);
    Ok(())
}

fn display_response(response: ExecResponse) {
    match response {
        ExecResponse::Select(table_iter) => {
            let mut builder = Builder::default();
            let column_names: Vec<String> = table_iter
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect();

            builder.push_record(column_names.iter());

            table_iter.for_each(|row| {
                builder.push_record(column_names.iter().filter_map(|col_name| {
                    row.get(&col_name).map(|col_val| col_val.to_string()).ok()
                }))
            });

            println!("{}", builder.build());
        }
        _ => println!("{response}"),
    }
}
