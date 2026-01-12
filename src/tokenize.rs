use std::str::Chars;

/// Keywords, as defined in Section 6.4.1 of the C99 standard draft (N1256, page 50)
#[derive(Debug)]
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
#[derive(Debug)]
pub enum TokenConstant {
    /// Decimal constant (`[1-9][0-9]*`), octal constant (`0[0-7]*`) or hexadecimal constant (`0x|X[0-9a-fA-F]+`)
    /// followed by an optional suffix (`(([uU][lL]{1,2})|([lL]{1,2}[uU]))+`)
    Integer(u64),
    /// Decimal floating constant (`(([0-9]*\.[0-9]+([eE][0-9]+)?)|([0-9]+[eE][0-9]+))[fFlL]`), or
    /// Hexadecimal floating constant
    /// (`0x(([0-9A-Fa-f]*\.[0-9A-Fa-f]+)|([0-9A-Fa-f]*\.))p[\+\-]?[0-9A-Fa-f]*[flFL]?`)
    Floating(String),
    /// Enumeration constant (identifier)
    Enumeration(String),
    /// Character constant (`L?'([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+'`)
    Character(Vec<u8>),
}

/// Punctuators, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
#[derive(Debug)]
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

#[derive(Debug)]
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
    StringLiteral(Vec<u8>, bool),

    /// Punctuator, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
    Punctuator(TokenPunctuator),
}

