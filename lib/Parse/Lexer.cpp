#include "Lexer.h"

#include <cctype>
#include <sstream>
#include <unordered_map>

namespace Shift {

Lexer::Lexer(std::string source)
    : source(std::move(source)) {
}

std::vector<Token> Lexer::lex() {
    tokens.clear();
    errors.clear();

    current = 0;
    line = 1;
    column = 1;

    while (!isAtEnd()) {
        skipWhitespaceAndComments();

        if (isAtEnd()) {
            break;
        }

        scanToken();
    }

    Token eof;
    eof.kind = TokenKind::EndOfFile;
    eof.text = "";
    eof.location = location();

    tokens.push_back(std::move(eof));

    return tokens;
}

const std::vector<std::string>& Lexer::diagnostics() const {
    return errors;
}

bool Lexer::hasErrors() const {
    return !errors.empty();
}

char Lexer::peek(std::size_t distance) const {
    const std::size_t position = current + distance;

    if (position >= source.size()) {
        return '\0';
    }

    return source[position];
}

char Lexer::advance() {
    if (isAtEnd()) {
        return '\0';
    }

    char c = source[current++];

    if (c == '\n') {
        ++line;
        column = 1;
    } else {
        ++column;
    }

    return c;
}

bool Lexer::match(char expected) {
    if (peek() != expected) {
        return false;
    }

    advance();
    return true;
}

bool Lexer::isAtEnd() const {
    return current >= source.size();
}

SourceLocation Lexer::location() const {
    SourceLocation result;
    result.offset = current;
    result.line = line;
    result.column = column;
    return result;
}

void Lexer::addError(const std::string& message,
                     const SourceLocation& location) {
    std::ostringstream stream;

    stream << "error at "
           << location.line
           << ":"
           << location.column
           << ": "
           << message;

    errors.push_back(stream.str());
}

void Lexer::addToken(TokenKind kind,
                     std::size_t start,
                     SourceLocation location) {
    Token token;

    token.kind = kind;
    token.text = source.substr(start, current - start);
    token.location = location;

    tokens.push_back(std::move(token));
}

bool Lexer::isDigit(char c) const {
    return c >= '0' && c <= '9';
}

bool Lexer::isHexDigit(char c) const {
    return (c >= '0' && c <= '9') ||
           (c >= 'a' && c <= 'f') ||
           (c >= 'A' && c <= 'F');
}

bool Lexer::isBinaryDigit(char c) const {
    return c == '0' || c == '1';
}

bool Lexer::isOctalDigit(char c) const {
    return c >= '0' && c <= '7';
}

bool Lexer::isIdentifierStart(char c) const {
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
           c == '_';
}

bool Lexer::isIdentifierContinue(char c) const {
    return isIdentifierStart(c) || isDigit(c);
}

void Lexer::skipWhitespaceAndComments() {
    while (!isAtEnd()) {
        char c = peek();

        if (c == ' ' ||
            c == '\t' ||
            c == '\r' ||
            c == '\n') {
            advance();
            continue;
        }

        // ';' starts a line comment in Shift.
        if (c == ';') {
            while (!isAtEnd() && peek() != '\n') {
                advance();
            }

            continue;
        }

        // C/C++ style comments are also accepted by the lexer.
        if (c == '/' && peek(1) == '/') {
            advance();
            advance();

            while (!isAtEnd() && peek() != '\n') {
                advance();
            }

            continue;
        }

        if (c == '/' && peek(1) == '*') {
            SourceLocation commentLocation = location();

            advance();
            advance();

            bool closed = false;

            while (!isAtEnd()) {
                if (peek() == '*' && peek(1) == '/') {
                    advance();
                    advance();
                    closed = true;
                    break;
                }

                advance();
            }

            if (!closed) {
                addError("unterminated block comment",
                         commentLocation);
            }

            continue;
        }

        break;
    }
}

void Lexer::scanToken() {
    char c = peek();

    if (isIdentifierStart(c)) {
        scanIdentifierOrKeyword();
        return;
    }

    if (isDigit(c)) {
        scanNumber();
        return;
    }

    if (c == '"') {
        scanString();
        return;
    }

    if (c == '\'') {
        scanCharacter();
        return;
    }

    scanOperatorOrPunctuation();
}

void Lexer::scanIdentifierOrKeyword() {
    const std::size_t start = current;
    const SourceLocation startLocation = location();

    advance();

    while (isIdentifierContinue(peek())) {
        advance();
    }

    std::string_view text(
        source.data() + start,
        current - start
    );

    TokenKind kind = keywordKind(text);

    if (kind == TokenKind::Identifier) {
        kind = builtinKind(text);
    }

    addToken(kind, start, startLocation);
}

TokenKind Lexer::keywordKind(std::string_view text) const {
    // Shift keywords are intentionally hardcoded.
    if (text == "true")       return TokenKind::True;
    if (text == "false")      return TokenKind::False;
    if (text == "struct")     return TokenKind::Struct;
    if (text == "func")       return TokenKind::Func;
    if (text == "then")       return TokenKind::Then;
    if (text == "if")         return TokenKind::If;
    if (text == "else")       return TokenKind::Else;
    if (text == "do")         return TokenKind::Do;
    if (text == "try")        return TokenKind::Try;
    if (text == "raw")        return TokenKind::Raw;
    if (text == "safe")       return TokenKind::Safe;
    if (text == "in")         return TokenKind::In;
    if (text == "for")        return TokenKind::For;
    if (text == "enum")       return TokenKind::Enum;
    if (text == "num")        return TokenKind::Num;
    if (text == "int")        return TokenKind::Int;
    if (text == "as")         return TokenKind::As;
    if (text == "async")      return TokenKind::Async;
    if (text == "fileID")     return TokenKind::FileID;
    if (text == "file")       return TokenKind::File;
    if (text == "elseif")     return TokenKind::ElseIf;
    if (text == "nil")        return TokenKind::Nil;
    if (text == "destroy")    return TokenKind::Destroy;
    if (text == "self")       return TokenKind::Self;
    if (text == "any")        return TokenKind::Any;
    if (text == "throws")     return TokenKind::Throws;
    if (text == "error")      return TokenKind::Error;
    if (text == "defer")      return TokenKind::Defer;
    if (text == "break")      return TokenKind::Break;
    if (text == "continue")   return TokenKind::Continue;
    if (text == "where")      return TokenKind::Where;
    if (text == "while")      return TokenKind::While;
    if (text == "return")     return TokenKind::Return;
    if (text == "repeat")     return TokenKind::Repeat;
    if (text == "guard")      return TokenKind::Guard;
    if (text == "yield")      return TokenKind::Yield;
    if (text == "add")        return TokenKind::Add;
    if (text == "let")        return TokenKind::Let;
    if (text == "var")        return TokenKind::Var;
    if (text == "import")     return TokenKind::Import;
    if (text == "protocol")   return TokenKind::Protocol;
    if (text == "class")      return TokenKind::Class;
    if (text == "static")     return TokenKind::Static;
    if (text == "internal")   return TokenKind::Internal;
    if (text == "init")       return TokenKind::Init;
    if (text == "deinit")     return TokenKind::Deinit;
    if (text == "extension")  return TokenKind::Extension;
    if (text == "type")       return TokenKind::Type;
    if (text == "string")     return TokenKind::String;
    if (text == "bool")       return TokenKind::Bool;

    return TokenKind::Identifier;
}

TokenKind Lexer::builtinKind(std::string_view text) const {
    // Shift built-ins are hardcoded too.
    if (text == "print")
        return TokenKind::BuiltinPrint;

    if (text == "errorMessage")
        return TokenKind::BuiltinErrorMessage;

    if (text == "panic")
        return TokenKind::BuiltinPanic;

    if (text == "abs")
        return TokenKind::BuiltinAbs;

    if (text == "min")
        return TokenKind::BuiltinMin;

    if (text == "max")
        return TokenKind::BuiltinMax;

    if (text == "stride")
        return TokenKind::BuiltinStride;

    if (text == "dump")
        return TokenKind::BuiltinDump;

    if (text == "read")
        return TokenKind::BuiltinRead;

    if (text == "input")
        return TokenKind::BuiltinInput;

    if (text == "length")
        return TokenKind::BuiltinLength;

    if (text == "type")
        return TokenKind::BuiltinType;

    if (text == "sort")
        return TokenKind::BuiltinSort;

    if (text == "reverse")
        return TokenKind::BuiltinReverse;

    if (text == "exit")
        return TokenKind::BuiltinExit;

    return TokenKind::Identifier;
}

void Lexer::scanNumber() {
    const std::size_t start = current;
    const SourceLocation startLocation = location();

    bool isFloat = false;

    // Hexadecimal.
    if (peek() == '0' &&
        (peek(1) == 'x' || peek(1) == 'X')) {

        advance();
        advance();

        if (!isHexDigit(peek())) {
            addError("expected hexadecimal digit after 0x",
                     startLocation);
        }

        while (isHexDigit(peek()) || peek() == '_') {
            advance();
        }

        addToken(TokenKind::IntegerLiteral,
                 start,
                 startLocation);

        return;
    }

    // Binary.
    if (peek() == '0' &&
        (peek(1) == 'b' || peek(1) == 'B')) {

        advance();
        advance();

        if (!isBinaryDigit(peek())) {
            addError("expected binary digit after 0b",
                     startLocation);
        }

        while (isBinaryDigit(peek()) || peek() == '_') {
            advance();
        }

        addToken(TokenKind::IntegerLiteral,
                 start,
                 startLocation);

        return;
    }

    // Octal.
    if (peek() == '0' &&
        (peek(1) == 'o' || peek(1) == 'O')) {

        advance();
        advance();

        if (!isOctalDigit(peek())) {
            addError("expected octal digit after 0o",
                     startLocation);
        }

        while (isOctalDigit(peek()) || peek() == '_') {
            advance();
        }

        addToken(TokenKind::IntegerLiteral,
                 start,
                 startLocation);

        return;
    }

    while (isDigit(peek()) || peek() == '_') {
        advance();
    }

    if (peek() == '.' &&
        peek(1) != '.' &&
        isDigit(peek(1))) {

        isFloat = true;

        advance();

        while (isDigit(peek()) || peek() == '_') {
            advance();
        }
    }

    if (peek() == 'e' || peek() == 'E') {
        isFloat = true;

        advance();

        if (peek() == '+' || peek() == '-') {
            advance();
        }

        if (!isDigit(peek())) {
            addError("expected exponent digits",
                     location());
        }

        while (isDigit(peek()) || peek() == '_') {
            advance();
        }
    }

    addToken(
        isFloat
            ? TokenKind::FloatLiteral
            : TokenKind::IntegerLiteral,
        start,
        startLocation
    );
}

std::string Lexer::decodeEscape(
    char escaped,
    SourceLocation escapeLocation) {

    switch (escaped) {
        case 'n':
            return "\n";

        case 'r':
            return "\r";

        case 't':
            return "\t";

        case '0':
            return std::string(1, '\0');

        case '\\':
            return "\\";

        case '"':
            return "\"";

        case '\'':
            return "'";

        case 'b':
            return "\b";

        case 'f':
            return "\f";

        case 'v':
            return "\v";

        default:
            addError(
                std::string("unknown escape sequence \\") + escaped,
                escapeLocation
            );

            return std::string(1, escaped);
    }
}

void Lexer::scanString() {
    const std::size_t start = current;
    const SourceLocation startLocation = location();

    advance();

    bool terminated = false;

    while (!isAtEnd()) {
        char c = peek();

        if (c == '"') {
            advance();
            terminated = true;
            break;
        }

        if (c == '\n') {
            addError(
                "newline inside string literal",
                location()
            );

            break;
        }

        if (c == '\\') {
            SourceLocation escapeLocation = location();

            advance();

            if (isAtEnd()) {
                addError(
                    "unterminated escape sequence",
                    escapeLocation
                );

                break;
            }

            char escaped = advance();

            decodeEscape(escaped, escapeLocation);
            continue;
        }

        advance();
    }

    if (!terminated) {
        addError(
            "unterminated string literal",
            startLocation
        );
    }

    addToken(
        TokenKind::StringLiteral,
        start,
        startLocation
    );
}

void Lexer::scanCharacter() {
    const std::size_t start = current;
    const SourceLocation startLocation = location();

    advance();

    if (isAtEnd()) {
        addError(
            "unterminated character literal",
            startLocation
        );

        addToken(
            TokenKind::CharacterLiteral,
            start,
            startLocation
        );

        return;
    }

    if (peek() == '\n') {
        addError(
            "newline inside character literal",
            location()
        );

        addToken(
            TokenKind::CharacterLiteral,
            start,
            startLocation
        );

        return;
    }

    if (peek() == '\\') {
        SourceLocation escapeLocation = location();

        advance();

        if (isAtEnd()) {
            addError(
                "unterminated character escape",
                escapeLocation
            );

            addToken(
                TokenKind::CharacterLiteral,
                start,
                startLocation
            );

            return;
        }

        char escaped = advance();

        decodeEscape(escaped, escapeLocation);
    } else {
        advance();
    }

    if (peek() != '\'') {
        addError(
            "character literal must contain one character",
            startLocation
        );

        while (!isAtEnd() &&
               peek() != '\'' &&
               peek() != '\n') {
            advance();
        }
    }

    if (peek() == '\'') {
        advance();
    } else {
        addError(
            "unterminated character literal",
            startLocation
        );
    }

    addToken(
        TokenKind::CharacterLiteral,
        start,
        startLocation
    );
}

void Lexer::scanOperatorOrPunctuation() {
    const std::size_t start = current;
    const SourceLocation startLocation = location();

    char c = advance();

    switch (c) {
        case '+':
            if (match('+')) {
                addToken(
                    TokenKind::PlusPlus,
                    start,
                    startLocation
                );
            } else if (match('=')) {
                addToken(
                    TokenKind::PlusEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Plus,
                    start,
                    startLocation
                );
            }
            return;

        case '-':
            if (match('-')) {
                addToken(
                    TokenKind::MinusMinus,
                    start,
                    startLocation
                );
            } else if (match('=')) {
                addToken(
                    TokenKind::MinusEqual,
                    start,
                    startLocation
                );
            } else if (match('>')) {
                addToken(
                    TokenKind::Arrow,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Minus,
                    start,
                    startLocation
                );
            }
            return;

        case '*':
            if (match('=')) {
                addToken(
                    TokenKind::StarEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Star,
                    start,
                    startLocation
                );
            }
            return;

        case '/':
            if (match('=')) {
                addToken(
                    TokenKind::SlashEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Slash,
                    start,
                    startLocation
                );
            }
            return;

        case '%':
            if (match('=')) {
                addToken(
                    TokenKind::PercentEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Percent,
                    start,
                    startLocation
                );
            }
            return;

        case '=':
            if (match('=')) {
                addToken(
                    TokenKind::EqualEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Equal,
                    start,
                    startLocation
                );
            }
            return;

        case '!':
            if (match('=')) {
                addToken(
                    TokenKind::NotEqual,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Not,
                    start,
                    startLocation
                );
            }
            return;

        case '<':
            if (match('=')) {
                addToken(
                    TokenKind::LessEqual,
                    start,
                    startLocation
                );
            } else if (match('<')) {
                addToken(
                    TokenKind::ShiftLeft,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Less,
                    start,
                    startLocation
                );
            }
            return;

        case '>':
            if (match('=')) {
                addToken(
                    TokenKind::GreaterEqual,
                    start,
                    startLocation
                );
            } else if (match('>')) {
                addToken(
                    TokenKind::ShiftRight,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Greater,
                    start,
                    startLocation
                );
            }
            return;

        case '&':
            if (match('&')) {
                addToken(
                    TokenKind::AndAnd,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::BitAnd,
                    start,
                    startLocation
                );
            }
            return;

        case '|':
            if (match('|')) {
                addToken(
                    TokenKind::OrOr,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::BitOr,
                    start,
                    startLocation
                );
            }
            return;

        case '^':
            addToken(
                TokenKind::BitXor,
                start,
                startLocation
            );
            return;

        case '~':
            addToken(
                TokenKind::BitNot,
                start,
                startLocation
            );
            return;

        case '.':
            if (match('.')) {
                if (match('<')) {
                    addToken(
                        TokenKind::DotDotLess,
                        start,
                        startLocation
                    );
                } else {
                    addToken(
                        TokenKind::DotDot,
                        start,
                        startLocation
                    );
                }
            } else {
                addToken(
                    TokenKind::Dot,
                    start,
                    startLocation
                );
            }
            return;

        case '?':
            if (match('?')) {
                addToken(
                    TokenKind::QuestionQuestion,
                    start,
                    startLocation
                );
            } else {
                addToken(
                    TokenKind::Question,
                    start,
                    startLocation
                );
            }
            return;

        case ':':
            addToken(
                TokenKind::Colon,
                start,
                startLocation
            );
            return;

        case ',':
            addToken(
                TokenKind::Comma,
                start,
                startLocation
            );
            return;

        case '(':
            addToken(
                TokenKind::LeftParen,
                start,
                startLocation
            );
            return;

        case ')':
            addToken(
                TokenKind::RightParen,
                start,
                startLocation
            );
            return;

        case '[':
            addToken(
                TokenKind::LeftBracket,
                start,
                startLocation
            );
            return;

        case ']':
            addToken(
                TokenKind::RightBracket,
                start,
                startLocation
            );
            return;

        case '{':
            addToken(
                TokenKind::LeftBrace,
                start,
                startLocation
            );
            return;

        case '}':
            addToken(
                TokenKind::RightBrace,
                start,
                startLocation
            );
            return;

        case ';':
            addToken(
                TokenKind::Semicolon,
                start,
                startLocation
            );
            return;

        default:
            addError(
                std::string("unexpected character '") + c + "'",
                startLocation
            );

            addToken(
                TokenKind::Unknown,
                start,
                startLocation
            );

            return;
    }
}

const char* Lexer::tokenKindName(TokenKind kind) {
    switch (kind) {
        case TokenKind::EndOfFile:
            return "EndOfFile";

        case TokenKind::Unknown:
            return "Unknown";

        case TokenKind::Identifier:
            return "Identifier";

        case TokenKind::IntegerLiteral:
            return "IntegerLiteral";

        case TokenKind::FloatLiteral:
            return "FloatLiteral";

        case TokenKind::StringLiteral:
            return "StringLiteral";

        case TokenKind::CharacterLiteral:
            return "CharacterLiteral";

        case TokenKind::True:
            return "True";

        case TokenKind::False:
            return "False";

        case TokenKind::Struct:
            return "Struct";

        case TokenKind::Func:
            return "Func";

        case TokenKind::Then:
            return "Then";

        case TokenKind::If:
            return "If";

        case TokenKind::Else:
            return "Else";

        case TokenKind::Do:
            return "Do";

        case TokenKind::Try:
            return "