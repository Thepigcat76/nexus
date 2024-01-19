use std::env;

use clutils::errors::FileHandlerError;

fn main() -> Result<(), FileHandlerError> {
    let args: Vec<String> = env::args().collect();
    let first_arg = args.get(1)
        .expect(
            "No argument supplied for file path. The standard nexus frontend requires one argument as the file path for execution"
        );
    nexus_api::execute_program(first_arg)?;
    Ok(())
}