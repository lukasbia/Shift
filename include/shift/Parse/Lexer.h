#ifndef SHIFT_LEXER_H
#define SHIFT_LEXER_H

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

namespace Shift {

enum class TokenKind {
    EndOfFile,
    Unknown,

    Identifier,

    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharacterLiteral,

    True,
    False,
    Struct,
    Func,
    Then,
    If,
    Else,
    Do,
    Try,
    Raw,
    Safe,
    In,
    For,
    Enum,
    Num,
    Int,
    As,
    Async,
    FileID,
    File,
    ElseIf,
    Nil,
    Destroy,
    Self,
    Any,
    Throws,
    Error,
    Defer,
    Break,
    Continue,
    Where,
    While,
    Return,
    Repeat,
    Guard,
    Yield,
    Add,
    Let,
    Var,
    Import,
    Protocol,
    Class,
    Static,
    Internal,
    Init,
    Deinit,
    Extension,
    Type,
    String,
    Bool,

    BuiltinPrint,
    BuiltinErrorMessage,
    BuiltinPanic,
    BuiltinAbs,
    BuiltinMin,
    BuiltinMax,
    BuiltinStride,
    BuiltinDump,
    BuiltinRead,
    BuiltinInput,
    BuiltinLength,
    BuiltinType,
    BuiltinSort,
    BuiltinReverse,
    BuiltinExit,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    PlusPlus,
    MinusMinus,

    Equal,
    EqualEqual,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    AndAnd,
    Or,
    OrOr,
    Not,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,

    Arrow,
    Dot,
    DotDot,
    DotDotLess,
    Question,
    QuestionQuestion,

    Colon,
    Comma,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Semicolon
};

struct SourceLocation {
    std::size_t offset = 0;
    std::size_t line = 1;
    std::size_t column = 1;
};

struct Token {
    TokenKind kind = TokenKind::Unknown;
    std::string text;
    SourceLocation location;
};

class Lexer {
public:
    explicit Lexer(std::string source);

    std::vector<Token> lex();

    const std::vector<std::string>& diagnostics() const;

    bool hasErrors() const;

    static const char* tokenKindName(TokenKind kind);

private:
    std::string source;

    std::size_t current = 0;
    std::size_t line = 1;
    std::size_t column = 1;

    std::vector<std::string> errors;

    std::vector<Token> tokens;

    char peek(std::size_t distance = 0) const;
    char advance();
    bool match(char expected);

    bool isAtEnd() const;

    SourceLocation location() const;

    void addError(const std::string& message,
                  const SourceLocation& location);

    void addToken(TokenKind kind,
                  std::size_t start,
                  SourceLocation location);

    void scanToken();

    void skipWhitespaceAndComments();

    void scanIdentifierOrKeyword();
    void scanNumber();
    void scanString();
    void scanCharacter();

    bool isIdentifierStart(char c) const;
    bool isIdentifierContinue(char c) const;
    bool isDigit(char c) const;
    bool isHexDigit(char c) const;
    bool isBinaryDigit(char c) const;
    bool isOctalDigit(char c) const;

    TokenKind keywordKind(std::string_view text) const;
    TokenKind builtinKind(std::string_view text) const;

    std::string decodeEscape(char escaped,
                             SourceLocation escapeLocation);

    void scanOperatorOrPunctuation();
};

} // namespace Shift

#endif