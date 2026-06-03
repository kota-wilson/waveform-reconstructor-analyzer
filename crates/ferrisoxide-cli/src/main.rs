use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match ferrisoxide_workflow::run(env::args().skip(1).collect()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
