use std::io::{self, Write};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser_files/code.pest"] // relative to src
pub struct MyParser;


#[derive(Debug, Default)]
struct MinishellCommand {
    command: String,
    arguments: Vec<String>,
}

impl MinishellCommand {
    
    fn new(command: String) -> Self{
        Self { command: command, arguments: Vec::new() }
    }

    fn add_argument(&mut self,argument: String){
        self.arguments.push(argument);
    }

    fn to_string(&self) -> String{
        let mut res = String::new();

        res.push_str(format!("The name of the command is: {}\n", self.command).as_str());

        let mut num_argum = 1;
        for argument in &self.arguments{
            res.push_str(format!("Arg{num_argum}: {argument}\n").as_str());
            num_argum+=1;
        }

        return res;
    }

}


fn main() -> Result<(), std::io::Error> {
    

    loop {
        let mut commands = String::new();
        let mut argvv: Vec<String> = Vec::new();
        let mut allMinishellCommand: Vec<MinishellCommand> = Vec::new();

        print!("msh>");
        std::io::stdout().flush()?;

        let _input = io::stdin().read_line(&mut commands).unwrap();

        let tokens =
            MyParser::parse(Rule::command_line, &commands).unwrap_or_else(|e| panic!("{}", e));

        for token in tokens {
            println!("{:?}", token.as_rule());
            println!("{:?}", token.as_str());

            let mut single_command: MinishellCommand = MinishellCommand {..Default::default()};

            
            if token.as_rule() == Rule::command{
                let single_command= MinishellCommand::new(token.as_str().to_string()); 
                allMinishellCommand.push(single_command);
            }else {
                // single_command.add_argument(token.as_str().to_string());
                match allMinishellCommand.pop(){
                    Some(mut minishell_command) => {
                        minishell_command.add_argument(token.as_str().to_string());
                        allMinishellCommand.push(minishell_command);
                    },
                    None => (),
                }
            }

            println!("{:?}", argvv);
        }

        for minishell_command in allMinishellCommand{
            println!("{}", minishell_command.to_string())
        }
    }
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
