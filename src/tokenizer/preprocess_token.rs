use crate::tokenizer::token::Punctuator;

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
    Punctuator(Punctuator),

    /// As defined in Section 6.4 of the C99 standard draft (N1256, page 49), preprocessing tokens
    /// include "each non-white-space character that cannot be one of the above"
    NonWhitespace,
}
