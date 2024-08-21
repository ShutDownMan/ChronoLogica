use tracing::debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(lexeme: String, kind: TokenKind) -> Token {
        Token { lexeme, kind }
    }

    pub fn to_string(&self) -> String {
        // print lexeme escaped
        let escaped_lexeme = self.lexeme.replace("\n", "\\n").replace("\t", "\\t");
        format!("Token {{ lexeme: \"{}\", kind: {:?} }}", escaped_lexeme, self.kind)
    }
}

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum TokenKind {
    At,
    Locf,
    Lerp,
    Within,
    Mirroring,

    UnixTimestamp,
    ISO8601Timestamp,
    TimeInterval,

    Identifier,
    Number,

    SingleLineString,
    MultiLineString,

    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,

    SingleLineComment,
    MultiLineComment,

    Comma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Power,
    Semicolon,
    Assign,
    WhiteSpace,
    Unmatched,
}

pub fn get_token_kind_pattern(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::At => r"at|@",
        TokenKind::Locf => r"locf",
        TokenKind::Lerp => r"lerp",
        TokenKind::Within => r"within",
        TokenKind::Mirroring => r"mirroring",

        TokenKind::UnixTimestamp => r"ut\d+",
        TokenKind::ISO8601Timestamp => r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}-\d{2}:\d{2}",
        TokenKind::TimeInterval => r"\d+[smhd]",

        TokenKind::Identifier => r"[a-zA-Z0-9_]+",
        TokenKind::Number => r"\d+(\.\d+)?([eE][+-]?\d+)?",

        TokenKind::SingleLineString => r#""[^"]*""#,
        TokenKind::MultiLineString => r#""""[^"]*""""#,
        TokenKind::OpenBracket => r"\[",
        TokenKind::CloseBracket => r"\]",
        TokenKind::OpenParen => r"\(",
        TokenKind::CloseParen => r"\)",

        TokenKind::SingleLineComment => r"//.*",
        TokenKind::MultiLineComment => r"/\*.*?\*/",

        TokenKind::Comma => r",",
        TokenKind::Plus => r"\+",
        TokenKind::Minus => r"\-",
        TokenKind::Asterisk => r"\*",
        TokenKind::Slash => r"\/",
        TokenKind::Power => r"\*\*",
        TokenKind::Semicolon => r";",
        TokenKind::Assign => r"=",
        TokenKind::WhiteSpace => r"\s+",
        TokenKind::Unmatched => r".",
    }
}

pub fn get_next_token(program: &str) -> Result<Option<(Token, &str)>> {
    debug!("Getting next token...");
    let mut chosen_match: Option<(TokenKind, regex::Match)> = None;
    for kind in TokenKind::iter() {
        let re = Regex::new(&format!("^{}", get_token_kind_pattern(&kind)))?;
        let matches = re.find(program);
        if matches.is_some() {
            let m = matches.unwrap();
            let chosen_match_end = match chosen_match {
                None => 0,
                Some((_, m)) => m.end(),
            };
            if chosen_match.is_none() || m.end() > chosen_match_end {
                chosen_match = Some((kind, m));
            }
        }
    }

    match chosen_match {
        None => return Ok(None),
        Some(_) => {
            let (kind, m) = chosen_match.unwrap();
            let lexeme = m.as_str().to_string();
            let token = Token::new(lexeme, kind.clone());
            let remaining_program = &program[m.end()..];

            Ok(Some((token, remaining_program)))
        }
    }
}