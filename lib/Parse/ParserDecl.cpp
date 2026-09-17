// ParseDecl.cpp

#include "Parser.h"

namespace Shift {
namespace Parser {

DeclarationPtr Parser::parseDeclaration() {
    switch (current().kind) {
        case TokenKind::Import:
            return parseImportDeclaration();

        case TokenKind::Struct:
            return parseStructDeclaration();

        case TokenKind::Class:
            return parseClassDeclaration();

        case TokenKind::Protocol:
            return parseProtocolDeclaration();

        case TokenKind::Enum:
            return parseEnumDeclaration();

        case TokenKind::Async:
        case TokenKind::Func:
            return parseFunctionDeclaration();

        case TokenKind::Let:
        case TokenKind::Var:
            return parseVariableDeclaration();

        default:
            return nullptr;
    }
}

DeclarationPtr Parser::parseImportDeclaration() {
    SourceLocation location = current().location;

    consume(TokenKind::Import, "expected 'import'");

    std::vector<std::string> modules;

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected module name after 'import'");
        return nullptr;
    }

    modules.push_back(previous().text);

    while (match(TokenKind::Dot)) {
        if (!check(TokenKind::Identifier)) {
            error(current(), "expected identifier after '.'");
            break;
        }

        modules.push_back(previous().text);
    }

    consumeOptional(TokenKind::Semicolon);

