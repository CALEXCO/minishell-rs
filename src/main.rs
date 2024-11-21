use std::io::{self, Write};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser_files/code.pest"] // relative to src
pub struct MyParser;
fn main() -> Result<(), std::io::Error> {
    let mut commands = String::new();

    let mut argvv: Vec<String> = Vec::new();

    loop {
        print!("msh>");
        std::io::stdout().flush()?;

        let _input = io::stdin().read_line(&mut commands).unwrap();

        let tokens =
            MyParser::parse(Rule::command_line, &commands).unwrap_or_else(|e| panic!("{}", e));

        for token in tokens {
            println!("{:?}", token.as_rule());
            println!("{:?}", token.as_str());

            argvv.push(token.as_str().to_string());

            println!("{:?}", argvv);
        }

        commands.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_command() {
        let input = "ls -la | grep test";
        assert!(MyParser::parse(Rule::command_line, input).is_ok());
    }

    #[test]
    fn test_command_with_logical_operator() {
        let input = "cd arg && ls";
        assert!(MyParser::parse(Rule::command_line, input).is_err());
    }

    #[test]
    fn test_command_with_redirection() {
        let input = "echo hello > output.txt";
        assert!(MyParser::parse(Rule::command_line, input).is_err());
    }

    #[test]
    fn test_command_with_options_and_arguments() {
        let input = "rm -rf folder_name";
        assert!(MyParser::parse(Rule::command_line, input).is_ok());
    }

    #[test]
    fn test_invalid_syntax() {
        let input = "cp sourcefile||destinationfile";
        assert!(MyParser::parse(Rule::command_line, input).is_err());
    }
}
