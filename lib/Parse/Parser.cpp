#include "Parser.h"

#include <sstream>
#include <utility>

namespace Shift {

Parser::Parser(std::vector<Token> tokens)
    : tokens(std::move(tokens)) {
}

std::unique_ptr<Program> Parser::parse() {
    current = 0;
    errors.clear();

    auto program = std::make_unique<Program>();

    while (!isAtEnd()) {
        try {
            DeclarationPtr declaration = parseDeclaration();

            if (declaration) {
                program->declarations.push_back(
                    std::move(declaration)
                );
            }
        } catch (...) {
            synchronize();
        }
    }

    return program;
}

const std::vector<std::string>& Parser::diagnostics() const {
    return errors;
}

bool Parser::hasErrors() const {
    return !errors.empty();
}

const Token& Parser::peek(std::size_t distance) const {
    static Token eof;

    if (current + distance >= tokens.size()) {
        return eof;
    }

    return tokens[current + distance];
}

const Token& Parser::previous() const {
    if (current == 0) {
        return peek();
    }

    return tokens[current - 1];
}

bool Parser::isAtEnd() const {
    return peek().kind == TokenKind::EndOfFile;
}

bool Parser::check(TokenKind kind) const {
    return peek().kind == kind;
}

bool Parser::checkNext(TokenKind kind) const {
    return peek(1).kind == kind;
}

const Token& Parser::advance() {
    if (!isAtEnd()) {
        ++current;
    }

    return previous();
}

bool Parser::match(TokenKind kind) {
    if (!check(kind)) {
        return false;
    }

    advance();
    return true;
}

const Token& Parser::consume(TokenKind kind,
                             const std::string& message) {
    if (check(kind)) {
        return advance();
    }

    error(peek(), message);

    return peek();
}

void Parser::error(const Token& token,
                   const std::string& message) {
    std::ostringstream stream;

    stream << "parser error at "
           << token.location.line
           << ":"
           << token.location.column
           << ": "
           << message;

    if (!token.text.empty()) {
        stream << " near '" << token.text << "'";
    }

    errors.push_back(stream.str());
}

void Parser::synchronize() {
    if (!isAtEnd()) {
        advance();
    }

    while (!isAtEnd()) {
        if (previous().kind == TokenKind::RightBrace) {
            return;
        }

        switch (peek().kind) {
            case TokenKind::Struct:
            case TokenKind::Class:
            case TokenKind::Protocol:
            case TokenKind::Enum:
            case TokenKind::Func:
            case TokenKind::Import:
            case TokenKind::Let:
            case TokenKind::Var:
                return;

            default:
                advance();
                break;
        }
    }
}

DeclarationPtr Parser::parseDeclaration() {
    if (match(TokenKind::Import)) {
        --current;
        return parseImport();
    }

    if (match(TokenKind::Struct)) {
        --current;
        return parseStruct();
    }

    if (match(TokenKind::Class)) {
        --current;
        return parseClass();
    }

    if (match(TokenKind::Protocol)) {
        --current;
        return parseProtocol();
    }

    if (match(TokenKind::Enum)) {
        --current;
        return parseEnum();
    }

    if (check(TokenKind::Async) ||
        check(TokenKind::Func)) {
        return parseFunction();
    }

    if (check(TokenKind::Let) ||
        check(TokenKind::Var)) {
        return parseVariableDeclaration();
    }

    error(
        peek(),
        "expected a declaration"
    );

    synchronize();
    return nullptr;
}

DeclarationPtr Parser::parseImport() {
    const Token& start = consume(
        TokenKind::Import,
        "expected 'import'"
    );

    auto declaration =
        std::make_unique<ImportDeclaration>();

    declaration->location = start.location;

    std::string name = parseIdentifier(
        "expected module name after 'import'"
    );

    if (!name.empty()) {
        declaration->path.push_back(name);
    }

    while (match(TokenKind::Dot)) {
        declaration->path.push_back(
            parseIdentifier("expected name after '.'")
        );
    }

    match(TokenKind::Semicolon);

    return declaration;
}

DeclarationPtr Parser::parseStruct() {
    const Token& start = consume(
        TokenKind::Struct,
        "expected 'struct'"
    );

    auto declaration =
        std::make_unique<StructDeclaration>();

    declaration->location = start.location;

    declaration->name = parseIdentifier(
        "expected struct name"
    );

    consume(
        TokenKind::LeftBrace,
        "expected '{' after struct name"
    );

    while (!isAtEnd() &&
           !check(TokenKind::RightBrace)) {

        if (check(TokenKind::Let) ||
            check(TokenKind::Var)) {

            declaration->members.push_back(
                parseVariableDeclaration()
            );

            continue;
        }

        if (check(TokenKind::Func) ||
            check(TokenKind::Async)) {

            declaration->members.push_back(
                parseFunction()
            );

            continue;
        }

        error(
            peek(),
            "expected struct member"
        );

        synchronize();
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after struct body"
    );

    return declaration;
}

DeclarationPtr Parser::parseClass() {
    const Token& start = consume(
        TokenKind::Class,
        "expected 'class'"
    );

    auto declaration =
        std::make_unique<ClassDeclaration>();

    declaration->location = start.location;

    declaration->name = parseIdentifier(
        "expected class name"
    );

    consume(
        TokenKind::LeftBrace,
        "expected '{' after class name"
    );

    while (!isAtEnd() &&
           !check(TokenKind::RightBrace)) {

        if (check(TokenKind::Let) ||
            check(TokenKind::Var)) {

            declaration->members.push_back(
                parseVariableDeclaration()
            );

            continue;
        }

        if (check(TokenKind::Func) ||
            check(TokenKind::Async)) {

            declaration->members.push_back(
                parseFunction()
            );

            continue;
        }

        error(
            peek(),
            "expected class member"
        );

        synchronize();
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after class body"
    );

    return declaration;
}

DeclarationPtr Parser::parseProtocol() {
    const Token& start = consume(
        TokenKind::Protocol,
        "expected 'protocol'"
    );

    auto declaration =
        std::make_unique<ProtocolDeclaration>();

    declaration->location = start.location;

    declaration->name = parseIdentifier(
        "expected protocol name"
    );

    consume(
        TokenKind::LeftBrace,
        "expected '{' after protocol name"
    );

    while (!isAtEnd() &&
           !check(TokenKind::RightBrace)) {

        if (check(TokenKind::Func) ||
            check(TokenKind::Async)) {

            declaration->members.push_back(
                parseFunction()
            );

            continue;
        }

        if (check(TokenKind::Let) ||
            check(TokenKind::Var)) {

            declaration->members.push_back(
                parseVariableDeclaration()
            );

            continue;
        }

        error(
            peek(),
            "expected protocol member"
        );

        synchronize();
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after protocol body"
    );

    return declaration;
}

DeclarationPtr Parser::parseEnum() {
    const Token& start = consume(
        TokenKind::Enum,
        "expected 'enum'"
    );

    auto declaration =
        std::make_unique<EnumDeclaration>();

    declaration->location = start.location;

    declaration->name = parseIdentifier(
        "expected enum name"
    );

    consume(
        TokenKind::LeftBrace,
        "expected '{' after enum name"
    );

    while (!isAtEnd() &&
           !check(TokenKind::RightBrace)) {

        std::string caseName;

        if (check(TokenKind::Identifier)) {
            caseName = advance().text;
        } else {
            error(
                peek(),
                "expected enum case"
            );
            synchronize();
            continue;
        }

        EnumCase enumCase;
        enumCase.name = std::move(caseName);

        if (match(TokenKind::LeftParen)) {
            while (!isAtEnd() &&
                   !check(TokenKind::RightParen)) {

                enumCase.associatedTypes.push_back(
                    parseTypeName()
                );

                if (!match(TokenKind::Comma)) {
                    break;
                }
            }

            consume(
                TokenKind::RightParen,
                "expected ')' after enum associated values"
            );
        }

        declaration->cases.push_back(
            std::move(enumCase)
        );

        match(TokenKind::Comma);
        match(TokenKind::Semicolon);
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after enum body"
    );

    return declaration;
}

DeclarationPtr Parser::parseFunction() {
    bool async = match(TokenKind::Async);

    const Token& start = consume(
        TokenKind::Func,
        "expected 'func'"
    );

    auto declaration =
        std::make_unique<FunctionDeclaration>();

    declaration->location = start.location;
    declaration->isAsync = async;

    declaration->name = parseIdentifier(
        "expected function name"
    );

    consume(
        TokenKind::LeftParen,
        "expected '(' after function name"
    );

    if (!check(TokenKind::RightParen)) {
        do {
            FunctionParameter parameter;

            parameter.name = parseIdentifier(
                "expected parameter name"
            );

            if (match(TokenKind::Colon)) {
                parameter.typeName = parseTypeName();
            }

            declaration->parameters.push_back(
                std::move(parameter)
            );

        } while (match(TokenKind::Comma));
    }

    consume(
        TokenKind::RightParen,
        "expected ')' after parameters"
    );

    if (match(TokenKind::Arrow)) {
        declaration->returnType = parseTypeName();
    }

    if (match(TokenKind::Throws)) {
        declaration->throwsErrors = true;
    }

    declaration->body = parseBlock();

    return declaration;
}

DeclarationPtr Parser::parseVariableDeclaration() {
    const Token& start = peek();

    bool mutableValue = false;

    if (match(TokenKind::Var)) {
        mutableValue = true;
    } else {
        consume(
            TokenKind::Let,
            "expected 'let' or 'var'"
        );
    }

    auto declaration =
        std::make_unique<VariableDeclaration>();

    declaration->location = start.location;
    declaration->mutableValue = mutableValue;

    declaration->name = parseIdentifier(
        "expected variable name"
    );

    if (match(TokenKind::Colon)) {
        declaration->typeName = parseTypeName();
    }

    if (match(TokenKind::Equal)) {
        declaration->initializer = parseExpression();
    }

    match(TokenKind::Semicolon);

    return declaration;
}

std::unique_ptr<BlockStatement> Parser::parseBlock() {
    const Token& start = consume(
        TokenKind::LeftBrace,
        "expected '{'"
    );

    auto block =
        std::make_unique<BlockStatement>();

    block->location = start.location;

    while (!isAtEnd() &&
           !check(TokenKind::RightBrace)) {

        StatementPtr statement = parseStatement();

        if (statement) {
            block->statements.push_back(
                std::move(statement)
            );
        }
    }

    consume(
        TokenKind::RightBrace,
        "expected '}' after block"
    );

    return block;
}

StatementPtr Parser::parseStatement() {
    if (match(TokenKind::If)) {
        --current;
        return parseIf();
    }

    if (match(TokenKind::While)) {
        --current;
        return parseWhile();
    }

    if (match(TokenKind::For)) {
        --current;
        return parseFor();
    }

    if (match(TokenKind::Repeat)) {
        --current;
        return parseRepeat();
    }

    if (match(TokenKind::Guard)) {
        --current;
        return parseGuard();
    }

    if (match(TokenKind::Return)) {
        --current;
        return parseReturn();
    }

    if (match(TokenKind::Break)) {
        auto statement =
            std::make_unique<BreakStatement>();

        statement->location = previous().location;

        match(TokenKind::Semicolon);

        return statement;
    }

    if (match(TokenKind::Continue)) {
        auto statement =
            std::make_unique<ContinueStatement>();

        statement->location = previous().location;

        match(TokenKind::Semicolon);

        return statement;
    }

    if (match(TokenKind::Yield)) {
        --current;
        return parseYield();
    }

    if (match(TokenKind::Defer)) {
        --current;
        return parseDefer();
    }

    if (match(TokenKind::Try)) {
        --current;
        return parseTryStatement();
    }

    if (check(TokenKind::Let) ||
        check(TokenKind::Var)) {

        DeclarationPtr declaration =
            parseVariableDeclaration();

        auto statement =
            std::make_unique<ExpressionStatement>();

        statement->location = declaration->location;

        return statement;
    }

    ExpressionPtr expression = parseExpression();

    auto statement =
        std::make_unique<ExpressionStatement>();

    statement->location = expression->location;
    statement->expression = std::move(expression);

    match(TokenKind::Semicolon);

    return statement;
}

StatementPtr Parser::parseIf() {
    const Token& start = consume(
        TokenKind::If,
        "expected 'if'"
    );

    auto statement =
        std::make_unique<IfStatement>();

    statement->location = start.location;
    statement->condition = parseExpression();

    match(TokenKind::Then);

    statement->body = parseBlock();

    while (match(TokenKind::ElseIf)) {
        ExpressionPtr condition = parseExpression();

        match(TokenKind::Then);

        auto body = parseBlock();

        statement->elseIfs.emplace_back(
            std::move(condition),
            std::move(body)
        );
    }

    if (match(TokenKind::Else)) {
        statement->elseBody = parseBlock();
    }

    return statement;
}

StatementPtr Parser::parseWhile() {
    const Token& start = consume(
        TokenKind::While,
        "expected 'while'"
    );

    auto statement =
        std::make_unique<WhileStatement>();

    statement->location = start.location;
    statement->condition = parseExpression();
    statement->body = parseBlock();

    return statement;
}

StatementPtr Parser::parseFor() {
    const Token& start = consume(
        TokenKind::For,
        "expected 'for'"
    );

    auto statement =
        std::make_unique<ForStatement>();

    statement->location = start.location;

    statement->variable = parseIdentifier(
        "expected loop variable"
    );

    consume(
        TokenKind::In,
        "expected 'in' in for loop"
    );

    statement->sequence = parseExpression();
    statement->body = parseBlock();

    return statement;
}

StatementPtr Parser::parseRepeat() {
    const Token& start = consume(
        TokenKind::Repeat,
        "expected 'repeat'"
    );

    auto statement =
        std::make_unique<RepeatStatement>();

    statement->location = start.location;
    statement->body = parseBlock();

    if (match(TokenKind::While)) {
        statement->condition = parseExpression();
    }

    return statement;
}

StatementPtr Parser::parseGuard() {
    const Token& start = consume(
        TokenKind::Guard,
        "expected 'guard'"
    );

    auto statement =
        std::make_unique<GuardStatement>();

    statement->location = start.location;
    statement->condition = parseExpression();

    match(TokenKind::Else);

    statement->body = parseBlock();

    return statement;
}

StatementPtr Parser::parseReturn() {
    const Token& start = consume(
        TokenKind::Return,
        "expected 'return'"
    );

    auto statement =
        std::make_unique<ReturnStatement>();

    statement->location = start.location;

    if (!check(TokenKind::RightBrace) &&
        !check(TokenKind::Semicolon) &&
        !isAtEnd()) {

        statement->value = parseExpression();
    }

    match(TokenKind::Semicolon);

    return statement;
}

StatementPtr Parser::parseYield() {
    const Token& start = consume(
        TokenKind::Yield,
        "expected 'yield'"
    );

    auto statement =
        std::make_unique<YieldStatement>();

    statement->location = start.location;

    if (!check(TokenKind::RightBrace)) {
        statement->value = parseExpression();
    }

    match(TokenKind::Semicolon);

    return statement;
}

StatementPtr Parser::parseDefer() {
    const Token& start = consume(
        TokenKind::Defer,
        "expected 'defer'"
    );

    auto statement =
        std::make_unique<DeferStatement>();

    statement->location = start.location;

    auto block = parseBlock();

    statement->body =
        std::move(block->statements);

    return statement;
}

StatementPtr Parser::parseTryStatement() {
    const Token& start = consume(
        TokenKind::Try,
        "expected 'try'"
    );

    auto statement =
        std::make_unique<TryStatement>();

    statement->location = start.location;
    statement->expression = parseExpression();

    match(TokenKind::Semicolon);

    return statement;
}

ExpressionPtr Parser::parseExpression() {
    return parseAssignment();
}

ExpressionPtr Parser::parseAssignment() {
    ExpressionPtr left = parseLogicalOr();

    if (isAssignmentOperator(peek().kind)) {
        TokenKind operation = advance().kind;

        ExpressionPtr right = parseAssignment();

        auto expression =
            std::make_unique<AssignmentExpression>();

        expression->location = left->location;
        expression->left = std::move(left);
        expression->operation = operation;
        expression->right = std::move(right);

        return expression;
    }

    return left;
}

ExpressionPtr Parser::parseLogicalOr() {
    ExpressionPtr expression =
        parseLogicalAnd();

    while (match(TokenKind::OrOr)) {
        TokenKind operation = previous().kind;

        ExpressionPtr right =
            parseLogicalAnd();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseLogicalAnd() {
    ExpressionPtr expression =
        parseBitwiseOr();

    while (match(TokenKind::AndAnd)) {
        TokenKind operation = previous().kind;

        ExpressionPtr right =
            parseBitwiseOr();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseBitwiseOr() {
    ExpressionPtr expression =
        parseBitwiseXor();

    while (match(TokenKind::BitOr)) {
        TokenKind operation = previous().kind;

        ExpressionPtr right =
            parseBitwiseXor();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseBitwiseXor() {
    ExpressionPtr expression =
        parseBitwiseAnd();

    while (match(TokenKind::BitXor)) {
        TokenKind operation = previous().kind;

        ExpressionPtr right =
            parseBitwiseAnd();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseBitwiseAnd() {
    ExpressionPtr expression =
        parseEquality();

    while (match(TokenKind::BitAnd)) {
        TokenKind operation = previous().kind;

        ExpressionPtr right =
            parseEquality();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseEquality() {
    ExpressionPtr expression =
        parseComparison();

    while (check(TokenKind::EqualEqual) ||
           check(TokenKind::NotEqual)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseComparison();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseComparison() {
    ExpressionPtr expression =
        parseShift();

    while (check(TokenKind::Less) ||
           check(TokenKind::LessEqual) ||
           check(TokenKind::Greater) ||
           check(TokenKind::GreaterEqual)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseShift();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseShift() {
    ExpressionPtr expression =
        parseRange();

    while (check(TokenKind::ShiftLeft) ||
           check(TokenKind::ShiftRight)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseRange();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseRange() {
    ExpressionPtr expression =
        parseTerm();

    while (check(TokenKind::DotDot) ||
           check(TokenKind::DotDotLess)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseTerm();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseTerm() {
    ExpressionPtr expression =
        parseFactor();

    while (check(TokenKind::Plus) ||
           check(TokenKind::Minus)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseFactor();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseFactor() {
    ExpressionPtr expression =
        parseUnary();

    while (check(TokenKind::Star) ||
           check(TokenKind::Slash) ||
           check(TokenKind::Percent)) {

        TokenKind operation = advance().kind;

        ExpressionPtr right =
            parseUnary();

        auto binary =
            std::make_unique<BinaryExpression>();

        binary->location = expression->location;
        binary->left = std::move(expression);
        binary->operation = operation;
        binary->right = std::move(right);

        expression = std::move(binary);
    }

    return expression;
}

ExpressionPtr Parser::parseUnary() {
    if (isUnaryOperator(peek().kind)) {
        const Token& token = advance();

        auto expression =
            std::make_unique<UnaryExpression>();

        expression->location = token.location;
        expression->operation = token.kind;
        expression->operand = parseUnary();

        return expression;
    }

    return parsePostfix();
}

ExpressionPtr Parser::parsePostfix() {
    ExpressionPtr expression =
        parsePrimary();

    while (true) {
        if (match(TokenKind::LeftParen)) {
            auto call =
                std::make_unique<CallExpression>();

            call->location = expression->location;
            call->callee = std::move(expression);

            if (!check(TokenKind::RightParen)) {
                call->arguments = parseArguments();
            }

            consume(
                TokenKind::RightParen,
                "expected ')' after arguments"
            );

            expression = std::move(call);
            continue;
        }

        if (match(TokenKind::Dot)) {
            std::string member =
                parseIdentifier(
                    "expected member name after '.'"
                );

            auto access =
                std::make_unique<MemberExpression>();

            access->location = expression->location;
            access->object = std::move(expression);
            access->member = std::move(member);

            expression = std::move(access);
            continue;
        }

        if (match(TokenKind::PlusPlus)) {
            auto unary =
                std::make_unique<UnaryExpression>();

            unary->location = expression->location;
            unary->operation = TokenKind::PlusPlus;
            unary->operand = std::move(expression);

            expression = std::move(unary);
            continue;
        }

        if (match(TokenKind::MinusMinus)) {
            auto unary =
                std::make_unique<UnaryExpression>();

            unary->location = expression->location;
            unary->operation = TokenKind::MinusMinus;
            unary->operand = std::move(expression);

            expression = std::move(unary);
            continue;
        }

        break;
    }

    return expression;
}

ExpressionPtr Parser::parsePrimary() {
    if (check(TokenKind::IntegerLiteral) ||
        check(TokenKind::FloatLiteral) ||
        check(TokenKind::StringLiteral) ||
        check(TokenKind::CharacterLiteral) ||
        check(TokenKind::True) ||
        check(TokenKind::False) ||
        check(TokenKind::Nil)) {

        const Token& token = advance();

        auto literal =
            std::make_unique<LiteralExpression>();

        literal->location = token.location;
        literal->kind = token.kind;
        literal->value = token.text;

        return literal;
    }

    if (check(TokenKind::Identifier) ||
        check(TokenKind::BuiltinPrint) ||
        check(TokenKind::BuiltinErrorMessage) ||
        check(TokenKind::BuiltinPanic) ||
        check(TokenKind::BuiltinAbs) ||
        check(TokenKind::BuiltinMin) ||
        check(TokenKind::BuiltinMax) ||
        check(TokenKind::BuiltinStride) ||
        check(TokenKind::BuiltinDump) ||
        check(TokenKind::BuiltinRead) ||
        check(TokenKind::BuiltinInput) ||
        check(TokenKind::BuiltinLength) ||
        check(TokenKind::BuiltinType) ||
        check(TokenKind::BuiltinSort) ||
        check(TokenKind::BuiltinReverse) ||
        check(TokenKind::BuiltinExit) ||
        check(TokenKind::Self)) {

        const Token& token = advance();

        auto identifier =
            std::make_unique<IdentifierExpression>();

        identifier->location = token.location;
        identifier->name = token.text;

        return identifier;
    }

    if (match(TokenKind::LeftParen)) {
        SourceLocation start = previous().location;

        ExpressionPtr expression =
            parseExpression();

        consume(
            TokenKind::RightParen,
            "expected ')' after expression"
        );

        auto group =
            std::make_unique<GroupExpression>();

        group->location = start;
        group->expression = std::move(expression);

        return group;
    }

    if (match(TokenKind::LeftBracket)) {
        SourceLocation start = previous().location;

        auto array =
            std::make_unique<ArrayExpression>();

        array->location = start;

        if (!check(TokenKind::RightBracket)) {
            do {
                array->elements.push_back(
                    parseExpression()
                );
            } while (match(TokenKind::Comma));
        }

        consume(
            TokenKind::RightBracket,
            "expected ']' after array"
        );

        return array;
    }

    error(
        peek(),
        "expected expression"
    );

    const Token& token = advance();

    auto fallback =
        std::make_unique<IdentifierExpression>();

    fallback->location = token.location;
    fallback->name = "<error>";

    return fallback;
}

std::vector<ExpressionPtr> Parser::parseArguments() {
    std::vector<ExpressionPtr> arguments;

    do {
        arguments.push_back(
            parseExpression()
        );
    } while (match(TokenKind::Comma));

    return arguments;
}

std::string Parser::parseTypeName() {
    std::string result;

    if (check(TokenKind::Identifier) ||
        check(TokenKind::Int) ||
        check(TokenKind::Num) ||
        check(TokenKind::String) ||
        check(TokenKind::Bool) ||
        check(TokenKind::Any) ||
        check(TokenKind::Type)) {

        result = advance().text;
    } else {
        error(
            peek(),
            "expected type name"
        );

        return "";
    }

    while (match(TokenKind::Dot)) {
        result += ".";
        result += parseIdentifier(
            "expected type name after '.'"
        );
    }

    if (match(TokenKind::LeftBracket)) {
        consume(
            TokenKind::RightBracket,
            "expected ']' in array type"
        );

        result += "[]";
    }

    if (match(TokenKind::Question)) {
        result += "?";
    }

    return result;
}

std::string Parser::parseIdentifier(
    const std::string& message) {

    if (check(TokenKind::Identifier)) {
        return advance().text;
    }

    error(peek(), message);

    return "";
}

bool Parser::isAssignmentOperator(
    TokenKind kind) const {

    switch (kind) {
        case TokenKind::Equal:
        case TokenKind::PlusEqual:
        case TokenKind::MinusEqual:
        case TokenKind::StarEqual:
        case TokenKind::SlashEqual:
        case TokenKind::PercentEqual:
            return true;

        default:
            return false;
    }
}

bool Parser::isUnaryOperator(
    TokenKind kind) const {

    switch (kind) {
        case TokenKind::Minus:
        case TokenKind::Plus:
        case TokenKind::Not:
        case TokenKind::BitNot:
        case TokenKind::PlusPlus:
        case TokenKind::MinusMinus:
            return true;

        default:
            return false;
    }
}

bool Parser::isBinaryOperator(
    TokenKind kind) const {

    return binaryPrecedence(kind) >= 0;
}

int Parser::binaryPrecedence(
    TokenKind kind) const {

    switch (kind) {
        case TokenKind::OrOr:
            return 1;

        case TokenKind::AndAnd:
            return 2;

        case TokenKind::BitOr:
            return 3;

        case TokenKind::BitXor:
            return 4;

        case TokenKind::BitAnd:
            return 5;

        case TokenKind::EqualEqual:
        case TokenKind::NotEqual:
            return 6;

        case TokenKind::Less:
        case TokenKind::LessEqual:
        case TokenKind::Greater:
        case TokenKind::GreaterEqual:
            return 7;

        case TokenKind::ShiftLeft:
        case TokenKind::ShiftRight:
            return 8;

        case TokenKind::Plus:
        case TokenKind::Minus:
            return 9;

        case TokenKind::Star:
        case TokenKind::Slash:
        case TokenKind::Percent:
            return 10;

        default:
            return -1;
    }
}

} // namespace Shift