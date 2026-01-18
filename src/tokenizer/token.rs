/// Keywords, as defined in Section 6.4.1 of the C99 standard draft (N1256, page 50)
#[derive(Debug)]
pub enum Keyword {
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
pub enum Constant {
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
pub enum Punctuator {
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
    Keyword(Keyword),

    /// Identifier, as defined in Section 6.4.2 of the C99 standard draft (N1256, page 51)
    /// Non-digit (`[A-Za-z_]`) followed by non-digit or digit (`[A-Za-z_0-9]`)
    Identifier(String),

    /// Constant, as defined in Section 6.4.4 of the C99 standard draft (N1256, page 54)
    Constant(Constant),

    /// String literal, as defined in Section 6.4.5 of the C99 standard draft (N1256, page 62)
    /// The `bool` value is true if the string literal is a wide string literal
    /// Defined as `L?"([^'\\\n]|\\['"?\\abfnrtv]|\\[0-7]{1,3}|\x[0-9A-Fa-f]+)+"`
    StringLiteral(Vec<u8>, bool),

    /// Punctuator, as defined in Section 6.4.6 of the C99 standard draft (N1256, page 63)
    Punctuator(Punctuator),
}

#[derive(Debug)]
pub enum NumericalSuffix {
    Unsigned,
    Long,
    LongLong,
}
