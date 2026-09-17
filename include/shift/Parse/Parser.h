#ifndef SHIFT_PARSER_H
#define SHIFT_PARSER_H

#include "Lexer.h"

#include <memory>
#include <string>
#include <vector>

namespace Shift {

struct ASTNode {
    SourceLocation location;

    virtual ~ASTNode() = default;
};

using ASTNodePtr = std::unique_ptr<ASTNode>;

struct Expression : ASTNode {
};

using ExpressionPtr = std::unique_ptr<Expression>;

struct Statement : ASTNode {
};

using StatementPtr = std::unique_ptr<Statement>;

struct Declaration : ASTNode {
};

using DeclarationPtr = std::unique_ptr<Declaration>;

struct Program : ASTNode {
    std::vector<DeclarationPtr> declarations;
};

struct IdentifierExpression : Expression {
    std::string name;
};

struct LiteralExpression : Expression {
    TokenKind kind;
    std::string value;
};

struct UnaryExpression : Expression {
    TokenKind operation;
    ExpressionPtr operand;
};

struct BinaryExpression : Expression {
    ExpressionPtr left;
    TokenKind operation;
    ExpressionPtr right;
};

struct AssignmentExpression : Expression {
    ExpressionPtr left;
    TokenKind operation;
    ExpressionPtr right;
};

struct MemberExpression : Expression {
    ExpressionPtr object;
    std::string member;
};

struct CallExpression : Expression {
    ExpressionPtr callee;
    std::vector<ExpressionPtr> arguments;
};

struct ArrayExpression : Expression {
    std::vector<ExpressionPtr> elements;
};

struct GroupExpression : Expression {
    ExpressionPtr expression;
};

struct ExpressionStatement : Statement {
    ExpressionPtr expression;
};

struct VariableDeclaration : Declaration {
    bool mutableValue = false;
    std::string name;
    std::string typeName;
    ExpressionPtr initializer;
};

struct ReturnStatement : Statement {
    ExpressionPtr value;
};

struct BreakStatement : Statement {
};

struct ContinueStatement : Statement {
};

struct YieldStatement : Statement {
    ExpressionPtr value;
};

struct DeferStatement : Statement {
    std::vector<StatementPtr> body;
};

struct BlockStatement : Statement {
    std::vector<StatementPtr> statements;
};

struct IfStatement : Statement {
    ExpressionPtr condition;
    std::unique_ptr<BlockStatement> body;
    std::unique_ptr<BlockStatement> elseBody;
    std::vector<std::pair<ExpressionPtr,
                          std::unique_ptr<BlockStatement>>> elseIfs;
};

struct WhileStatement : Statement {
    ExpressionPtr condition;
    std::unique_ptr<BlockStatement> body;
};

struct ForStatement : Statement {
    std::string variable;
    ExpressionPtr sequence;
    std::unique_ptr<BlockStatement> body;
};

struct RepeatStatement : Statement {
    std::unique_ptr<BlockStatement> body;
    ExpressionPtr condition;
};

struct GuardStatement : Statement {
    ExpressionPtr condition;
    std::unique_ptr<BlockStatement> body;
};

struct TryStatement : Statement {
    ExpressionPtr expression;
};

struct FunctionParameter {
    std::string name;
    std::string typeName;
};

struct FunctionDeclaration : Declaration {
    std::string name;
    std::vector<FunctionParameter> parameters;
    std::string returnType;
    bool isAsync = false;
    bool throwsErrors = false;
    std::unique_ptr<BlockStatement> body;
};

struct StructDeclaration : Declaration {
    std::string name;
    std::vector<DeclarationPtr> members;
};

struct ClassDeclaration : Declaration {
    std::string name;
    std::vector<DeclarationPtr> members;
};

struct ProtocolDeclaration : Declaration {
    std::string name;
    std::vector<DeclarationPtr> members;
};

struct EnumCase {
    std::string name;
    std::vector<std::string> associatedTypes;
};

struct EnumDeclaration : Declaration {
    std::string name;
    std::vector<EnumCase> cases;
};

struct ImportDeclaration : Declaration {
    std::vector<std::string> path;
};

class Parser {
public:
    explicit Parser(std::vector<Token> tokens);

    std::unique_ptr<Program> parse();

    const std::vector<std::string>& diagnostics() const;

    bool hasErrors() const;

private:
    std::vector<Token> tokens;
    std::size_t current = 0;
    std::vector<std::string> errors;

    const Token& peek(std::size_t distance = 0) const;
    const Token& previous() const;

    bool isAtEnd() const;
    bool check(TokenKind kind) const;
    bool checkNext(TokenKind kind) const;

    const Token& advance();
    bool match(TokenKind kind);

    const Token& consume(TokenKind kind,
                         const std::string& message);

    void error(const Token& token,
               const std::string& message);

    void synchronize();

    DeclarationPtr parseDeclaration();

    DeclarationPtr parseImport();
    DeclarationPtr parseStruct();
    DeclarationPtr parseClass();
    DeclarationPtr parseProtocol();
    DeclarationPtr parseEnum();
    DeclarationPtr parseFunction();
    DeclarationPtr parseVariableDeclaration();

    std::unique_ptr<BlockStatement> parseBlock();

    StatementPtr parseStatement();

    StatementPtr parseIf();
    StatementPtr parseWhile();
    StatementPtr parseFor();
    StatementPtr parseRepeat();
    StatementPtr parseGuard();
    StatementPtr parseReturn();
    StatementPtr parseBreak();
    StatementPtr parseContinue();
    StatementPtr parseYield();
    StatementPtr parseDefer();
    StatementPtr parseTryStatement();

    ExpressionPtr parseExpression();

    ExpressionPtr parseAssignment();
    ExpressionPtr parseLogicalOr();
    ExpressionPtr parseLogicalAnd();
    ExpressionPtr parseBitwiseOr();
    ExpressionPtr parseBitwiseXor();
    ExpressionPtr parseBitwiseAnd();
    ExpressionPtr parseEquality();
    ExpressionPtr parseComparison();
    ExpressionPtr parseShift();
    ExpressionPtr parseRange();
    ExpressionPtr parseTerm();
    ExpressionPtr parseFactor();
    ExpressionPtr parseUnary();
    ExpressionPtr parsePostfix();
    ExpressionPtr parsePrimary();

    std::vector<ExpressionPtr> parseArguments();

    std::string parseTypeName();

    std::string parseIdentifier(
        const std::string& message);

    bool isAssignmentOperator(TokenKind kind) const;
    bool isUnaryOperator(TokenKind kind) const;
    bool isBinaryOperator(TokenKind kind) const;

    int binaryPrecedence(TokenKind kind) const;
};

} // namespace Shift

#endif