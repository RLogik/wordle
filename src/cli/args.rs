// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate argparse;

use self::argparse::ArgumentParser;
use self::argparse::StoreTrue;
use self::argparse::Store;

use crate::models::cli::CmdArguments;

// ----------------------------------------------------------------
// Argument Parser
// ----------------------------------------------------------------

pub fn construct_arg_parser() -> CmdArguments {
    let mut quiet = false;
    let mut interactive = false;
    let mut path = "World".to_string();
    {  // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();
        parser.set_description("The wordle guessing aid.");
        parser.refer(&mut quiet)
            .add_option(
                &["-q", "--quiet"],
                StoreTrue,
                "Run in quiet mode."
            );
        parser.refer(&mut path)
            .add_option(
                &["-p", "--path"],
                Store,
                "Path to list of possible words."
            );
        parser.refer(&mut interactive)
            .add_option(
                &["--it"],
                StoreTrue,
                "Run in interactive mode."
            );
        match parser.parse_args() {
            Ok(()) => {

            },
            Err(code) => {
                let mut buf = Vec::<u8>::new();
                match parser.print_usage("Usage", &mut buf) {
                    Ok(()) => { },
                    Err(_code) => { },
                };
                match parser.print_help("Guide", &mut buf) {
                    Ok(()) => { },
                    Err(_code) => {
                        println!("Incorrect usage!");
                    },
                };
                std::process::exit(code);
            },
        }
    }
    return CmdArguments { quiet, path, interactive };
}
