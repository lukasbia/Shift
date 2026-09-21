use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Unknown,

    Identifier,
    EscapedIdentifier,

    IntegerLiteral,
    BinaryLiteral,
    OctalLiteral,
    HexLiteral,
    FloatLiteral,

    StringLiteral,
    RawStringLiteral,
    MultilineStringLiteral,
    CharacterLiteral,

    Directive,

    If,
    Else,
    Guard,
    Switch,
    Case,
    Default,
    While,
    For,
    In,
    Loop,
    Repeat,
    Break,
    Continue,
    Do,
    Then,

    True,
    False,
    Nil,

    Var,
    Let,
    Const,
    Static,
    Lazy,
    Defer,

    Int,
    Float,
    Double,
    Bool,
    String,
    Char,
    Bytes,
    Void,
    Any,
    Never,
    Num,

    Struct,
    Class,
    Enum,
    Protocol,
    Extension,
    TypeAlias,
    AssociatedType,

    Func,
    Init,
    Deinit,
    Return,

    SelfValue,
    Super,

    Import,
    Export,
    Public,
    Private,
    Internal,
    FilePrivate,
    Package,

    Get,
    Set,
    WillSet,
    DidSet,
    Subscript,

    Async,
    Await,
    Actor,
    Isolated,
    Nonisolated,

    Throws,
    Throw,
    Try,
    Catch,
    Finally,

    Where,
    Is,
    As,
    Some,

    Operator,
    Precedence,
    Associativity,

    Move,
    Copy,
    Borrow,
    Consume,
    Owned,
    Weak,
    Unowned,

    Unsafe,
    Safe,
    Mutating,
    Nonmutating,

    Final,
    Override,
    Required,
    Convenience,
    Open,
    Dynamic,
    InOut,
    Yield,
    Macro,

    Match,
    Panic,

    Output,
    Message,
    Call,
    Error,
    Sink,
    Range,
    Delete,
    Cut,
    End,
    Connect,
    Data,
    GetData,
    CreateData,
    Control,
    Section,
    Change,
    Backup,
    Pass,
    Binary,
    Os,
    Kernel,
    Play,
    Destroy,
    FileName,
    When,
    Single,
    Dont,
    Use,
    Instead,
    Of,
    Block,
    Shrink,
    Math,
    Line,
    Base,
    Placeholder,

    FileDirective,
    ApiDirective,
    RepoDirective,
    WebLinkDirective,
    DatabaseDirective,
    PlayAudioIdDirective,
    PlayAudioDirective,
    AudioIdDirective,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,

    Equal,
    EqualEqual,
    NotEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    TildeEqual,

    Arrow,
    FatArrow,

    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,

    Question,
    QuestionQuestion,
    Exclamation,

    Increment,
    Decrement,

    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    ShiftLeftEqual,
    ShiftRightEqual,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Colon,
    Dot,
    Range,
    RangeInclusive,

    Semicolon,

    At,
    Hash,
    Backslash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    InvalidCharacter,
    InvalidNumber,
    InvalidEscape,
    UnterminatedString,
    UnterminatedCharacter,
    UnterminatedComment,
    InvalidDirective,
    InvalidIdentifier,
    UnexpectedEnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub message: String,
    pub position: SourcePosition,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}",
            self.message,
            self.position.line,
            self.position.column
        )
    }
}

impl std::error::Error for LexerError {}

pub struct Lexer {
    source: Vec<char>,
    offset: usize,
    line: usize,
    column: usize,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            offset: 0,
            line: 1,
            column: 1,
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> &[LexerError] {
        &self.errors
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_trivia();

            if self.at_end() {
                break;
            }

            match self.next_token() {
                Ok(token) => tokens.push(token),
                Err(error) => {
                    self.errors.push(error);
                    self.recover();
                }
            }
        }

        tokens.push(self.token(
            TokenKind::Eof,
            self.offset,
            self.offset,
            self.position(),
            self.position(),
        ));

        tokens
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let start = self.offset;
        let start_position = self.position();
        let c = self.current();

        if c == '@' {
            return self.lex_directive(start, start_position);
        }

        if c == '#' {
            return self.lex_hash(start, start_position);
        }

        if c == '"' {
            return self.lex_string(start, start_position);
        }

        if c == '\'' {
            return self.lex_character(start, start_position);
        }

        if c.is_ascii_digit() {
            return self.lex_number(start, start_position);
        }

        if c == '`' {
            return self.lex_escaped_identifier(start, start_position);
        }

        if is_identifier_start(c) {
            return Ok(self.lex_identifier(start, start_position));
        }

