use tracing::{error, info, debug};
use anyhow::Result;

use super::token::Token;

use crate::lex::token::get_next_token;

pub fn tokenize(program: &str) -> Result<Vec<Token>> {
    info!("Tokenizing program...");
    let mut tokens = Vec::new();
    let mut rem_program = program;

    debug!("Program: {}", rem_program);

    // tokenize using get_next_token
    while let Some((token, rest)) = get_next_token(rem_program)? {
        debug!("Token: {:?}", token.to_string());

        match check_token(&token, &program, &rest) {
            Ok(()) => {
                tokens.push(token);
            },
            Err(e) => {
                error!("Error tokenizing program: {}", e);
            }
        }
        rem_program = rest;
    }

    Ok(tokens)
}


fn check_token(token: &Token, program: &str, rest: &str) -> Result<()> {
    let current_line = program.lines().count() - rest.lines().count();
    let current_column = program.len() - rest.len() - program[..program.len() - rest.len()].rfind("\n").unwrap_or(0);
    match token.kind {
        super::token::TokenKind::Unmatched => {
            error!("Unknown token at line {} column {}: {}", current_line, current_column, token.lexeme);
            Err(anyhow::anyhow!("Unknown token"))
        },
        super::token::TokenKind::SingleLineString => {
            // check if string contains newline
            if token.lexeme.contains("\n") {
                error!("Single line string at line {} column {} contains newline", current_line, current_column);
                return Err(anyhow::anyhow!("Single line string contains newline"));
            }

            // check if string is closed
            if !token.lexeme.ends_with("\"") {
                error!("Single line string at line {} column {} is not closed", current_line, current_column);
                return Err(anyhow::anyhow!("Single line string is not closed"))
            }

            // check for invalid escape sequences
            let valid_escape_sequences = vec!["\\n", "\\t", "\\r", "\\\"", "\\\\"];
            let escape_sequences = token.lexeme.matches("\\");
            for escape_sequence in escape_sequences {
                if !valid_escape_sequences.contains(&escape_sequence) {
                    error!("Invalid escape sequence at line {} column {}: {}", current_line, current_column, escape_sequence);
                    return Err(anyhow::anyhow!("Invalid escape sequence"));
                }
            }

            Ok(())
        },
        super::token::TokenKind::MultiLineString => {
            // check if string is closed
            if !token.lexeme.ends_with("\"\"\"") {
                error!("Multi line string at line {} column {} is not closed", current_line, current_column);
                return Err(anyhow::anyhow!("Multi line string is not closed"))
            }

            // check for invalid escape sequences
            let valid_escape_sequences = vec!["\\n", "\\t", "\\r", "\\\"", "\\\\"];
            let escape_sequences = token.lexeme.matches("\\");
            for escape_sequence in escape_sequences {
                if !valid_escape_sequences.contains(&escape_sequence) {
                    error!("Invalid escape sequence at line {} column {}: {}", current_line, current_column, escape_sequence);
                    return Err(anyhow::anyhow!("Invalid escape sequence"));
                }
            }

            Ok(())
        },
        super::token::TokenKind::MultiLineComment => {
            // check if comment is closed
            if !token.lexeme.ends_with("*/") {
                error!("Multi line comment at line {} column {} is not closed", current_line, current_column);
                return Err(anyhow::anyhow!("Multi line comment is not closed"))
            }

            Ok(())
        },
        super::token::TokenKind::Identifier => {
            // check if identifier starts with a number
            if token.lexeme.chars().next().unwrap().is_numeric() {
                error!("Identifier '{}' at line {} column {} starts with a number", token.lexeme, current_line, current_column);
                return Err(anyhow::anyhow!("Identifier starts with a number"));
            }

            Ok(())
        },
        _ => Ok(())
    }
}