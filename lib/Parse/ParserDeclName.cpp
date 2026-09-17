// ParseDeclName.cpp

#include "Parser.h"

namespace Shift {
namespace Parser {

std::string Parser::parseDeclarationName(const char *declarationKind) {
    if (!check(TokenKind::Identifier)) {
        std::string message = "expected ";
        message += declarationKind;
        message += " name";

        error(current(), message);
        return {};
    }

    std::string name = current().text;
    advance();

    return name;
}

std::string Parser::parseOptionalDeclarationName(
    const char *declarationKind
) {
    if (!check(TokenKind::Identifier)) {
        return {};
    }

    std::string name = current().text;
    advance();

    return name;
}

bool Parser::isValidDeclarationName(const std::string &name) const {
    if (name.empty()) {
        return false;
    }

    const unsigned char first =
        static_cast<unsigned char>(name.front());

    if (!(
        (first >= 'a' && first <= 'z') ||
        (first >= 'A' && first <= 'Z') ||
        first == '_'
    )) {
        return false;
    }

    for (std::size_t i = 1; i < name.size(); ++i) {
        const unsigned char c =
            static_cast<unsigned char>(name[i]);

        if (!(
            (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            (c >= '0' && c <= '9') ||
            c == '_'
        )) {
            return false;
        }
    }

    return true;
}

void Parser::validateDeclarationName(
    const std::string &name,
    const Token &token,
    const char *declarationKind
) {
    if (name.empty()) {
        return;
    }

    if (!isValidDeclarationName(name)) {
        std::string message = "invalid ";
        message += declarationKind;
        message += " name '" + name + "'";

        error(token, message);
    }

    if (name.front() == '_') {
        return;
    }

    if (name.size() > 1 &&
        name.front() >= 'A' &&
        name.front() <= 'Z') {

        std::string message =
            "declaration name '" + name +
            "' should use lowerCamelCase";

        warning(token, message);
    }
}

std::string Parser::parseFunctionName() {
    if (!check(TokenKind::Identifier)) {
        error(current(), "expected function name");
        return {};
    }

    Token nameToken = current();
    std::string name = current().text;
    advance();

    validateDeclarationName(
        name,
        nameToken,
        "function"
    );

    return name;
}

std::string Parser::parseTypeName() {
    if (check(TokenKind::Identifier)) {
        Token nameToken = current();
        std::string name = current().text;
        advance();

        validateDeclarationName(
            name,
            nameToken,
            "type"
        );

        return name;
    }

    switch (current().kind) {
        case TokenKind::Int:
            advance();
            return "int";

        case TokenKind::String:
            advance();
            return "string";

        case TokenKind::Bool:
            advance();
            return "bool";

        case TokenKind::Any:
            advance();
            return "any";

        case TokenKind::Num:
            advance();
            return "num";

        default:
            error(current(), "expected type name");
            return {};
    }
}

} // namespace Parser
} // namespace Shift