        self.lex_operator_or_punctuation(start, start_position)
    }

    fn lex_identifier(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Token {
        while !self.at_end() && is_identifier_continue(self.current()) {
            self.advance();
        }

        let text = self.slice(start, self.offset);
        let kind = keyword_kind(&text).unwrap_or(TokenKind::Identifier);

        self.token(
            kind,
            start,
            self.offset,
            start_position,
            self.position(),
        )
    }

    fn lex_escaped_identifier(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        self.advance();

        let content_start = self.offset;

        while !self.at_end() && self.current() != '`' {
            if self.current() == '\n' {
                return Err(self.error(
                    LexerErrorKind::InvalidIdentifier,
                    "newline in escaped identifier",
                    start_position,
                ));
            }
            self.advance();
        }

        if self.at_end() {
            return Err(self.error(
                LexerErrorKind::InvalidIdentifier,
                "unterminated escaped identifier",
                start_position,
            ));
        }

        let text = self.slice(content_start, self.offset);
        self.advance();

        Ok(self.token(
            TokenKind::EscapedIdentifier,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn lex_directive(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        self.advance();

        if self.at_end() || !is_identifier_start(self.current()) {
            return Err(self.error(
                LexerErrorKind::InvalidDirective,
                "expected a directive name after '@'",
                start_position,
            ));
        }

        while !self.at_end() && is_identifier_continue(self.current()) {
            self.advance();
        }

        let text = self.slice(start, self.offset);
        let kind = directive_kind(&text).unwrap_or(TokenKind::Directive);

        Ok(self.token(
            kind,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn lex_hash(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        self.advance();

        if self.current() == "#" {
            self.advance();
            return Ok(self.token(
                TokenKind::Hash,
                start,
                self.offset,
                start_position,
                self.position(),
            ));
        }

        Ok(self.token(
            TokenKind::Hash,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn lex_number(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        if self.current() == '0' {
            match self.peek() {
                'b' | 'B' => return self.lex_based_integer(start, start_position, 2),
                'o' | 'O' => return self.lex_based_integer(start, start_position, 8),
                'x' | 'X' => return self.lex_based_integer(start, start_position, 16),
                _ => {}
            }
        }

        self.consume_digits(10);

        let mut is_float = false;

        if self.current() == '.' && self.peek() != '.' && self.peek().is_ascii_digit() {
            is_float = true;
            self.advance();
            self.consume_digits(10);
        }

        if matches!(self.current(), 'e' | 'E') {
            is_float = true;
            self.advance();

            if matches!(self.current(), '+' | '-') {
                self.advance();
            }

            if !self.current().is_ascii_digit() {
                return Err(self.error(
                    LexerErrorKind::InvalidNumber,
                    "expected digits after exponent",
                    self.position(),
                ));
            }

            self.consume_digits(10);
        }

        let text = self.slice(start, self.offset);

        if text.ends_with('_') {
            return Err(self.error(
                LexerErrorKind::InvalidNumber,
                "numeric literal cannot end with '_'",
                start_position,
            ));
        }

        let kind = if is_float {
            TokenKind::FloatLiteral
        } else {
            TokenKind::IntegerLiteral
        };

        Ok(self.token(
            kind,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn lex_based_integer(
        &mut self,
        start: usize,
        start_position: SourcePosition,
        radix: u32,
    ) -> Result<Token, LexerError> {
        self.advance();
        self.advance();

        let digit_start = self.offset;
        let mut count = 0usize;

        while !self.at_end() {
            let c = self.current();

            if c == '_' {
                self.advance();
                continue;
            }

            if c.to_digit(radix).is_some() {
                count += 1;
                self.advance();
                continue;
            }

            if c.is_ascii_alphanumeric() {
                return Err(self.error(
                    LexerErrorKind::InvalidNumber,
                    "invalid digit in numeric literal",
                    self.position(),
                ));
            }

            break;
        }

        if count == 0 || self.offset == digit_start {
            return Err(self.error(
                LexerErrorKind::InvalidNumber,
                "numeric literal requires at least one digit",
                start_position,
            ));
        }

        let text = self.slice(start, self.offset);

        if text.ends_with('_') {
            return Err(self.error(
                LexerErrorKind::InvalidNumber,
                "numeric literal cannot end with '_'",
                start_position,
            ));
        }

        let kind = match radix {
            2 => TokenKind::BinaryLiteral,
            8 => TokenKind::OctalLiteral,
            16 => TokenKind::HexLiteral,
            _ => TokenKind::IntegerLiteral,
        };

        Ok(self.token(
            kind,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn lex_string(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        if self.peek() == '"' && self.peek2() == '"' {
            return self.lex_multiline_string(start, start_position);
        }

        self.advance();

        while !self.at_end() {
            match self.current() {
                '"' => {
                    self.advance();
                    return Ok(self.token(
                        TokenKind::StringLiteral,
                        start,
                        self.offset,
                        start_position,
                        self.position(),
                    ));
                }

                '\\' => self.consume_escape()?,

                '\n' => {
                    return Err(self.error(
                        LexerErrorKind::UnterminatedString,
                        "newline in string literal",
                        self.position(),
                    ));
                }

                _ => {
                    self.advance();
                }
            }
        }

        Err(self.error(
            LexerErrorKind::UnterminatedString,
            "unterminated string literal",
            start_position,
        ))
    }

    fn lex_multiline_string(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        self.advance();
        self.advance();
        self.advance();

        while !self.at_end() {
            if self.current() == '"' && self.peek() == '"' && self.peek2() == '"' {
                self.advance();
                self.advance();
                self.advance();

                return Ok(self.token(
                    TokenKind::MultilineStringLiteral,
                    start,
                    self.offset,
                    start_position,
                    self.position(),
                ));
            }

            if self.current() == '\\' {
                self.consume_escape()?;
            } else {
                self.advance();
            }
        }

        Err(self.error(
            LexerErrorKind::UnterminatedString,
            "unterminated multiline string literal",
            start_position,
        ))
    }

    fn lex_character(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        self.advance();

        if self.at_end() {
            return Err(self.error(
                LexerErrorKind::UnterminatedCharacter,
                "unterminated character literal",
                start_position,
            ));
        }

        if self.current() == '\\' {
            self.consume_escape()?;
        } else {
            self.advance();
        }

        if self.current() != '\'' {
            return Err(self.error(
                LexerErrorKind::UnterminatedCharacter,
                "character literal must contain one character",
                start_position,
            ));
        }

        self.advance();

        Ok(self.token(
            TokenKind::CharacterLiteral,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn consume_escape(&mut self) -> Result<(), LexerError> {
        self.advance();

        if self.at_end() {
            return Err(self.error(
                LexerErrorKind::InvalidEscape,
                "unterminated escape sequence",
                self.position(),
            ));
        }

        match self.current() {
            'n' | 'r' | 't' | '0' | '\\' | '"' | '\'' | 'b' | 'f' | 'v' | 'a' => {
                self.advance();
                Ok(())
            }

            'u' => {
                self.advance();

                if self.current() == '{' {
                    self.advance();

                    let mut count = 0;

                    while !self.at_end() && self.current() != '}' {
                        if !self.current().is_ascii_hexdigit() {
                            return Err(self.error(
                                LexerErrorKind::InvalidEscape,
                                "invalid Unicode escape",
                                self.position(),
                            ));
                        }

                        count += 1;
                        self.advance();
                    }

                    if self.current() != '}' || count == 0 {
                        return Err(self.error(
                            LexerErrorKind::InvalidEscape,
                            "invalid Unicode escape",
                            self.position(),
                        ));
                    }

                    self.advance();
                    Ok(())
                } else {
                    for _ in 0..4 {
                        if !self.current().is_ascii_hexdigit() {
                            return Err(self.error(
                                LexerErrorKind::InvalidEscape,
                                "Unicode escape requires four hexadecimal digits",
                                self.position(),
                            ));
                        }

                        self.advance();
                    }

                    Ok(())
                }
            }

            _ => Err(self.error(
                LexerErrorKind::InvalidEscape,
                "unknown escape sequence",
                self.position(),
            )),
        }
    }

    fn lex_operator_or_punctuation(
        &mut self,
        start: usize,
        start_position: SourcePosition,
    ) -> Result<Token, LexerError> {
        let c = self.current();
        let n1 = self.peek();
        let n2 = self.peek2();

        let kind = match (c, n1, n2) {
            ('.', '.', '=') => {
                self.advance();
                self.advance();
                self.advance();
                TokenKind::RangeInclusive
            }

            ('.', '.', _) => {
                self.advance();
                self.advance();
                TokenKind::Range
            }

            ('+', '+', _) => {
                self.advance();
                self.advance();
                TokenKind::Increment
            }

            ('-', '-', _) => {
                self.advance();
                self.advance();
                TokenKind::Decrement
            }

            ('+', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::PlusEqual
            }

            ('-', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::MinusEqual
            }

            ('*', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::StarEqual
            }

            ('/', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::SlashEqual
            }

            ('%', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::PercentEqual
            }

            ('&', '&', _) => {
                self.advance();
                self.advance();
                TokenKind::And
            }

            ('|', '|', _) => {
                self.advance();
                self.advance();
                TokenKind::Or
            }

            ('&', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::AmpersandEqual
            }

            ('|', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::PipeEqual
            }

            ('^', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::CaretEqual
            }

            ('<', '<', '=') => {
                self.advance();
                self.advance();
                self.advance();
                TokenKind::ShiftLeftEqual
            }

            ('>', '>', '=') => {
                self.advance();
                self.advance();
                self.advance();
                TokenKind::ShiftRightEqual
            }

            ('<', '<', _) => {
                self.advance();
                self.advance();
                TokenKind::ShiftLeft
            }

            ('>', '>', _) => {
                self.advance();
                self.advance();
                TokenKind::ShiftRight
            }

            ('=', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::EqualEqual
            }

            ('!', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::NotEqual
            }

            ('>', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::GreaterEqual
            }

            ('<', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::LessEqual
            }

            ('~', '=', _) => {
                self.advance();
                self.advance();
                TokenKind::TildeEqual
            }

            ('-', '>', _) => {
                self.advance();
                self.advance();
                TokenKind::Arrow
            }

            ('=', '>', _) => {
                self.advance();
                self.advance();
                TokenKind::FatArrow
            }

            ('?', '?', _) => {
                self.advance();
                self.advance();
                TokenKind::QuestionQuestion
            }

            ('+', _, _) => {
                self.advance();
                TokenKind::Plus
            }

            ('-', _, _) => {
                self.advance();
                TokenKind::Minus
            }

            ('*', _, _) => {
                self.advance();
                TokenKind::Star
            }

            ('/', _, _) => {
                self.advance();
                TokenKind::Slash
            }

            ('%', _, _) => {
                self.advance();
                TokenKind::Percent
            }

            ('=', _, _) => {
                self.advance();
                TokenKind::Equal
            }

            ('!', _, _) => {
                self.advance();
                TokenKind::Exclamation
            }

            ('>', _, _) => {
                self.advance();
                TokenKind::Greater
            }

            ('<', _, _) => {
                self.advance();
                TokenKind::Less
            }

            ('~', _, _) => {
                self.advance();
                TokenKind::Unknown
            }

            ('&', _, _) => {
                self.advance();
                TokenKind::BitAnd
            }

            ('|', _, _) => {
                self.advance();
                TokenKind::BitOr
            }

            ('^', _, _) => {
                self.advance();
                TokenKind::BitXor
            }

            ('?', _, _) => {
                self.advance();
                TokenKind::Question
            }

            ('(', _, _) => {
                self.advance();
                TokenKind::LeftParen
            }

            (')', _, _) => {
                self.advance();
                TokenKind::RightParen
            }

            ('{', _, _) => {
                self.advance();
                TokenKind::LeftBrace
            }

            ('}', _, _) => {
                self.advance();
                TokenKind::RightBrace
            }

            ('[', _, _) => {
                self.advance();
                TokenKind::LeftBracket
            }

            (']', _, _) => {
                self.advance();
                TokenKind::RightBracket
            }

            (',', _, _) => {
                self.advance();
                TokenKind::Comma
            }

            (':', _, _) => {
                self.advance();
                TokenKind::Colon
            }

            ('.', _, _) => {
                self.advance();
                TokenKind::Dot
            }

            (';', _, _) => {
                self.advance();
                TokenKind::Semicolon
            }

            ('@', _, _) => {
                self.advance();
                TokenKind::At
            }

            ('#', _, _) => {
                self.advance();
                TokenKind::Hash
            }

            ('\\', _, _) => {
                self.advance();
                TokenKind::Backslash
            }

            _ => {
                return Err(self.error(
                    LexerErrorKind::InvalidCharacter,
                    "invalid character",
                    start_position,
                ));
            }
        };

        Ok(self.token(
            kind,
            start,
            self.offset,
            start_position,
            self.position(),
        ))
    }

    fn skip_trivia(&mut self) {
        loop {
            let mut changed = false;

            while !self.at_end() && self.current().is_whitespace() {
                self.advance();
                changed = true;
            }

            if self.current() == ';' {
                changed = true;
                while !self.at_end() && self.current() != '\n' {
                    self.advance();
                }
            }

            if self.current() == '/' && self.peek() == '/' {
                changed = true;
                self.advance();
                self.advance();

                while !self.at_end() && self.current() != '\n' {
                    self.advance();
                }
            }

            if self.current() == '/' && self.peek() == '*' {
                changed = true;
                self.skip_block_comment();
            }

            if !changed {
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) {
        self.advance();
        self.advance();

        let mut depth = 1usize;

        while !self.at_end() {
            if self.current() == '/' && self.peek() == '*' {
                depth += 1;
                self.advance();
                self.advance();
                continue;
            }

            if self.current() == '*' && self.peek() == '/' {
                depth -= 1;
                self.advance();
                self.advance();

                if depth == 0 {
                    return;
                }

                continue;
            }

            self.advance();
        }

        self.errors.push(self.error(
            LexerErrorKind::UnterminatedComment,
            "unterminated block comment",
            self.position(),
        ));
    }

    fn recover(&mut self) {
        if self.at_end() {
            return;
        }

        while !self.at_end() {
            let c = self.current();

            if c == '\n' || c.is_whitespace() {
                self.advance();
                break;
            }

            if is_identifier_start(c) || c.is_ascii_digit() {
                break;
            }

            self.advance();
        }
    }

    fn current(&self) -> char {
        self.source.get(self.offset).copied().unwrap_or('\0')
    }

    fn peek(&self) -> char {
        self.source.get(self.offset + 1).copied().unwrap_or('\0')
    }

    fn peek2(&self) -> char {
        self.source.get(self.offset + 2).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.current();

        if self.at_end() {
            return '\0';
        }

        self.offset += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn consume_digits(&mut self, radix: u32) {
        let mut previous_was_digit = false;

        while !self.at_end() {
            let c = self.current();

            if c.to_digit(radix).is_some() {
                previous_was_digit = true;
                self.advance();
            } else if c == '_' && previous_was_digit {
                previous_was_digit = false;
                self.advance();
            } else {
                break;
            }
        }
    }

    fn at_end(&self) -> bool {
        self.offset >= self.source.len()
    }

    fn position(&self) -> SourcePosition {
        SourcePosition {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }

    fn slice(&self, start: usize, end: usize) -> String {
        self.source[start..end].iter().collect()
    }

    fn token(
        &self,
        kind: TokenKind,
        start: usize,
        end: usize,
        start_position: SourcePosition,
        end_position: SourcePosition,
    ) -> Token {
        Token {
            kind,
            text: self.slice(start, end),
            range: SourceRange {
                start: start_position,
                end: end_position,
            },
        }
    }

    fn error(
        &self,
        kind: LexerErrorKind,
        message: impl Into<String>,
        position: SourcePosition,
    ) -> LexerError {
        LexerError {
            kind,
            message: message.into(),
            position,
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit()
}

fn keyword_kind(text: &str) -> Option<TokenKind> {
    Some(match text {
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "guard" => TokenKind::Guard,
        "switch" => TokenKind::Switch,
        "case" => TokenKind::Case,
        "default" => TokenKind::Default,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "loop" => TokenKind::Loop,
        "repeat" => TokenKind::Repeat,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "do" => TokenKind::Do,
        "then" => TokenKind::Then,

        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "nil" => TokenKind::Nil,

        "var" => TokenKind::Var,
        "let" => TokenKind::Let,
        "const" => TokenKind::Const,
        "static" => TokenKind::Static,
        "lazy" => TokenKind::Lazy,
        "defer" => TokenKind::Defer,

        "int" => TokenKind::Int,
        "float" => TokenKind::Float,
        "double" => TokenKind::Double,
        "bool" => TokenKind::Bool,
        "string" => TokenKind::String,
        "char" => TokenKind::Char,
        "bytes" => TokenKind::Bytes,
        "void" => TokenKind::Void,
        "any" => TokenKind::Any,
        "never" => TokenKind::Never,
        "num" => TokenKind::Num,

        "struct" => TokenKind::Struct,
        "class" => TokenKind::Class,
        "enum" => TokenKind::Enum,
        "protocol" => TokenKind::Protocol,
        "extension" => TokenKind::Extension,
        "typealias" => TokenKind::TypeAlias,
        "associatedtype" => TokenKind::AssociatedType,

        "func" => TokenKind::Func,
        "init" => TokenKind::Init,
        "deinit" => TokenKind::Deinit,
        "return" => TokenKind::Return,

        "self" => TokenKind::SelfValue,
        "super" => TokenKind::Super,

        "import" => TokenKind::Import,
        "export" => TokenKind::Export,

        "public" => TokenKind::Public,
        "private" => TokenKind::Private,
        "internal" => TokenKind::Internal,
        "fileprivate" => TokenKind::FilePrivate,
        "package" => TokenKind::Package,

        "get" => TokenKind::Get,
        "set" => TokenKind::Set,
        "willSet" => TokenKind::WillSet,
        "didSet" => TokenKind::DidSet,
        "subscript" => TokenKind::Subscript,

        "async" => TokenKind::Async,
        "await" => TokenKind::Await,
        "actor" => TokenKind::Actor,
        "isolated" => TokenKind::Isolated,
        "nonisolated" => TokenKind::Nonisolated,

        "throws" => TokenKind::Throws,
        "throw" => TokenKind::Throw,
        "try" => TokenKind::Try,
        "catch" => TokenKind::Catch,
        "finally" => TokenKind::Finally,

        "where" => TokenKind::Where,
        "is" => TokenKind::Is,
        "as" => TokenKind::As,
        "some" => TokenKind::Some,

        "operator" => TokenKind::Operator,
        "precedence" => TokenKind::Precedence,
        "associativity" => TokenKind::Associativity,

        "move" => TokenKind::Move,
        "copy" => TokenKind::Copy,
        "borrow" => TokenKind::Borrow,
        "consume" => TokenKind::Consume,
        "owned" => TokenKind::Owned,
        "weak" => TokenKind::Weak,
        "unowned" => TokenKind::Unowned,

        "unsafe" => TokenKind::Unsafe,
        "safe" => TokenKind::Safe,
        "mutating" => TokenKind::Mutating,
        "nonmutating" => TokenKind::Nonmutating,

        "final" => TokenKind::Final,
        "override" => TokenKind::Override,
        "required" => TokenKind::Required,
        "convenience" => TokenKind::Convenience,
        "open" => TokenKind::Open,
        "dynamic" => TokenKind::Dynamic,
        "inout" => TokenKind::InOut,
        "yield" => TokenKind::Yield,
        "macro" => TokenKind::Macro,

        "match" => TokenKind::Match,

        "output" => TokenKind::Output,
        "message" => TokenKind::Message,
        "call" => TokenKind::Call,
        "error" => TokenKind::Error,
        "sink" => TokenKind::Sink,
        "range" => TokenKind::Range,
        "delete" => TokenKind::Delete,
        "cut" => TokenKind::Cut,
        "end" => TokenKind::End,
        "connect" => TokenKind::Connect,
        "data" => TokenKind::Data,
        "getData" => TokenKind::GetData,
        "createData" => TokenKind::CreateData,
        "control" => TokenKind::Control,
        "section" => TokenKind::Section,
        "change" => TokenKind::Change,
        "backup" => TokenKind::Backup,
        "pass" => TokenKind::Pass,
        "binary" => TokenKind::Binary,
        "os" => TokenKind::Os,
        "kernel" => TokenKind::Kernel,
        "play" => TokenKind::Play,
        "destroy" => TokenKind::Destroy,
        "fileName" => TokenKind::FileName,
        "when" => TokenKind::When,
        "single" => TokenKind::Single,
        "dont" => TokenKind::Dont,
        "use" => TokenKind::Use,
        "intead" => TokenKind::Instead,
        "instead" => TokenKind::Instead,
        "of" => TokenKind::Of,
        "block" => TokenKind::Block,
        "shrink" => TokenKind::Shrink,
        "math" => TokenKind::Math,
        "line" => TokenKind::Line,
        "base" => TokenKind::Base,
        "placeholder" => TokenKind::Placeholder,

        _ => return None,
    })
}

fn directive_kind(text: &str) -> Option<TokenKind> {
    Some(match text {
        "@fileID" => TokenKind::FileDirective,
        "@file" => TokenKind::FileDirective,
        "@api" => TokenKind::ApiDirective,
        "@repo" => TokenKind::RepoDirective,
        "@webLink" => TokenKind::WebLinkDirective,
        "@database" => TokenKind::DatabaseDirective,
        "@playAudioID" => TokenKind::PlayAudioIdDirective,
        "@playAudio" => TokenKind::PlayAudioDirective,
        "@AudioID" => TokenKind::AudioIdDirective,
        _ => return None,
    })
}

pub fn lex(source: &str) -> (Vec<Token>, Vec<LexerError>) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    (tokens, lexer.errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "{errors:?}");
        tokens.into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn keywords() {
        let result = kinds(
            "if else true false while do placeholder int var let defer num enum init then static output string call move error"
        );

        assert!(result.contains(&TokenKind::If));
        assert!(result.contains(&TokenKind::Else));
        assert!(result.contains(&TokenKind::True));
        assert!(result.contains(&TokenKind::If));
    }

    #[test]
    fn assignment_operators() {
        let result = kinds("= == += -= *= /= %= != >= <= ~=");

        assert_eq!(
            result,
            vec![
                TokenKind::Equal,
                TokenKind::EqualEqual,
                TokenKind::PlusEqual,
                TokenKind::MinusEqual,
                TokenKind::StarEqual,
                TokenKind::SlashEqual,
                TokenKind::PercentEqual,
                TokenKind::NotEqual,
                TokenKind::GreaterEqual,
                TokenKind::LessEqual,
                TokenKind::TildeEqual,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn dot_notation() {
        let result = kinds("system.console.output(value.name)");

        assert!(result.contains(&TokenKind::Dot));
    }

    #[test]
    fn comments() {
        let result = kinds(
            "; comment
             let value = 10",
        );

        assert!(result.contains(&TokenKind::Let));
        assert!(result.contains(&TokenKind::IntegerLiteral));
    }

    #[test]
    fn directives() {
        let result = kinds("@file @api @repo @webLink @database");

        assert_eq!(
            result,
            vec![
                TokenKind::FileDirective,
                TokenKind::ApiDirective,
                TokenKind::RepoDirective,
                TokenKind::WebLinkDirective,
                TokenKind::DatabaseDirective,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn strings() {
        let result = kinds(r#""hello" "hello\nworld" "#);

        assert_eq!(
            result,
            vec![
                TokenKind::StringLiteral,
                TokenKind::StringLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn numbers() {
        let result = kinds("10 10.5 0b1010 0o77 0xff");

        assert_eq!(
            result,
            vec![
                TokenKind::IntegerLiteral,
                TokenKind::FloatLiteral,
                TokenKind::BinaryLiteral,
                TokenKind::OctalLiteral,
                TokenKind::HexLiteral,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn function() {
        let result = kinds(
            "func add(a: int, b: int) -> int { return a + b }",
        );

        assert!(result.contains(&TokenKind::If));
        assert!(result.contains(&TokenKind::Int));
        assert!(result.contains(&TokenKind::Arrow));
        assert!(result.contains(&TokenKind::Return));
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorFixity {
    Prefix,
    Infix,
    Postfix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorAssociativity {
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorInfo {
    pub spelling: &'static str,
    pub precedence: u8,
    pub fixity: OperatorFixity,
    pub associativity: OperatorAssociativity,
}

pub fn operator_info(kind: &TokenKind) -> Option<OperatorInfo> {
    match kind {
        TokenKind::Star
        | TokenKind::StarEqual => Some(OperatorInfo {
            spelling: "*",
            precedence: 70,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::Slash
        | TokenKind::SlashEqual => Some(OperatorInfo {
            spelling: "/",
            precedence: 70,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::Percent
        | TokenKind::PercentEqual => Some(OperatorInfo {
            spelling: "%",
            precedence: 70,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::Plus
        | TokenKind::PlusEqual
        | TokenKind::Minus
        | TokenKind::MinusEqual => Some(OperatorInfo {
            spelling: "+",
            precedence: 60,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::Equal
        | TokenKind::PlusEqual
        | TokenKind::MinusEqual
        | TokenKind::StarEqual
        | TokenKind::SlashEqual
        | TokenKind::PercentEqual => Some(OperatorInfo {
            spelling: "=",
            precedence: 10,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Right,
        }),

        TokenKind::EqualEqual
        | TokenKind::NotEqual
        | TokenKind::TildeEqual => Some(OperatorInfo {
            spelling: "==",
            precedence: 40,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::None,
        }),

        TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Greater
        | TokenKind::GreaterEqual => Some(OperatorInfo {
            spelling: "<",
            precedence: 50,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::None,
        }),

        TokenKind::And => Some(OperatorInfo {
            spelling: "&&",
            precedence: 30,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::Or => Some(OperatorInfo {
            spelling: "||",
            precedence: 20,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Left,
        }),

        TokenKind::QuestionQuestion => Some(OperatorInfo {
            spelling: "??",
            precedence: 15,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Right,
        }),

        TokenKind::Exclamation => Some(OperatorInfo {
            spelling: "!",
            precedence: 90,
            fixity: OperatorFixity::Prefix,
            associativity: OperatorAssociativity::None,
        }),

        TokenKind::Increment | TokenKind::Decrement => Some(OperatorInfo {
            spelling: "++",
            precedence: 90,
            fixity: OperatorFixity::Postfix,
            associativity: OperatorAssociativity::None,
        }),

        TokenKind::Arrow => Some(OperatorInfo {
            spelling: "->",
            precedence: 5,
            fixity: OperatorFixity::Infix,
            associativity: OperatorAssociativity::Right,
        }),

        _ => None,
    }
}

pub fn token_spelling(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::Eof => "",
        TokenKind::Unknown => "",
        TokenKind::Identifier => "identifier",
        TokenKind::EscapedIdentifier => "escaped identifier",
        TokenKind::IntegerLiteral => "integer",
        TokenKind::BinaryLiteral => "binary integer",
        TokenKind::OctalLiteral => "octal integer",
        TokenKind::HexLiteral => "hex integer",
        TokenKind::FloatLiteral => "float",
        TokenKind::StringLiteral => "string",
        TokenKind::RawStringLiteral => "raw string",
        TokenKind::MultilineStringLiteral => "multiline string",
        TokenKind::CharacterLiteral => "character",
        TokenKind::Directive => "directive",
        TokenKind::If => "if",
        TokenKind::Else => "else",
        TokenKind::Guard => "guard",
        TokenKind::Switch => "switch",
        TokenKind::Case => "case",
        TokenKind::Default => "default",
        TokenKind::While => "while",
        TokenKind::For => "for",
        TokenKind::In => "in",
        TokenKind::Loop => "loop",
        TokenKind::Repeat => "repeat",
        TokenKind::Break => "break",
        TokenKind::Continue => "continue",
        TokenKind::Do => "do",
        TokenKind::Then => "then",
        TokenKind::True => "true",
        TokenKind::False => "false",
        TokenKind::Nil => "nil",
        TokenKind::Var => "var",
        TokenKind::Let => "let",
        TokenKind::Const => "const",
        TokenKind::Static => "static",
        TokenKind::Lazy => "lazy",
        TokenKind::Defer => "defer",
        TokenKind::Int => "int",
        TokenKind::Float => "float",
        TokenKind::Double => "double",
        TokenKind::Bool => "bool",
        TokenKind::String => "string",
        TokenKind::Char => "char",
        TokenKind::Bytes => "bytes",
        TokenKind::Void => "void",
        TokenKind::Any => "any",
        TokenKind::Never => "never",
        TokenKind::Num => "num",
        TokenKind::Struct => "struct",
        TokenKind::Class => "class",
        TokenKind::Enum => "enum",
        TokenKind::Protocol => "protocol",
        TokenKind::Extension => "extension",
        TokenKind::TypeAlias => "typealias",
        TokenKind::AssociatedType => "associatedtype",
        TokenKind::Func => "func",
        TokenKind::Init => "init",
        TokenKind::Deinit => "deinit",
        TokenKind::Return => "return",
        TokenKind::SelfValue => "self",
        TokenKind::Super => "super",
        TokenKind::Import => "import",
        TokenKind::Export => "export",
        TokenKind::Public => "public",
        TokenKind::Private => "private",
        TokenKind::Internal => "internal",
        TokenKind::FilePrivate => "fileprivate",
        TokenKind::Package => "package",
        TokenKind::Get => "get",
        TokenKind::Set => "set",
        TokenKind::WillSet => "willSet",
        TokenKind::DidSet => "didSet",
        TokenKind::Subscript => "subscript",
        TokenKind::Async => "async",
        TokenKind::Await => "await",
        TokenKind::Actor => "actor",
        TokenKind::Isolated => "isolated",
        TokenKind::Nonisolated => "nonisolated",
        TokenKind::Throws => "throws",
        TokenKind::Throw => "throw",
        TokenKind::Try => "try",
        TokenKind::Catch => "catch",
        TokenKind::Finally => "finally",
        TokenKind::Where => "where",
        TokenKind::Is => "is",
        TokenKind::As => "as",
        TokenKind::Some => "some",
        TokenKind::Operator => "operator",
        TokenKind::Precedence => "precedence",
        TokenKind::Associativity => "associativity",
        TokenKind::Move => "move",
        TokenKind::Copy => "copy",
        TokenKind::Borrow => "borrow",
        TokenKind::Consume => "consume",
        TokenKind::Owned => "owned",
        TokenKind::Weak => "weak",
        TokenKind::Unowned => "unowned",
        TokenKind::Unsafe => "unsafe",
        TokenKind::Safe => "safe",
        TokenKind::Mutating => "mutating",
        TokenKind::Nonmutating => "nonmutating",
        TokenKind::Final => "final",
        TokenKind::Override => "override",
        TokenKind::Required => "required",
        TokenKind::Convenience => "convenience",
        TokenKind::Open => "open",
        TokenKind::Dynamic => "dynamic",
        TokenKind::InOut => "inout",
        TokenKind::Yield => "yield",
        TokenKind::Macro => "macro",
        TokenKind::Match => "match",
        TokenKind::Panic => "!panic",
        TokenKind::Output => "output",
        TokenKind::Message => "message",
        TokenKind::Call => "call",
        TokenKind::Error => "error",
        TokenKind::Sink => "sink",
        TokenKind::Range => "range",
        TokenKind::Delete => "delete",
        TokenKind::Cut => "cut",
        TokenKind::End => "end",
        TokenKind::Connect => "connect",
        TokenKind::Data => "data",
        TokenKind::GetData => "getData",
        TokenKind::CreateData => "createData",
        TokenKind::Control => "control",
        TokenKind::Section => "section",
        TokenKind::Change => "change",
        TokenKind::Backup => "backup",
        TokenKind::Pass => "pass",
        TokenKind::Binary => "binary",
        TokenKind::Os => "os",
        TokenKind::Kernel => "kernel",
        TokenKind::Play => "play",
        TokenKind::Destroy => "destroy",
        TokenKind::FileName => "fileName",
        TokenKind::When => "when",
        TokenKind::Single => "single",
        TokenKind::Dont => "dont",
        TokenKind::Use => "use",
        TokenKind::Instead => "instead",
        TokenKind::Of => "of",
        TokenKind::Block => "block",
        TokenKind::Shrink => "shrink",
        TokenKind::Math => "math",
        TokenKind::Line => "line",
        TokenKind::Base => "base",
        TokenKind::Placeholder => "placeholder",
        TokenKind::FileDirective => "@file",
        TokenKind::ApiDirective => "@api",
        TokenKind::RepoDirective => "@repo",
        TokenKind::WebLinkDirective => "@webLink",
        TokenKind::DatabaseDirective => "@database",
        TokenKind::PlayAudioIdDirective => "@playAudioID",
        TokenKind::PlayAudioDirective => "@playAudio",
        TokenKind::AudioIdDirective => "@AudioID",
        TokenKind::Plus => "+",
        TokenKind::PlusEqual => "+=",
        TokenKind::Minus => "-",
        TokenKind::MinusEqual => "-=",
        TokenKind::Star => "*",
        TokenKind::StarEqual => "*=",
        TokenKind::Slash => "/",
        TokenKind::SlashEqual => "/=",
        TokenKind::Percent => "%",
        TokenKind::PercentEqual => "%=",
        TokenKind::Equal => "=",
        TokenKind::EqualEqual => "==",
        TokenKind::NotEqual => "!=",
        TokenKind::Greater => ">",
        TokenKind::GreaterEqual => ">=",
        TokenKind::Less => "<",
        TokenKind::LessEqual => "<=",
        TokenKind::TildeEqual => "~=",
        TokenKind::Arrow => "->",
        TokenKind::FatArrow => "=>",
        TokenKind::And => "&&",
        TokenKind::Or => "||",
        TokenKind::BitAnd => "&",
        TokenKind::BitOr => "|",
        TokenKind::BitXor => "^",
        TokenKind::ShiftLeft => "<<",
        TokenKind::ShiftRight => ">>",
        TokenKind::Question => "?",
        TokenKind::QuestionQuestion => "??",
        TokenKind::Exclamation => "!",
        TokenKind::Increment => "++",
        TokenKind::Decrement => "--",
        TokenKind::AmpersandEqual => "&=",
        TokenKind::PipeEqual => "|=",
        TokenKind::CaretEqual => "^=",
        TokenKind::ShiftLeftEqual => "<<=",
        TokenKind::ShiftRightEqual => ">>=",
        TokenKind::LeftParen => "(",
        TokenKind::RightParen => ")",
        TokenKind::LeftBrace => "{",
        TokenKind::RightBrace => "}",
        TokenKind::LeftBracket => "[",
        TokenKind::RightBracket => "]",
        TokenKind::Comma => ",",
        TokenKind::Colon => ":",
        TokenKind::Dot => ".",
        TokenKind::Range => "..",
        TokenKind::RangeInclusive => "...=",
        TokenKind::Semicolon => ";",
        TokenKind::At => "@",
        TokenKind::Hash => "#",
        TokenKind::Backslash => "\\",
    }
}

pub fn is_literal(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::IntegerLiteral
            | TokenKind::BinaryLiteral
            | TokenKind::OctalLiteral
            | TokenKind::HexLiteral
            | TokenKind::FloatLiteral
            | TokenKind::StringLiteral
            | TokenKind::RawStringLiteral
            | TokenKind::MultilineStringLiteral
            | TokenKind::CharacterLiteral
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Nil
    )
}

pub fn is_assignment_operator(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual
            | TokenKind::AmpersandEqual
            | TokenKind::PipeEqual
            | TokenKind::CaretEqual
            | TokenKind::ShiftLeftEqual
            | TokenKind::ShiftRightEqual
    )
}

pub fn is_comparison_operator(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::EqualEqual
            | TokenKind::NotEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::TildeEqual
    )
}

pub fn is_keyword_token(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::If
            | TokenKind::Else
            | TokenKind::Guard
            | TokenKind::Switch
            | TokenKind::Case
            | TokenKind::Default
            | TokenKind::While
            | TokenKind::For
            | TokenKind::In
            | TokenKind::Loop
            | TokenKind::Repeat
            | TokenKind::Break
            | TokenKind::Continue
            | TokenKind::Do
            | TokenKind::Then
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Nil
            | TokenKind::Var
            | TokenKind::Let
            | TokenKind::Const
            | TokenKind::Static
            | TokenKind::Lazy
            | TokenKind::Defer
            | TokenKind::Int
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Bool
            | TokenKind::String
            | TokenKind::Char
            | TokenKind::Bytes
            | TokenKind::Void
            | TokenKind::Any
            | TokenKind::Never
            | TokenKind::Num
            | TokenKind::Struct
            | TokenKind::Class
            | TokenKind::Enum
            | TokenKind::Protocol
            | TokenKind::Extension
            | TokenKind::TypeAlias
            | TokenKind::AssociatedType
            | TokenKind::Func
            | TokenKind::Init
            | TokenKind::Deinit
            | TokenKind::Return
            | TokenKind::SelfValue
            | TokenKind::Super
            | TokenKind::Import
            | TokenKind::Export
            | TokenKind::Public
            | TokenKind::Private
            | TokenKind::Internal
            | TokenKind::FilePrivate
            | TokenKind::Package
            | TokenKind::Async
            | TokenKind::Await
            | TokenKind::Actor
            | TokenKind::Throws
            | TokenKind::Throw
            | TokenKind::Try
            | TokenKind::Catch
            | TokenKind::Finally
            | TokenKind::Where
            | TokenKind::Is
            | TokenKind::As
            | TokenKind::Some
            | TokenKind::Move
            | TokenKind::Copy
            | TokenKind::Borrow
            | TokenKind::Consume
            | TokenKind::Owned
            | TokenKind::Weak
            | TokenKind::Unowned
            | TokenKind::Unsafe
            | TokenKind::Safe
            | TokenKind::Mutating
            | TokenKind::Nonmutating
            | TokenKind::Final
            | TokenKind::Override
            | TokenKind::Required
            | TokenKind::Convenience
            | TokenKind::Open
            | TokenKind::Dynamic
            | TokenKind::InOut
            | TokenKind::Yield
            | TokenKind::Macro
            | TokenKind::Match
            | TokenKind::Output
            | TokenKind::Message
            | TokenKind::Call
            | TokenKind::Error
            | TokenKind::Sink
            | TokenKind::Delete
            | TokenKind::Cut
            | TokenKind::End
            | TokenKind::Connect
            | TokenKind::Data
            | TokenKind::GetData
            | TokenKind::CreateData
            | TokenKind::Control
            | TokenKind::Section
            | TokenKind::Change
            | TokenKind::Backup
            | TokenKind::Pass
            | TokenKind::Binary
            | TokenKind::Os
            | TokenKind::Kernel
            | TokenKind::Play
            | TokenKind::Destroy
            | TokenKind::FileName
            | TokenKind::When
            | TokenKind::Single
            | TokenKind::Dont
            | TokenKind::Use
            | TokenKind::Instead
            | TokenKind::Of
            | TokenKind::Block
            | TokenKind::Shrink
            | TokenKind::Math
            | TokenKind::Line
            | TokenKind::Base
            | TokenKind::Placeholder
    )
}

#[cfg(test)]
mod lexer_behavior_tests {
    use super::*;

    #[test]
    fn all_requested_assignment_operators() {
        let (tokens, errors) = lex("== += -= != = *= >= <= ~=");
        assert!(errors.is_empty());

        let expected = [
            TokenKind::EqualEqual,
            TokenKind::PlusEqual,
            TokenKind::MinusEqual,
            TokenKind::NotEqual,
            TokenKind::Equal,
            TokenKind::StarEqual,
            TokenKind::GreaterEqual,
            TokenKind::LessEqual,
            TokenKind::TildeEqual,
            TokenKind::Eof,
        ];

        let actual: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn shift_dot_expression() {
        let (tokens, errors) =
            lex("system.console.output(person.name)");
        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Dot));
    }

    #[test]
    fn shift_function() {
        let (tokens, errors) = lex(
            r#"
            func main() {
                let name: string = "Shift"
                system.console.output(name)
            }
            "#,
        );

        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Func));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::StringLiteral));
    }

    #[test]
    fn semicolon_comment() {
        let (tokens, errors) = lex(
            "; comment
             let value = 10",
        );

        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
    }

    #[test]
    fn nested_block_comments() {
        let (tokens, errors) = lex(
            "/* one /* two */ three */ let x = 1",
        );

        assert!(errors.is_empty());
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
    }

    #[test]
    fn numeric_bases() {
        let (tokens, errors) =
            lex("0b1010 0o755 0xff 123_456 12.5e3");

        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::BinaryLiteral);
        assert_eq!(tokens[1].kind, TokenKind::OctalLiteral);
        assert_eq!(tokens[2].kind, TokenKind::HexLiteral);
        assert_eq!(tokens[3].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[4].kind, TokenKind::FloatLiteral);
    }

    #[test]
    fn unicode_identifier() {
        let (tokens, errors) = lex("let café = 1");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].text, "café");
    }

    #[test]
    fn escaped_identifier() {
        let (tokens, errors) = lex("let `class` = 1");
        assert!(errors.is_empty());
        assert_eq!(tokens[1].kind, TokenKind::EscapedIdentifier);
    }

    #[test]
    fn string_escape() {
        let (tokens, errors) = lex(r#""hello\nworld""#);
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn multiline_string() {
        let (tokens, errors) = lex("\"\"\"hello\nworld\"\"\"");
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::MultilineStringLiteral);
    }
}


pub const SHIFT_KEYWORDS: &[&str] = &[
    "if", "else", "guard", "switch", "case", "default",
    "while", "for", "in", "loop", "repeat", "break", "continue",
    "do", "then", "true", "false", "nil",
    "var", "let", "const", "static", "lazy", "defer",
    "int", "float", "double", "bool", "string", "char", "bytes",
    "void", "any", "never", "num",
    "struct", "class", "enum", "protocol", "extension",
    "typealias", "associatedtype",
    "func", "init", "deinit", "return", "self", "super",
    "import", "export", "public", "private", "internal",
    "fileprivate", "package",
    "get", "set", "willSet", "didSet", "subscript",
    "async", "await", "actor", "isolated", "nonisolated",
    "throws", "throw", "try", "catch", "finally",
    "where", "is", "as", "some",
    "operator", "precedence", "associativity",
    "move", "copy", "borrow", "consume", "owned",
    "weak", "unowned", "unsafe", "safe",
    "mutating", "nonmutating",
    "final", "override", "required", "convenience",
    "open", "dynamic", "inout", "yield", "macro", "match",
    "output", "message", "call", "error", "sink", "range",
    "delete", "cut", "end", "connect", "data", "getData",
    "createData", "control", "section", "change", "backup",
    "pass", "binary", "os", "kernel", "play", "destroy",
    "fileName", "when", "single", "dont", "use", "intead",
    "instead", "of", "block", "shrink", "math", "line",
    "base", "placeholder",
];

pub const SHIFT_DIRECTIVES: &[&str] = &[
    "@fileID",
    "@file",
    "@api",
    "@repo",
    "@webLink",
    "@database",
    "@playAudioID",
    "@playAudio",
    "@AudioID",
];

pub const SHIFT_OPERATORS: &[&str] = &[
    "==", "+=", "-=", "!=", "=", "*=", "/=", "%=",
    ">=", "<=", "~=", "+", "-", "*", "/", "%",
    ">", "<", "&&", "||", "??", "->", "=>",
    "&", "|", "^", "<<", ">>", "&=", "|=", "^=",
    "<<=", ">>=", "?", "!", "++", "--",
    "(", ")", "{", "}", "[", "]",
    ",", ":", ".", "..", "...=", ";",
];

pub fn is_shift_keyword(text: &str) -> bool {
    SHIFT_KEYWORDS.iter().any(|item| *item == text)
        || keyword_kind(text).is_some()
}

pub fn is_shift_directive(text: &str) -> bool {
    SHIFT_DIRECTIVES.iter().any(|item| *item == text)
}

pub fn is_shift_operator(text: &str) -> bool {
    SHIFT_OPERATORS.iter().any(|item| *item == text)
}

pub fn classify_word(text: &str) -> TokenKind {
    keyword_kind(text).unwrap_or(TokenKind::Identifier)
}

pub fn classify_directive(text: &str) -> TokenKind {
    directive_kind(text).unwrap_or(TokenKind::Directive)
}

pub fn lex_one(source: &str) -> Result<Token, LexerError> {
    let (tokens, errors) = lex(source);

    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    Ok(tokens.into_iter().next().unwrap_or(Token {
        kind: TokenKind::Eof,
        text: String::new(),
        range: SourceRange {
            start: SourcePosition {
                offset: 0,
                line: 1,
                column: 1,
            },
            end: SourcePosition {
                offset: 0,
                line: 1,
                column: 1,
            },
        },
    }))
}

pub fn format_token(token: &Token) -> String {
    format!(
        "{}:{} {} {:?}",
        token.range.start.line,
        token.range.start.column,
        token_spelling(&token.kind),
        token.text
    )
}

pub fn dump_tokens(tokens: &[Token]) -> String {
    let mut output = String::new();

    for token in tokens {
        output.push_str(&format_token(token));
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod lexer_case_1 {
    use super::*;

    #[test]
    fn case_1() {
        let source = match 1 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_2 {
    use super::*;

    #[test]
    fn case_2() {
        let source = match 2 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_3 {
    use super::*;

    #[test]
    fn case_3() {
        let source = match 3 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_4 {
    use super::*;

    #[test]
    fn case_4() {
        let source = match 4 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_5 {
    use super::*;

    #[test]
    fn case_5() {
        let source = match 5 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_6 {
    use super::*;

    #[test]
    fn case_6() {
        let source = match 6 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_7 {
    use super::*;

    #[test]
    fn case_7() {
        let source = match 7 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_8 {
    use super::*;

    #[test]
    fn case_8() {
        let source = match 8 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_9 {
    use super::*;

    #[test]
    fn case_9() {
        let source = match 9 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_10 {
    use super::*;

    #[test]
    fn case_10() {
        let source = match 10 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_11 {
    use super::*;

    #[test]
    fn case_11() {
        let source = match 11 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_12 {
    use super::*;

    #[test]
    fn case_12() {
        let source = match 12 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_13 {
    use super::*;

    #[test]
    fn case_13() {
        let source = match 13 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_14 {
    use super::*;

    #[test]
    fn case_14() {
        let source = match 14 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_15 {
    use super::*;

    #[test]
    fn case_15() {
        let source = match 15 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_16 {
    use super::*;

    #[test]
    fn case_16() {
        let source = match 16 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_17 {
    use super::*;

    #[test]
    fn case_17() {
        let source = match 17 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_18 {
    use super::*;

    #[test]
    fn case_18() {
        let source = match 18 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_19 {
    use super::*;

    #[test]
    fn case_19() {
        let source = match 19 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_20 {
    use super::*;

    #[test]
    fn case_20() {
        let source = match 20 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_21 {
    use super::*;

    #[test]
    fn case_21() {
        let source = match 21 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_22 {
    use super::*;

    #[test]
    fn case_22() {
        let source = match 22 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_23 {
    use super::*;

    #[test]
    fn case_23() {
        let source = match 23 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_24 {
    use super::*;

    #[test]
    fn case_24() {
        let source = match 24 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}


#[cfg(test)]
mod lexer_case_25 {
    use super::*;

    #[test]
    fn case_25() {
        let source = match 25 % 16 {
            0 => "if value == 10 { output(value) }",
            1 => "let value = 10",
            2 => "var value += 2",
            3 => "value -= 1",
            4 => "value *= 3",
            5 => "value /= 2",
            6 => "value %= 2",
            7 => "value != 20",
            8 => "value >= 5",
            9 => "value <= 50",
            10 => "value ~= 10",
            11 => "system.console.output(value.name)",
            12 => "@file @api @repo",
            13 => "func main() { return }",
            14 => "0b1010 0o77 0xff 12.5",
            _ => "\"hello\\nShift\"",
        };

        let (tokens, errors) = lex(source);
        assert!(errors.is_empty(), "lexer errors: {errors:?}");
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }
}