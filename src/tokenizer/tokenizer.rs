use std::str::Chars;

use crate::tokenizer::{preprocess_token::PreprocessingToken, token::{Constant, Keyword, NumericalSuffix, Punctuator, Token}, TokenizationIssue};

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

    fn next_number_constant(&mut self, char: char) -> Option<Constant> {
        match char {
            '0' => {
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
                            return Some(Constant::Integer(
                                u64::from_str_radix(parts.get(0)?, 16)
                                    .expect("This should be a valid integer. Report this."),
                            ));
                        };
                    }
                    _ => {
                        return Some(Constant::Integer(
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
					return Some(Constant::Integer(u64::from_str_radix(&format!("{}{}", first, digits?), 10).expect("This should be a valid integer. Report this."))); // TODO: Suffix
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
			"auto" => Token::Keyword(Keyword::Auto),
			"break" => Token::Keyword(Keyword::Break),
			"case" => Token::Keyword(Keyword::Case),
			"char" => Token::Keyword(Keyword::Char),
			"const" => Token::Keyword(Keyword::Const),
			"continue" => Token::Keyword(Keyword::Continue),
			"default" => Token::Keyword(Keyword::Default),
			"do" => Token::Keyword(Keyword::Do),
			"double" => Token::Keyword(Keyword::Double),
			"else" => Token::Keyword(Keyword::Else),
			"enum" => Token::Keyword(Keyword::Enum),
			"extern" => Token::Keyword(Keyword::Extern),
			"float" => Token::Keyword(Keyword::Float),
			"for" => Token::Keyword(Keyword::For),
			"goto" => Token::Keyword(Keyword::Goto),
			"if" => Token::Keyword(Keyword::If),
			"inline" => Token::Keyword(Keyword::Inline),
			"int" => Token::Keyword(Keyword::Int),
			"long" => Token::Keyword(Keyword::Long),
			"register" => Token::Keyword(Keyword::Register),
			"restrict" => Token::Keyword(Keyword::Restrict),
			"return" => Token::Keyword(Keyword::Return),
			"short" => Token::Keyword(Keyword::Short),
			"signed" => Token::Keyword(Keyword::Signed),
			"sizeof" => Token::Keyword(Keyword::Sizeof),
			"static" => Token::Keyword(Keyword::Static),
			"struct" => Token::Keyword(Keyword::Struct),
			"switch" => Token::Keyword(Keyword::Switch),
			"typedef" => Token::Keyword(Keyword::Typedef),
			"union" => Token::Keyword(Keyword::Union),
			"unsigned" => Token::Keyword(Keyword::Unsigned),
			"void" => Token::Keyword(Keyword::Void),
			"volatile" => Token::Keyword(Keyword::Volatile),
			"while" => Token::Keyword(Keyword::While),
			"_Bool" => Token::Keyword(Keyword::Bool),
			"_Complex" => Token::Keyword(Keyword::Complex),
			"_Imaginary" => Token::Keyword(Keyword::Imaginary),
			other => Token::Identifier(other.into())
		};
		Some(token)
	}

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(next_character) = self.next_character() {
            match next_character {
                '[' => Some(Token::Punctuator(Punctuator::LeftBracket)),
                ']' => Some(Token::Punctuator(Punctuator::RightBracket)),
                '(' => Some(Token::Punctuator(Punctuator::LeftParen)),
                ')' => Some(Token::Punctuator(Punctuator::RightParen)),
                '{' => Some(Token::Punctuator(Punctuator::LeftBrace)),
                '}' => Some(Token::Punctuator(Punctuator::RightBrace)),
                '.' => {
                    if self.peek(0) == Some('.') && self.peek(1) == Some('.') {
                        self.index += 2;
                        Some(Token::Punctuator(Punctuator::Ellipsis))
                    } else if let Some(next) = self.peek(0)
                        && next.is_ascii_digit()
                        && let Some(num) = self.next_number_constant(next)
                    {
                        Some(Token::Constant(num))
                    } else {
                        Some(Token::Punctuator(Punctuator::Dot))
                    }
                }
                '-' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('>') => Some(Token::Punctuator(Punctuator::Arrow)),
                        Some('-') => Some(Token::Punctuator(Punctuator::Decrement)),
                        Some('=') => Some(Token::Punctuator(Punctuator::MinusAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Minus))
                        }
                    }
                }
                '+' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('+') => Some(Token::Punctuator(Punctuator::Increment)),
                        Some('=') => Some(Token::Punctuator(Punctuator::PlusAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Plus))
                        }
                    }
                }
                '&' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('&') => Some(Token::Punctuator(Punctuator::BooleanAnd)),
                        Some('=') => Some(Token::Punctuator(Punctuator::BitwiseAndAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Ampersand))
                        }
                    }
                }
                '*' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('=') => Some(Token::Punctuator(Punctuator::AsteriskAssign)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Asterisk))
                        }
                    }
                }
                '~' => Some(Token::Punctuator(Punctuator::Tilde)),
                '!' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('=') => Some(Token::Punctuator(Punctuator::NotEquality)),
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Exclamation))
                        }
                    }
                }
                '/' => {
                    self.index += 1;
                    match self.peek(0) {
                        Some('=') => Some(Token::Punctuator(Punctuator::SlashAssign)),
						Some('/') => { self.skip_line(); None },
						Some('*') => { self.skip_comment(); None },
                        _ => {
                            self.index -= 1;
                            Some(Token::Punctuator(Punctuator::Slash))
                        }
                    }
                }
                '%' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::PercentAssign))
                    }
                    Some('>') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::RightBrace))
                    }
                    Some(':') => {
                        if self.peek(1) == Some('%') && self.peek(2) == Some(':') {
                            self.index += 3;
                            Some(Token::Punctuator(Punctuator::DoubleHash))
                        } else {
                            self.index += 1;
                            Some(Token::Punctuator(Punctuator::Hash))
                        }
                    }
                    _ => Some(Token::Punctuator(Punctuator::Percent)),
                },
                '>' => match self.peek(0) {
                    Some('>') => {
                        self.index += 1;
                        if self.peek(1) == Some('=') {
                            self.index += 1;
                            Some(Token::Punctuator(Punctuator::ShiftRightAssign))
                        } else {
                            Some(Token::Punctuator(Punctuator::ShiftRight))
                        }
                    }
                    Some('=') => Some(Token::Punctuator(Punctuator::GreaterEqual)),
                    _ => Some(Token::Punctuator(Punctuator::Greater)),
                },
                '<' => match self.peek(0) {
                    Some('<') => {
                        self.index += 1;
                        if self.peek(1) == Some('=') {
                            self.index += 1;
                            Some(Token::Punctuator(Punctuator::ShiftLeftAssign))
                        } else {
                            Some(Token::Punctuator(Punctuator::ShiftLeft))
                        }
                    }
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::LessEqual))
                    }
                    Some(':') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::LeftBracket))
                    }
                    Some('%') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::LeftBrace))
                    }
                    _ => Some(Token::Punctuator(Punctuator::Less)),
                },
                '=' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::Equality))
                    }
                    _ => Some(Token::Punctuator(Punctuator::Equal)),
                },
                '^' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::BitwiseXorAssign))
                    }
                    _ => Some(Token::Punctuator(Punctuator::Caret)),
                },
                '|' => match self.peek(0) {
                    Some('=') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::BitwiseOrAssign))
                    }
                    Some('|') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::BooleanOr))
                    }
                    _ => Some(Token::Punctuator(Punctuator::BitwiseOrAssign)),
                },
                '?' => Some(Token::Punctuator(Punctuator::Question)),
                ':' => match self.peek(0) {
                    Some('>') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::RightBracket))
                    }
                    _ => Some(Token::Punctuator(Punctuator::Colon)),
                },
                ';' => Some(Token::Punctuator(Punctuator::Semicolon)),
                '#' => match self.peek(0) {
                    Some('#') => {
                        self.index += 1;
                        Some(Token::Punctuator(Punctuator::DoubleHash))
                    }
                    _ => Some(Token::Punctuator(Punctuator::Hash)),
                },
                ',' => Some(Token::Punctuator(Punctuator::Comma)),
                '"' => {
                    let str = self.next_string('"');
                    Some(Token::StringLiteral(str, false))
                }
                '\'' => {
                    let str = self.next_string('\'');
                    Some(Token::Constant(Constant::Character(str)))
                }
                c @ '0'..='9' => self.next_number_constant(c).map(|x| Token::Constant(x)),
				c @ '_' | c @ 'A'..='Z' | c @ 'a'..='z' => self.next_keyword_or_identifier(c),
                _ => None,
            }
        } else {
            None
        }
    }

	pub fn next_preprocessor_token(&mut self) -> Option<PreprocessingToken> {
		//match self.next_character() {
		//	Some('<') => self.next_string('>'),
		//};

		None
	}

    pub fn end_of_stream(&self) -> bool {
        self.index >= self.source.len()
    }

	pub fn issues(&self) -> &Vec<(usize, TokenizationIssue)> {
		&self.emitted_issues
	}
}

//fn preprocess_tokenize(source: &str) -> Vec<PreprocessingToken> {}
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut out_vec = Vec::new();
    let mut tokenizer = Tokenizer::new(source);

    println!("Source: \n{}", source);
	println!("Issues: {:?}", tokenizer.issues());

    while !tokenizer.end_of_stream() {
        if let Some(token) = tokenizer.next_token() {
            out_vec.push(token);
        }
    }

    out_vec
}

//fn process_tokens() {}