    return std::make_shared<ImportDeclaration>(
        std::move(modules),
        location
    );
}

DeclarationPtr Parser::parseStructDeclaration() {
    SourceLocation location = current().location;

    consume(TokenKind::Struct, "expected 'struct'");

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected struct name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<StructDeclaration>(name, location);

    if (!consume(TokenKind::LeftBrace, "expected '{' after struct name")) {
        synchronizeDeclaration();
        return declaration;
    }

    while (!isAtEnd() && !check(TokenKind::RightBrace)) {
        if (check(TokenKind::Var) || check(TokenKind::Let)) {
            auto member = parseVariableDeclaration();

            if (member) {
                declaration->members.push_back(member);
            }

            continue;
        }

        if (check(TokenKind::Func) || check(TokenKind::Async)) {
            auto method = parseFunctionDeclaration();

            if (method) {
                declaration->methods.push_back(method);
            }

            continue;
        }

        error(current(),
              "expected property or function declaration inside struct");

        synchronizeDeclaration();
    }

    consume(TokenKind::RightBrace, "expected '}' after struct declaration");

    return declaration;
}

DeclarationPtr Parser::parseClassDeclaration() {
    SourceLocation location = current().location;

    consume(TokenKind::Class, "expected 'class'");

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected class name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<ClassDeclaration>(name, location);

    if (match(TokenKind::Colon)) {
        if (!check(TokenKind::Identifier)) {
            error(current(), "expected superclass name after ':'");
        } else {
            declaration->superclass = current().text;
            advance();
        }
    }

    if (!consume(TokenKind::LeftBrace, "expected '{' after class declaration")) {
        synchronizeDeclaration();
        return declaration;
    }

    while (!isAtEnd() && !check(TokenKind::RightBrace)) {
        if (check(TokenKind::Var) || check(TokenKind::Let)) {
            auto member = parseVariableDeclaration();

            if (member) {
                declaration->members.push_back(member);
            }

            continue;
        }

        if (check(TokenKind::Func) ||
            check(TokenKind::Async) ||
            check(TokenKind::Init) ||
            check(TokenKind::Deinit)) {

            auto method = parseFunctionDeclaration();

            if (method) {
                declaration->methods.push_back(method);
            }

            continue;
        }

        if (check(TokenKind::Static)) {
            declaration->staticMembers.push_back(parseStaticMember());
            continue;
        }

        error(current(),
              "expected member declaration inside class");

        synchronizeDeclaration();
    }

    consume(TokenKind::RightBrace, "expected '}' after class declaration");

    return declaration;
}

DeclarationPtr Parser::parseProtocolDeclaration() {
    SourceLocation location = current().location;

    consume(TokenKind::Protocol, "expected 'protocol'");

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected protocol name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<ProtocolDeclaration>(name, location);

    if (!consume(
            TokenKind::LeftBrace,
            "expected '{' after protocol name")) {

        synchronizeDeclaration();
        return declaration;
    }

    while (!isAtEnd() && !check(TokenKind::RightBrace)) {
        if (check(TokenKind::Func) ||
            check(TokenKind::Async)) {

            auto requirement = parseFunctionDeclaration();

            if (requirement) {
                declaration->requirements.push_back(requirement);
            }

            continue;
        }

        if (check(TokenKind::Var) ||
            check(TokenKind::Let)) {

            auto property = parseVariableDeclaration();

            if (property) {
                declaration->properties.push_back(property);
            }

            continue;
        }

        error(current(),
              "expected function or property requirement inside protocol");

        synchronizeDeclaration();
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after protocol declaration"
    );

    return declaration;
}

DeclarationPtr Parser::parseEnumDeclaration() {
    SourceLocation location = current().location;

    consume(TokenKind::Enum, "expected 'enum'");

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected enum name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<EnumDeclaration>(name, location);

    if (!consume(
            TokenKind::LeftBrace,
            "expected '{' after enum name")) {

        synchronizeDeclaration();
        return declaration;
    }

    while (!isAtEnd() && !check(TokenKind::RightBrace)) {
        if (!check(TokenKind::Identifier)) {
            error(current(), "expected enum case name");
            synchronizeDeclaration();
            continue;
        }

        EnumCase enumCase;
        enumCase.location = current().location;
        enumCase.name = current().text;
        advance();

        if (match(TokenKind::LeftParen)) {
            while (!isAtEnd() &&
                   !check(TokenKind::RightParen)) {

                auto type = parseType();

                if (type) {
                    enumCase.associatedValues.push_back(type);
                } else {
                    synchronizeDeclaration();
                    break;
                }

                if (!match(TokenKind::Comma)) {
                    break;
                }
            }

            consume(
                TokenKind::RightParen,
                "expected ')' after enum associated values"
            );
        }

        declaration->cases.push_back(std::move(enumCase));

        consumeOptional(TokenKind::Comma);
        consumeOptional(TokenKind::Semicolon);
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after enum declaration"
    );

    return declaration;
}

DeclarationPtr Parser::parseFunctionDeclaration() {
    SourceLocation location = current().location;

    bool isAsync = match(TokenKind::Async);

    consume(TokenKind::Func, "expected 'func'");

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected function name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<FunctionDeclaration>(name, location);

    declaration->isAsync = isAsync;

    if (match(TokenKind::LeftParen)) {
        if (!check(TokenKind::RightParen)) {
            do {
                FunctionParameter parameter;

                parameter.location = current().location;

                if (!check(TokenKind::Identifier)) {
                    error(
                        current(),
                        "expected parameter name"
                    );

                    synchronizeDeclaration();
                    break;
                }

                parameter.name = current().text;
                advance();

                if (match(TokenKind::Colon)) {
                    parameter.type = parseType();
                } else {
                    error(
                        current(),
                        "expected ':' followed by parameter type"
                    );
                }

                if (match(TokenKind::Equal)) {
                    parameter.defaultValue =
                        parseExpression();
                }

                declaration->parameters.push_back(
                    std::move(parameter)
                );

            } while (match(TokenKind::Comma));
        }

        consume(
            TokenKind::RightParen,
            "expected ')' after function parameters"
        );
    }

    if (match(TokenKind::Throws)) {
        declaration->throws = true;
    }

    if (match(TokenKind::Arrow)) {
        declaration->returnType = parseType();
    } else if (match(TokenKind::Colon)) {
        declaration->returnType = parseType();
    }

    if (check(TokenKind::LeftBrace)) {
        declaration->body = parseBlockStatement();
    } else {
        consumeOptional(TokenKind::Semicolon);
    }

    return declaration;
}

DeclarationPtr Parser::parseVariableDeclaration() {
    SourceLocation location = current().location;

    bool mutableValue = check(TokenKind::Var);

    if (mutableValue) {
        consume(TokenKind::Var, "expected 'var'");
    } else {
        consume(TokenKind::Let, "expected 'let'");
    }

    if (!check(TokenKind::Identifier)) {
        error(current(), "expected variable name");
        return nullptr;
    }

    std::string name = current().text;
    advance();

    auto declaration =
        std::make_shared<VariableDeclaration>(
            name,
            mutableValue,
            location
        );

    if (match(TokenKind::Colon)) {
        declaration->type = parseType();
    }

    if (match(TokenKind::Equal)) {
        declaration->initializer = parseExpression();
    }

    if (!declaration->type && !declaration->initializer) {
        error(
            current(),
            "variable declaration requires a type or initializer"
        );
    }

    consumeOptional(TokenKind::Semicolon);

    return declaration;
}

std::shared_ptr<ASTNode> Parser::parseStaticMember() {
    SourceLocation location = current().location;

    consume(TokenKind::Static, "expected 'static'");

    if (check(TokenKind::Var) || check(TokenKind::Let)) {
        auto declaration = parseVariableDeclaration();

        if (declaration) {
            declaration->isStatic = true;
        }

        return declaration;
    }

    if (check(TokenKind::Func) || check(TokenKind::Async)) {
        auto declaration = parseFunctionDeclaration();

        if (declaration) {
            declaration->isStatic = true;
        }

        return declaration;
    }

    error(
        current(),
        "expected variable or function declaration after 'static'"
    );

    return nullptr;
}

void Parser::synchronizeDeclaration() {
    while (!isAtEnd()) {
        if (check(TokenKind::RightBrace)) {
            return;
        }

        if (check(TokenKind::Struct) ||
            check(TokenKind::Class) ||
            check(TokenKind::Protocol) ||
            check(TokenKind::Enum) ||
            check(TokenKind::Func) ||
            check(TokenKind::Async) ||
            check(TokenKind::Import) ||
            check(TokenKind::Let) ||
            check(TokenKind::Var)) {

            return;
        }

        if (match(TokenKind::Semicolon)) {
            return;
        }

        advance();
    }
}

} // namespace Parser
} // namespace Shift