#[derive(Debug)]
pub enum PPNumberToken {
    /// Decimal constant (`[1-9][0-9]*`), octal constant (`0[0-7]*`) or hexadecimal constant (`0x|X[0-9a-fA-F]+`)
    /// followed by an optional suffix (`(([uU][lL]{1,2})|([lL]{1,2}[uU]))+`)
    Integer(String),
    /// Decimal floating constant (`(([0-9]*\.[0-9]+([eE][0-9]+)?)|([0-9]+[eE][0-9]+))[fFlL]`), or
    /// Hexadecimal floating constant
    /// (`0x(([0-9A-Fa-f]*\.[0-9A-Fa-f]+)|([0-9A-Fa-f]*\.))p[\+\-]?[0-9A-Fa-f]*[flFL]?`)
    Floating(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
enum TokenizationIssue {
    UnknownEscapeSequence(String),
    UnterminatedString,
    OutOfRange,
    MissingHexadecimalDigits,
    HexadecimalFloatWithoutExponent,
}

#[derive(Debug)]
enum NumericalSuffix {
    Unsigned,
    Long,
    LongLong,
}

#[derive(Debug)]
struct Tokenizer {
    source: Vec<char>,
    index: usize,
    emitted_issues: Vec<(usize, TokenizationIssue)>,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        let source = source.replace("\r\n", "\n").replace("\\\n", "");
        Self {
            source: source.chars().collect(),
            index: 0,
            emitted_issues: Vec::new(),
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

    /// Iterates to the next character
    fn next_character(&mut self) -> Option<char> {
        let char = self.source.get(self.index).map(|x| *x);
        self.index += 1;
        char
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

    fn next_hex_digit_sequence(&mut self) -> Option<String> {
        let mut sequence = String::new();
        while let Some(next_character) = self.next_character() {
            match next_character {
                digit @ 'a'..='f' | digit @ 'A'..='F' | digit @ '0'..='9' => sequence.push(digit),
                _ => {
                    self.index -= 1;
                    break;
                }
            };
        }
        if sequence.is_empty() {
            None
        } else {
            Some(sequence)
        }
    }

    fn next_oct_digit_sequence(&mut self) -> Option<String> {
        let mut sequence = String::new();
        while let Some(next_character) = self.next_character() {
            match next_character {
                digit @ '0'..='7' => sequence.push(digit),
                _ => {
                    self.index -= 1;
                    break;
                }
            };
        }
        if sequence.is_empty() {
            None
        } else {
            Some(sequence)
        }
    }

    fn next_dec_digit_sequence(&mut self) -> Option<String> {
        let mut sequence = String::new();
        while let Some(next_character) = self.next_character() {
            match next_character {
                digit @ '0'..='9' => sequence.push(digit),
                _ => {
                    self.index -= 1;
                    break;
                }
            };
        }
        if sequence.is_empty() {
            None
        } else {
            Some(sequence)
        }
    }

    fn next_num_suffix(&mut self) -> Option<NumericalSuffix> {
        if let Some(next_character) = self.next_character() {
            match next_character {
                suffix @ 'l' | suffix @ 'L' => {
                    if let Some(next_suffix) = self.peek(0)
                        && next_suffix == suffix
                    {
                        self.index += 1;
                        return Some(NumericalSuffix::LongLong);
                    }
                    return Some(NumericalSuffix::Long);
                }
                'u' | 'U' => {
                    return Some(NumericalSuffix::Unsigned);
                }
                _ => {
                    self.index -= 1;
                }
            }
        }
        return None;
    }

    fn next_string(&mut self, end: char) -> Vec<u8> {
        let mut string: Vec<u8> = Vec::new();

        while let Some(next_character) = self.next_character() {
            match next_character {
                '\\' => match self.peek(0) {
                    Some('\'') => {
                        self.index += 1;
                        string.push('\'' as u8);
                    }
                    Some('"') => {
                        self.index += 1;
                        string.push('"' as u8);
                    }
                    Some('?') => {
                        self.index += 1;
                        string.push('?' as u8);
                    }
                    Some('\\') => {
                        self.index += 1;
                        string.push('\\' as u8);
                    }
                    Some('a') => {
                        self.index += 1;
                        string.push(0x07);
                    }
                    Some('b') => {
                        self.index += 1;
                        string.push(0x08);
                    }
                    Some('f') => {
                        self.index += 1;
                        string.push(0x0c);
                    }
                    Some('n') => {
                        self.index += 1;
                        string.push('\n' as u8);
                    }
                    Some('r') => {
                        self.index += 1;
                        string.push('\r' as u8);
                    }
                    Some('t') => {
                        self.index += 1;
                        string.push('\t' as u8);
                    }
                    Some('v') => {
                        self.index += 1;
                        string.push(0x0b);
                    }
                    Some(digit1 @ '0'..'7') => match self.peek(0) {
                        // TODO: bounds checking
                        Some(digit2 @ '0'..'7') => {
                            self.index += 1;
                            match self.peek(1) {
                                Some(digit3 @ '0'..'7') => {
                                    self.index += 1;
                                    if let Ok(char) =
                                        u8::from_str_radix(&format!("{digit1}{digit2}{digit3}"), 8)
                                    {
                                        string.push(char);
                                    } else {
                                        self.emitted_issues
                                            .push((self.index, TokenizationIssue::OutOfRange))
                                    }
                                }
                                _ => {
                                    if let Ok(char) =
                                        u8::from_str_radix(&format!("{digit1}{digit2}"), 8)
                                    {
                                        string.push(char);
                                    } else {
                                        self.emitted_issues
                                            .push((self.index, TokenizationIssue::OutOfRange))
                                    }
                                }
                            }
                        }
                        _ => {
                            if let Ok(char) = u8::from_str_radix(&format!("{digit1}"), 8) {
                                string.push(char);
                            } else {
                                self.emitted_issues
                                    .push((self.index, TokenizationIssue::OutOfRange))
                            }
                        }
                    },
                    Some('x') => {
                        if let Some(sequence) = self.next_hex_digit_sequence() {
                            if sequence.len() > 2 {
                                self.emitted_issues
                                    .push((self.index, TokenizationIssue::OutOfRange));
                            } else {
                                string.push(u8::from_str_radix(&sequence, 16).unwrap());
                            }
                        } else {
                            self.emitted_issues
                                .push((self.index, TokenizationIssue::MissingHexadecimalDigits));
                        }
                    }
                    Some(other) => {
                        self.emitted_issues.push((
                            self.index - 1,
                            TokenizationIssue::UnknownEscapeSequence(format!("\\{}", other)),
                        ));
                        let mut dst_buf = [0; 4];
                        string.extend_from_slice(other.encode_utf8(&mut dst_buf).as_bytes());
                    }
                    None => {
                        self.emitted_issues
                            .push((self.index - 1, TokenizationIssue::UnterminatedString));
                        break;
                    }
                },
                a if a == end => break,
                other => {
                    let mut dst_buf = [0; 4];
                    string.extend_from_slice(other.encode_utf8(&mut dst_buf).as_bytes());
                }
            };
        }
        return string;
    }

    fn next_number_constant(&mut self, char: char) -> Option<TokenConstant> {
        let mut characters_peeked = 0;
        match char {
            '0' => {
                characters_peeked += 1;
                match self.peek(0) {
                    Some('x') | Some('X') => {
                        let mut is_float = false;
                        let mut has_whole_part = true;
                        let mut has_fractional_part = false;
                        let mut exponent_part_is_negative = false;
                        let mut parts = Vec::new();

                        self.index += 1;

                        // Consume the '.', if any, from the start of the number
                        if self.peek(0) == Some('.') {
                            has_whole_part = false;
                            has_fractional_part = true;
                            is_float = true;
                            self.index += 1;
                        }

                        // Consume the whole part, or fractional part if the number has no whole
                        // part
                        parts.push(self.next_hex_digit_sequence()?);

                        // Consume the '.' after a digit sequence
                        if self.peek(0) == Some('.') && has_whole_part {
                            is_float = true;
                            self.index += 1;
                        }

                        // If there are digits after the '.', consume them
                        if let Some(next) = self.peek(0)
                            && next.is_ascii_hexdigit()
                        {
                            has_fractional_part = true;
                            parts.push(self.next_hex_digit_sequence()?);
                        }

                        if is_float {
                            // Consume exponent delimiter
                            match self.peek(0) {
                                Some('p') | Some('P') => self.index += 1,
                                _ => {
                                    self.emitted_issues.push((
                                        self.index,
                                        TokenizationIssue::HexadecimalFloatWithoutExponent,
                                    ));
                                    self.index += 1;
                                    return None;
                                }
                            };
                            // Consume sign of exponent
                            match self.peek(0) {
                                Some('+') => self.index += 1,
                                Some('-') => {
                                    self.index += 1;
                                    exponent_part_is_negative = true;
                                }
                                _ => {}
                            };

                            // Consume exponent
                            if let Some(exponent) = self.next_dec_digit_sequence() {
                                parts.push(exponent);
                            } else {
                                self.emitted_issues.push((
                                    self.index,
                                    TokenizationIssue::HexadecimalFloatWithoutExponent,
                                ));
                                return None;
                            };
                        }

                        // Construct float/int
                        if is_float {
							return None; // TODO: Implement
                        } else {
                            return Some(TokenConstant::Integer(
                                u64::from_str_radix(parts.get(0)?, 16)
                                    .expect("This should be a valid integer. Report this."),
                            ));
                        };
                    }
                    _ => {
                        return Some(TokenConstant::Integer(
                            u64::from_str_radix(
                                &self.next_oct_digit_sequence().unwrap_or("0".into()),
                                8,
                            ) // TODO: Suffix
                            .expect("This should be a valid integer. Report this."),
                        ));
                    }
                }
            }
            first @ '1'..='9' => {
                /* dec */
                let mut str = String::from(char);
                let digits = self.next_dec_digit_sequence();
                if self.peek(0) == Some('.')
                    || self.peek(0) == Some('e')
                    || self.peek(0) == Some('E')
                {
					return None; // TODO: implement
                    /* float_dec */
                } else {
					return Some(TokenConstant::Integer(u64::from_str_radix(&format!("{}{}", first, digits?), 10).expect("This should be a valid integer. Report this."))); // TODO: Suffix
                }
            }
            '.' => { return None; /* float_dec*/ } // TODO: implement
            _ => return None,
        };
    }

	fn next_keyword_or_identifier(&mut self, last_char: char) -> Option<Token> {
        let mut string = String::from(last_char);
        while let Some(current_char) = self.next_character()
            && (current_char.is_ascii_alphabetic() || current_char.is_ascii_digit() || current_char == '_')
        {
            string.push(current_char);
        }
        self.index -= 1;

		if string.is_empty() {
			return None;
		}

		let token = match string.as_str() {
			"auto" => Token::Keyword(TokenKeyword::Auto),
			"break" => Token::Keyword(TokenKeyword::Break),
			"case" => Token::Keyword(TokenKeyword::Case),
			"char" => Token::Keyword(TokenKeyword::Char),
			"const" => Token::Keyword(TokenKeyword::Const),
			"continue" => Token::Keyword(TokenKeyword::Continue),
			"default" => Token::Keyword(TokenKeyword::Default),
			"do" => Token::Keyword(TokenKeyword::Do),
			"double" => Token::Keyword(TokenKeyword::Double),
			"else" => Token::Keyword(TokenKeyword::Else),
			"enum" => Token::Keyword(TokenKeyword::Enum),
			"extern" => Token::Keyword(TokenKeyword::Extern),
			"float" => Token::Keyword(TokenKeyword::Float),
			"for" => Token::Keyword(TokenKeyword::For),
			"goto" => Token::Keyword(TokenKeyword::Goto),
			"if" => Token::Keyword(TokenKeyword::If),
			"inline" => Token::Keyword(TokenKeyword::Inline),
			"int" => Token::Keyword(TokenKeyword::Int),
			"long" => Token::Keyword(TokenKeyword::Long),
			"register" => Token::Keyword(TokenKeyword::Register),
			"restrict" => Token::Keyword(TokenKeyword::Restrict),
			"return" => Token::Keyword(TokenKeyword::Return),
			"short" => Token::Keyword(TokenKeyword::Short),
			"signed" => Token::Keyword(TokenKeyword::Signed),
			"sizeof" => Token::Keyword(TokenKeyword::Sizeof),
			"static" => Token::Keyword(TokenKeyword::Static),
			"struct" => Token::Keyword(TokenKeyword::Struct),
			"switch" => Token::Keyword(TokenKeyword::Switch),
			"typedef" => Token::Keyword(TokenKeyword::Typedef),
			"union" => Token::Keyword(TokenKeyword::Union),
			"unsigned" => Token::Keyword(TokenKeyword::Unsigned),
			"void" => Token::Keyword(TokenKeyword::Void),
			"volatile" => Token::Keyword(TokenKeyword::Volatile),
			"while" => Token::Keyword(TokenKeyword::While),
			"_Bool" => Token::Keyword(TokenKeyword::Bool),
			"_Complex" => Token::Keyword(TokenKeyword::Complex),
			"_Imaginary" => Token::Keyword(TokenKeyword::Imaginary),
			other => Token::Identifier(other.into())
		};
		Some(token)
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
                    if self.peek(0) == Some('.') && self.peek(1) == Some('.') {
                        self.index += 2;
                        Some(Token::Punctuator(TokenPunctuator::Ellipsis))
                    } else if let Some(next) = self.peek(0)
                        && next.is_ascii_digit()
                        && let Some(num) = self.next_number_constant(next)
                    {
                        Some(Token::Constant(num))
                    } else {
                        Some(Token::Punctuator(TokenPunctuator::Dot))
                    }
                }
                '-' => {
                    self.index += 1;
                    match self.peek(0) {
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
                    match self.peek(0) {
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
                    match self.peek(0) {
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
                    match self.peek(0) {
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
                    match self.peek(0) {
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::NotEquality)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Exclamation))
                        }
                    }
                }
                '/' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('=') => Some(Token::Punctuator(TokenPunctuator::SlashAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(TokenPunctuator::Slash))
                        }
                    }
                }
                '%' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::PercentAssign))
                    }
                    Some('>') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::RightBrace))
                    }
                    Some(':') => {
                        if self.peek(1) == Some('%') && self.peek(2) == Some(':') {
                            self.index += 3;
                            Some(Token::Punctuator(TokenPunctuator::DoubleHash))
                        } else {
                            self.index += 1;
                            Some(Token::Punctuator(TokenPunctuator::Hash))
                        }
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Percent)),
                },
                '>' => match self.peek(0) {
                    Some('>') => {
                        self.index += 1;
                        if self.peek(1) == Some('=') {
                            self.index += 1;
                            Some(Token::Punctuator(TokenPunctuator::ShiftRightAssign))
                        } else {
                            Some(Token::Punctuator(TokenPunctuator::ShiftRight))
                        }
                    }
                    Some('=') => Some(Token::Punctuator(TokenPunctuator::GreaterEqual)),
                    _ => Some(Token::Punctuator(TokenPunctuator::Greater)),
                },
                '<' => match self.peek(0) {
                    Some('<') => {
                        self.index += 1;
                        if self.peek(1) == Some('=') {
                            self.index += 1;
                            Some(Token::Punctuator(TokenPunctuator::ShiftLeftAssign))
                        } else {
                            Some(Token::Punctuator(TokenPunctuator::ShiftLeft))
                        }
                    }
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::LessEqual))
                    }
                    Some(':') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::LeftBracket))
                    }
                    Some('%') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::LeftBrace))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Less)),
                },
                '=' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::Equality))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Equal)),
                },
                '^' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::BitwiseXorAssign))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Caret)),
                },
                '|' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::BitwiseOrAssign))
                    }
                    Some('|') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::BooleanOr))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::BitwiseOrAssign)),
                },
                '?' => Some(Token::Punctuator(TokenPunctuator::Question)),
                ':' => match self.peek(0) {
                    Some('>') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::RightBracket))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Colon)),
                },
                ';' => Some(Token::Punctuator(TokenPunctuator::Semicolon)),
                '#' => match self.peek(0) {
                    Some('#') => {
                        self.index += 1;
                        Some(Token::Punctuator(TokenPunctuator::DoubleHash))
                    }
                    _ => Some(Token::Punctuator(TokenPunctuator::Hash)),
                },
                ',' => Some(Token::Punctuator(TokenPunctuator::Comma)),
                '"' => {
                    let str = self.next_string('"');
                    Some(Token::StringLiteral(str, false))
                }
                '\'' => {
                    let str = self.next_string('\'');
                    Some(Token::Constant(TokenConstant::Character(str)))
                }
                c @ '0'..='9' | c @ '.' => self.next_number_constant(c).map(|x| Token::Constant(x)),
				c @ '_' | c @ 'A'..='Z' | c @ 'a'..='z' => self.next_keyword_or_identifier(c),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn end_of_stream(&self) -> bool {
        self.index >= self.source.len()
    }
    // pub fn next_preprocessing_token() -> PreprocessingToken {}
}

//fn preprocess_tokenize(source: &str) -> Vec<PreprocessingToken> {}
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut out_vec = Vec::new();
    let mut tokenizer = Tokenizer::new(source);

    println!("Source: \n{}", source);

    while !tokenizer.end_of_stream() {
        if let Some(token) = tokenizer.next_token() {
            out_vec.push(token);
        }
    }

    out_vec
}

//fn process_tokens() {}
