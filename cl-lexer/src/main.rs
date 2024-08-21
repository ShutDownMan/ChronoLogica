use tracing::{error, info, debug};
use tracing_subscriber::EnvFilter;

mod lex;

use lex::lexer::tokenize;

fn main() {
    setup_logger();

    let mut user_input = String::new();
    println!("Digite o nome do arquivo fonte (Ex: fonte1.txt, fonte2.txt): ");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input = user_input.trim().to_string();

    if user_input.is_empty() {
        error!("Nome do arquivo fonte não pode ser vazio");
        return;
    }

    let program = match std::fs::read_to_string(&user_input) {
        Ok(program) => program,
        Err(e) => {
            error!("Erro ao ler arquivo fonte: {}", e);
            return;
        }
    };

    info!("Lexer for ChronoLogica");
    let tokens = match tokenize(&program) {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("Error tokenizing program: {}", e);
            return;
        }
    };

    debug!("Found {} tokens", tokens.len());
    for token in tokens.iter() {
        match token.kind {
            lex::token::TokenKind::WhiteSpace => {},
            _ => {
                println!("{}", token.to_string());
            }
        }
    }


}

fn setup_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
