use std::str::Chars;

/// Keywords, as defined in Section 6.4.1 of the C99 standard draft (N1256, page 50)
pub enum TokenKeyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    Bool,
    Complex,
    Imaginary,
}

/// Constants, as defined in Section 6.4.4 of the C99 standard draft (N1256, page 54)
pub enum TokenConstant {
    /// Decimal constant (`[1-9][0-9]*`), octal constant (`0[0-7]*`) or hexadecimal constant (`0x|X[0-9a-fA-F]+`)
    /// followed by an optional suffix (`(([uU][lL]{1,2})|([lL]{1,2}[uU]))+`)
    Integer(String),
    /// Decimal floating constant (`(([0-9]*\.[0-9]+([eE][0-9]+)?)|([0-9]+[eE][0-9]+))[fFlL]`), or
    /// Hexadecimal floating constant
    /// (`0x(([0-9A-Fa-f]*\.[0-9A-Fa-f]+)|([0-9A-Fa-f]*\.))p[\+\-]?[0-9A-Fa-f]*[flFL]?`)
    Floating(String),
    /// Enumeration constant (identifier)
    Enumeration(String),
    /// Character constant (`L?'([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+'`)
    Character(String),
}

/// Punctuators, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
pub enum TokenPunctuator {
    /// The `[` or `<:` symbol
    LeftBracket,
    /// The `]` or `:>` symbol
    RightBracket,
    /// The `(` symbol
    LeftParen,
    /// The `)` symbol
    RightParen,
    /// The `{` or `<% symbol
    LeftBrace,
    /// The `}` or `%>` symbol
    RightBrace,
    /// The `.` symbol
    Dot,
    /// The `->` symbol
    Arrow,

    /// The `++` symbol
    Increment,
    /// The `--` symbol
    Decrement,
    /// The `&` symbol
    Ampersand,
    /// The `*` symbol
    Asterisk,
    /// The `+` symbol
    Plus,
    /// The `-` symbol
    Minus,
    /// The `~` symbol
    Tilde,
    /// The `!` symbol
    Exclamation,

    /// The `/` symbol
    Slash,
    /// The `%` symbol
    Percent,
    /// The `<<` symbol
    ShiftLeft,
    /// The `>>` symbol
    ShiftRight,
    /// The `<` symbol
    Less,
    /// The `>` symbol
    Greater,
    /// The `<=` symbol
    LessEqual,
    /// The `>=` symbol
    GreaterEqual,
    /// The `==` symbol
    Equality,
    /// The `!=` symbol
    NotEquality,
    /// The `^` symbol
    Caret,
    /// The `|` symbol
    Pipe,
    /// The `&&` symbol
    BooleanAnd,
    /// The `||` symbol
    BooleanOr,

    /// The `?` symbol
    Question,
    /// The `:` symbol
    Colon,
    /// The `;` symbol
    Semicolon,
    /// The `...` symbol
    Ellipsis,

    /// The `=` symbol
    Equal,
    /// The `*=` symbol
    AsteriskAssign,
    /// The `/=` symbol
    SlashAssign,
    /// The `%=` symbol
    PercentAssign,
    /// The `+=` symbol
    PlusAssign,
    /// The `-=` symbol
    MinusAssign,
    /// The `<<=` symbol
    ShiftLeftAssign,
    /// The `>>=` symbol
    ShiftRightAssign,
    /// The `&=` symbol
    BitwiseAndAssign,
    /// The `^=` symbol
    BitwiseXorAssign,
    /// The `|=` symbol
    BitwiseOrAssign,

    /// The `,` symbol
    Comma,
    /// The `#` or `%:` symbol
    Hash,
    /// The `##` or `%:%:` symbol
    DoubleHash,
}

pub enum Token {
    /// Keyword, as defined in Section 6.4.1 of the C99 standard draft (N1256, page 50)
    Keyword(TokenKeyword),

    /// Identifier, as defined in Section 6.4.2 of the C99 standard draft (N1256, page 51)
    /// Non-digit (`[A-Za-z_]`) followed by non-digit or digit (`[A-Za-z_0-9]`)
    Identifier(String),

    /// Constant, as defined in Section 6.4.4 of the C99 standard draft (N1256, page 54)
    Constant(TokenConstant),

    /// String literal, as defined in Section 6.4.5 of the C99 standard draft (N1256, page 62)
    /// The `bool` value is true if the string literal is a wide string literal
    /// Defined as `L?"([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+"`
    StringLiteral(String, bool),

    /// Punctuator, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
    Punctuator(TokenPunctuator),
}

pub enum PPNumberToken {
    /// Decimal constant (`[1-9][0-9]*`), octal constant (`0[0-7]*`) or hexadecimal constant (`0x|X[0-9a-fA-F]+`)
    /// followed by an optional suffix (`(([uU][lL]{1,2})|([lL]{1,2}[uU]))+`)
    Integer(String),
    /// Decimal floating constant (`(([0-9]*\.[0-9]+([eE][0-9]+)?)|([0-9]+[eE][0-9]+))[fFlL]`), or
    /// Hexadecimal floating constant
    /// (`0x(([0-9A-Fa-f]*\.[0-9A-Fa-f]+)|([0-9A-Fa-f]*\.))p[\+\-]?[0-9A-Fa-f]*[flFL]?`)
    Floating(String),
}

pub enum PreprocessingToken {
    /// Header name, as defined in Section 6.4.7 of the C99 standard draft (N1256, page 64)
    /// Defined as `(<[^>\n]+>)|("[^"\n]+")`
    /// String includes the quotes and angle brackets
    HeaderName(String),

    /// Identifier, as defined in Section 6.4.2 of the C99 standard draft (N1256, page 51)
    /// Non-digit (`[A-Za-z_]`) followed by non-digit or digit (`[A-Za-z_0-9]`)
    Identifier(String),

    /// Preprocessing numbers, as defined in Section 6.4.8 of the C99 standard draft (N1256, page 65)
    PPNumber(PPNumberToken),

    /// Character constant (`L?'([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+'`)
    CharacterConstant(String),

    /// String literal, as defined in Section 6.4.5 of the C99 standard draft (N1256, page 62)
    /// The `bool` value is true if the string literal is a wide string literal
    /// Defined as `L?"([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+"`
    StringLiteral(String, bool),

    /// Punctuator, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
    Punctuator(TokenPunctuator),

    /// As defined in Section 6.4 of the C99 standard draft (N1256, page 49), preprocessing tokens
    /// include "each non-white-space character that cannot be one of the above"
    NonWhitespace,
}

struct Tokenizer {
    source: Vec<char>,
    index: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        let source = source.replace("\r\n", "\n").replace("\\\n", "");
        Self {
            source: source.chars().collect(),
            index: 0,
        }
    }

    /// Consumes until next whitespace
    fn skip_whitespace(&mut self) {
        while let Some(current_char) = self.next_character()
            && current_char.is_whitespace()
        {}
        self.index -= 1;
    }

    /// Consumes the remainder of the current line
    fn skip_line(&mut self) {
        while let Some(current_char) = self.next_character()
            && current_char != '\n'
        {}
        self.index -= 1;
    }

    /// Returns the next character
    fn peek(&self, offset: usize) -> Option<char> {
        self.source.get(self.index + offset).map(|x| *x)
    }

    /// Iterates to the next character
    fn next_character(&mut self) -> Option<char> {
        let char = self.source.get(self.index).map(|x| *x);
        self.index += 1;
        char
    }

    /// Consumes until the next `*/`
    fn skip_comment(&mut self) {
        while let Some(current_char) = self.next_character() {
            match current_char {
                '*' => {
                    if Some('/') == self.peek(1) {
                        self.index += 1;
                        break;
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };
        }
    }

    /// Consumes the next word (`[A-Za-z]`)
    fn next_word(&mut self) -> Option<String> {
        let mut string = String::new();
        while let Some(current_char) = self.next_character()
            && current_char.is_ascii_alphabetic()
        {
            string.push(current_char);
        }
        self.index -= 1;
        if string.len() != 0 {
            Some(string)
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(next_character) = self.next_character() {
            match next_character {
                '[' => Some(Token::Punctuator(TokenPunctuator::LeftBracket)),
                ']' => Some(Token::Punctuator(TokenPunctuator::RightBracket)),
                '(' => Some(Token::Punctuator(TokenPunctuator::LeftParen)),
                ')' => Some(Token::Punctuator(TokenPunctuator::RightParen)),
                '{' => Some(Token::Punctuator(TokenPunctuator::LeftBrace)),
                '}' => Some(Token::Punctuator(TokenPunctuator::RightBrace)),
                '.' => {
                    if self.peek(1) == Some('.') && self.peek(2) == Some('.') {
                        self.index += 2;
                        Some(Token::Punctuator(TokenPunctuator::Ellipsis))
                    } else {
                        Some(Token::Punctuator(TokenPunctuator::Dot))
                    }
                }
                '-' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('>') => Some(Token::Punctuator(TokenPunctuator::Arrow)),
                        Some('-') => Some(Token::Punctuator(TokenPunctuator::Decrement)),
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::MinusAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Minus))
                        }
                    }
                }
                '+' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('+') => Some(Token::Punctuator(TokenPunctuator::Increment)),
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::PlusAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Plus))
                        }
                    }
                }
                '&' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('&') => Some(Token::Punctuator(TokenPunctuator::BooleanAnd)),
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::BitwiseAndAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Ampersand))
                        }
                    }
                }
                '*' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::AsteriskAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Asterisk))
                        }
                    }
                }
                '~' => Some(Token::Punctuator(TokenPunctuator::Tilde)),
                '!' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::NotEquality)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Exclamation))
                        }
                    }
                }
                '/' => {
                    self.index += 1;
                    match self.peek(1) {
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::SlashAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Slash))
                        }
                    }
                }
                '%' => match self.peek(1) {},
            }
        } else {
            None
        }
    }
    pub fn next_preprocessing_token() -> PreprocessingToken {}
}

// todo
//
//
// % << >> < > <= >= == ^ | ||
// ? : ; ...
// = %= <<= >>= ^= |=
// , # ##
// <: :> <% %> %: %:%:

fn preprocess_tokenize(source: &str) -> Vec<PreprocessingToken> {}
fn tokenize(source: &str) -> Vec<Token> {}

fn process_tokens() {}
