//
// Shift Parser
// Single-file recursive-descent parser for the Shift language.
//
// The parser consumes the token model used by the Shift lexer and builds
// a Swift-style syntax tree. It keeps parsing after recoverable errors,
// allowing the compiler to report multiple diagnostics in one invocation.
//

import Foundation

public enum ParserSeverity {
    case error
    case warning
}

public struct ParserDiagnostic: CustomStringConvertible {
    public let severity: ParserSeverity
    public let message: String
    public let line: Int
    public let column: Int

    public var description: String {
        "\(severity): \(message) at \(line):\(column)"
    }
}

public enum TokenKind: Equatable {
    case identifier
    case integerLiteral
    case floatingLiteral
    case stringLiteral
    case characterLiteral
    case eof
    case unknown

    case ifKeyword
    case elseKeyword
    case trueKeyword
    case falseKeyword
    case whileKeyword
    case doKeyword
    case placeholderKeyword
    case intKeyword
    case varKeyword
    case letKeyword
    case deferKeyword
    case numKeyword
    case enumKeyword
    case initKeyword
    case thenKeyword
    case staticKeyword
    case outputKeyword
    case stringKeyword
    case callKeyword
    case moveKeyword
    case errorKeyword
    case fileIDDirective
    case fileDirective
    case apiDirective
    case repoDirective
    case webLinkDirective
    case databaseDirective
    case structKeyword
    case funcKeyword
    case panicKeyword
    case playAudioIDDirective
    case playAudioDirective
    case audioIDDirective
    case sinkKeyword
    case rangeKeyword
    case loopKeyword
    case deleteKeyword
    case cutKeyword
    case endKeyword
    case connectKeyword
    case dataKeyword
    case getDataKeyword
    case createDataKeyword
    case bytesKeyword
    case controlKeyword
    case sectionKeyword
    case changeKeyword
    case backupKeyword
    case passKeyword
    case binaryKeyword
    case osKeyword
    case kernelKeyword
    case playKeyword
    case destroyKeyword
    case fileNameKeyword
    case whereKeyword
    case whenKeyword
    case singleKeyword
    case dontKeyword
    case useKeyword
    case insteadKeyword
    case ofKeyword
    case blockKeyword
    case shrinkKeyword
    case mathKeyword
    case lineKeyword
    case baseKeyword
    case messageKeyword
    case nilKeyword

    case importKeyword
    case publicKeyword
    case privateKeyword
    case internalKeyword
    case fileprivateKeyword
    case openKeyword
    case finalKeyword
    case classKeyword
    case actorKeyword
    case protocolKeyword
    case extensionKeyword
    case typealiasKeyword
    case associatedtypeKeyword
    case packageKeyword
    case asyncKeyword
    case awaitKeyword
    case throwsKeyword
    case rethrowsKeyword
    case `return`Keyword
    case breakKeyword
    case continueKeyword
    case fallthroughKeyword
    case switchKeyword
    case caseKeyword
    case defaultKeyword
    case guardKeyword
    case repeatKeyword
    case inKeyword
    case isKeyword
    case asKeyword
    case asQuestionKeyword
    case asExclamationKeyword
    case selfKeyword
    case SelfKeyword
    case superKeyword
    case initAccessorKeyword
    case deinitKeyword
    case getKeyword
    case setKeyword
    case willSetKeyword
    case didSetKeyword
    case mutatingKeyword
    case nonmutatingKeyword
    case consumingKeyword
    case borrowingKeyword
    case convenienceKeyword
    case requiredKeyword
    case overrideKeyword
    case weakKeyword
    case unownedKeyword
    case lazyKeyword
    case indirectKeyword
    case isolatedKeyword
    case nonisolatedKeyword
    case precedencegroupKeyword
    case infixKeyword
    case prefixKeyword
    case postfixKeyword
    case associativityKeyword
    case leftKeyword
    case rightKeyword
    case higherThanKeyword
    case lowerThanKeyword
    case assignmentKeyword
    case optionalKeyword

    case leftBrace
    case rightBrace
    case leftParen
    case rightParen
    case leftBracket
    case rightBracket
    case dot
    case comma
    case colon
    case semicolon
    case question
    case exclamation
    case at
    case arrow
    case plus
    case minus
    case star
    case slash
    case percent
    case ampersand
    case pipe
    case caret
    case tilde
    case less
    case greater
    case equal
    case doubleEqual
    case notEqual
    case plusEqual
    case minusEqual
    case starEqual
    case slashEqual
    case percentEqual
    case greaterEqual
    case lessEqual
    case approximateEqual
    case logicalAnd
    case logicalOr
    case nullCoalescing
    case range
    case closedRange
    case leftShift
    case rightShift
}

public struct SourceLocation: Equatable {
    public let line: Int
    public let column: Int
    public let offset: Int

    public init(line: Int, column: Int, offset: Int) {
        self.line = line
        self.column = column
        self.offset = offset
    }
}

public struct Token: Equatable {
    public let kind: TokenKind
    public let lexeme: String
    public let location: SourceLocation

    public init(kind: TokenKind, lexeme: String, location: SourceLocation) {
        self.kind = kind
        self.lexeme = lexeme
        self.location = location
    }
}

public indirect enum TypeSyntax: Equatable {
    case named(String)
    case optional(TypeSyntax)
    case array(TypeSyntax)
    case function(parameters: [TypeSyntax], result: TypeSyntax?)
    case tuple([TypeSyntax])
    case inferred
    case void
}

public indirect enum Expression: Equatable {
    case identifier(String)
    case integer(String)
    case floating(String)
    case string(String)
    case character(String)
    case boolean(Bool)
    case nilLiteral
    case unary(operator: TokenKind, operand: Expression)
    case binary(left: Expression, operator: TokenKind, right: Expression)
    case assignment(left: Expression, operator: TokenKind, right: Expression)
    case member(base: Expression, name: String)
    case call(callee: Expression, arguments: [Argument])
    case subscriptExpression(base: Expression, arguments: [Expression])
    case tuple([Expression])
    case array([Expression])
    case range(lower: Expression, upper: Expression, closed: Bool)
    case closure(parameters: [Parameter], body: [Statement])
    case conditional(condition: Expression, thenBranch: Expression, elseBranch: Expression?)
    case cast(Expression, TypeSyntax)
}

public struct Argument: Equatable {
    public let label: String?
    public let value: Expression
}

public struct Parameter: Equatable {
    public let externalName: String?
    public let localName: String
    public let type: TypeSyntax?
    public let defaultValue: Expression?
}

public struct Attribute: Equatable {
    public let name: String
    public let arguments: [Expression]
}

public enum AccessLevel: Equatable {
    case `private`
    case fileprivate
    case internal
    case public
    case open
    case package
}

public enum DeclarationModifier: Equatable {
    case `static`
    case final
    case class
    case actor
    case override
    case required
    case convenience
    case mutating
    case nonmutating
    case consuming
    case borrowing
    case lazy
    case weak
    case unowned
    case isolated
    case nonisolated
    case indirect
}

public struct FunctionDeclaration: Equatable {
    public let name: String
    public let parameters: [Parameter]
    public let returnType: TypeSyntax?
    public let modifiers: [DeclarationModifier]
    public let attributes: [Attribute]
    public let body: [Statement]
}

public struct VariableDeclaration: Equatable {
    public let isLet: Bool
    public let name: String
    public let type: TypeSyntax?
    public let initializer: Expression?
}

public struct StructDeclaration: Equatable {
    public let name: String
    public let members: [Declaration]
}

public struct EnumCase: Equatable {
    public let name: String
    public let associatedValues: [Parameter]
}

public struct EnumDeclaration: Equatable {
    public let name: String
    public let cases: [EnumCase]
}

public struct ImportDeclaration: Equatable {
    public let path: [String]
}

public struct DirectiveDeclaration: Equatable {
    public let kind: TokenKind
    public let arguments: [Expression]
}

public indirect enum Declaration: Equatable {
    case function(FunctionDeclaration)
    case variable(VariableDeclaration)
    case `struct`(StructDeclaration)
    case enumeration(EnumDeclaration)
    case import(ImportDeclaration)
    case directive(DirectiveDeclaration)
    case typealias(name: String, type: TypeSyntax)
    case extensionDecl(name: String, members: [Declaration])
}

public indirect enum Statement: Equatable {
    case declaration(Declaration)
    case expression(Expression)
    case `if`(condition: Expression, thenBody: [Statement], elseBody: [Statement]?)
    case whileLoop(condition: Expression, body: [Statement])
    case doWhile(body: [Statement], condition: Expression)
    case `for`(name: String, sequence: Expression, body: [Statement])
    case loop(body: [Statement])
    case `return`(Expression?)
    case breakStatement
    case continueStatement
    case deferStatement([Statement])
    case guardStatement(condition: Expression, body: [Statement])
    case switchStatement(expression: Expression, cases: [SwitchCase])
    case block([Statement])
    case empty
}

public struct SwitchCase: Equatable {
    public let pattern: Expression?
    public let body: [Statement]
}

public struct SourceFile: Equatable {
    public let declarations: [Declaration]
}

public final class ShiftParser {
    private let tokens: [Token]
    private var index: Int = 0
    public private(set) var diagnostics: [ParserDiagnostic] = []

    public init(tokens: [Token]) {
        self.tokens = tokens
    }

    public func parse() -> SourceFile {
        var declarations: [Declaration] = []
        while !isAtEnd {
            if let declaration = parseDeclaration() {
                declarations.append(declaration)
            } else {
                synchronize()
            }
        }
        return SourceFile(declarations: declarations)
    }

    private var current: Token {
        tokens[min(index, tokens.count - 1)]
    }

    private var previous: Token {
        tokens[max(0, index - 1)]
    }

    private var isAtEnd: Bool {
        current.kind == .eof
    }

    @discardableResult
    private func advance() -> Token {
        let token = current
        if !isAtEnd { index += 1 }
        return token
    }

    private func check(_ kind: TokenKind) -> Bool {
        current.kind == kind
    }

    @discardableResult
    private func match(_ kinds: TokenKind...) -> Bool {
        for kind in kinds {
            if check(kind) {
                advance()
                return true
            }
        }
        return false
    }

    private func expect(_ kind: TokenKind, _ message: String) -> Token? {
        if check(kind) {
            return advance()
        }
        diagnose(message)
        return nil
    }

    private func diagnose(_ message: String) {
        diagnostics.append(
            ParserDiagnostic(
                severity: .error,
                message: message,
                line: current.location.line,
                column: current.location.column
            )
        )
    }

    private func synchronize() {
        while !isAtEnd {
            if previous.kind == .semicolon || previous.kind == .rightBrace {
                return
            }

            switch current.kind {
            case .funcKeyword, .structKeyword, .enumKeyword, .letKeyword,
                 .varKeyword, .importKeyword, .ifKeyword, .whileKeyword:
                return
            default:
                advance()
            }
        }
    }

    private func parseDeclaration() -> Declaration? {
        if match(.importKeyword) {
            return parseImport()
        }

        if check(.funcKeyword) {
            return .function(parseFunction())
        }

        if check(.structKeyword) {
            return .struct(parseStruct())
        }

        if check(.enumKeyword) {
            return .enumeration(parseEnum())
        }

        if check(.letKeyword) || check(.varKeyword) {
            return .variable(parseVariable())
        }

        if isDirective(current.kind) {
            return parseDirective()
        }

        if match(.typealiasKeyword) {
            let name = parseIdentifier("expected typealias name")
            _ = expect(.equal, "expected '=' in typealias")
            let type = parseType()
            consumeStatementTerminator()
            return .typealias(name: name, type: type)
        }

        if match(.extensionKeyword) {
            let name = parseIdentifier("expected extension name")
            let members = parseMemberList()
            return .extensionDecl(name: name, members: members)
        }

        diagnose("expected declaration")
        return nil
    }

    private func parseImport() -> Declaration {
        var path: [String] = []
        path.append(parseIdentifier("expected module name after import"))
        while match(.dot) {
            path.append(parseIdentifier("expected name after '.' in import"))
        }
        consumeStatementTerminator()
        return .import(ImportDeclaration(path: path))
    }

    private func parseFunction() -> FunctionDeclaration {
        _ = expect(.funcKeyword, "expected 'func'")

        let modifiers = parseModifiers()
        let attributes = parseAttributes()
        let name = parseIdentifier("expected function name")
        let parameters = parseParameters()
        var returnType: TypeSyntax?

        if match(.arrow) {
            returnType = parseType()
        } else if match(.colon) {
            returnType = parseType()
        }

        let body = parseStatementBlock()
        return FunctionDeclaration(
            name: name,
            parameters: parameters,
            returnType: returnType,
            modifiers: modifiers,
            attributes: attributes,
            body: body
        )
    }

    private func parseStruct() -> StructDeclaration {
        _ = expect(.structKeyword, "expected 'struct'")
        let name = parseIdentifier("expected struct name")
        let members = parseMemberList()
        return StructDeclaration(name: name, members: members)
    }

    private func parseEnum() -> EnumDeclaration {
        _ = expect(.enumKeyword, "expected 'enum'")
        let name = parseIdentifier("expected enum name")
        _ = expect(.leftBrace, "expected '{' after enum name")

        var cases: [EnumCase] = []
        while !check(.rightBrace) && !isAtEnd {
            guard match(.caseKeyword) else {
                diagnose("expected 'case' in enum")
                synchronize()
                continue
            }

            let caseName = parseIdentifier("expected enum case name")
            var associated: [Parameter] = []

            if match(.leftParen) {
                associated = parseParameterList(afterOpeningParen: true)
            }

            consumeStatementTerminator()
            cases.append(EnumCase(name: caseName, associatedValues: associated))
        }

        _ = expect(.rightBrace, "expected '}' after enum")
        return EnumDeclaration(name: name, cases: cases)
    }

    private func parseVariable() -> VariableDeclaration {
        let isLet = match(.letKeyword)
        if !isLet {
            _ = expect(.varKeyword, "expected 'let' or 'var'")
        }

        let name = parseIdentifier("expected variable name")
        var type: TypeSyntax?

        if match(.colon) {
            type = parseType()
        }

        var initializer: Expression?
        if match(.equal) {
            initializer = parseExpression()
        }

        consumeStatementTerminator()
        return VariableDeclaration(
            isLet: isLet,
            name: name,
            type: type,
            initializer: initializer
        )
    }

    private func parseDirective() -> Declaration {
        let kind = advance().kind
        var arguments: [Expression] = []

        while !check(.semicolon) &&
              !check(.rightBrace) &&
              !isAtEnd {
            arguments.append(parseExpression())
            if !match(.comma) { break }
        }

        consumeStatementTerminator()
        return .directive(DirectiveDeclaration(kind: kind, arguments: arguments))
    }

    private func parseMemberList() -> [Declaration] {
        _ = expect(.leftBrace, "expected '{'")
        var members: [Declaration] = []

        while !check(.rightBrace) && !isAtEnd {
            if let member = parseDeclaration() {
                members.append(member)
            } else {
                synchronize()
            }
        }

        _ = expect(.rightBrace, "expected '}'")
        return members
    }

    private func parseStatementBlock() -> [Statement] {
        _ = expect(.leftBrace, "expected '{'")
        var statements: [Statement] = []

        while !check(.rightBrace) && !isAtEnd {
            if let statement = parseStatement() {
                statements.append(statement)
            } else {
                synchronize()
            }
        }

        _ = expect(.rightBrace, "expected '}'")
        return statements
    }

    private func parseStatement() -> Statement? {
        if check(.leftBrace) {
            return .block(parseStatementBlock())
        }

        if check(.letKeyword) || check(.varKeyword) {
            return .declaration(parseVariable())
        }

        if check(.funcKeyword) {
            return .declaration(.function(parseFunction()))
        }

        if match(.ifKeyword) {
            return parseIf()
        }

        if match(.whileKeyword) {
            let condition = parseExpression()
            let body = parseStatementBlock()
            return .whileLoop(condition: condition, body: body)
        }

        if match(.doKeyword) {
            let body = parseStatementBlock()
            _ = expect(.whileKeyword, "expected 'while' after do block")
            let condition = parseExpression()
            consumeStatementTerminator()
            return .doWhile(body: body, condition: condition)
        }

        if match(.loop) {
            let body = parseStatementBlock()
            return .loop(body: body)
        }

        if match(.return) {
            let value: Expression?
            if check(.semicolon) || check(.rightBrace) || isAtEnd {
                value = nil
            } else {
                value = parseExpression()
            }
            consumeStatementTerminator()
            return .return(value)
        }

        if match(.breakKeyword) {
            consumeStatementTerminator()
            return .breakStatement
        }

        if match(.continueKeyword) {
            consumeStatementTerminator()
            return .continueStatement
        }

        if match(.deferKeyword) {
            return .deferStatement(parseStatementBlock())
        }

        if match(.guardKeyword) {
            let condition = parseExpression()
            let body = parseStatementBlock()
            return .guardStatement(condition: condition, body: body)
        }

        if match(.switchKeyword) {
            return parseSwitch()
        }

        if check(.semicolon) {
            advance()
            return .empty
        }

        let expression = parseExpression()
        consumeStatementTerminator()
        return .expression(expression)
    }

    private func parseIf() -> Statement {
        let condition = parseExpression()
        let thenBody = parseStatementBlock()
        var elseBody: [Statement]?

        if match(.elseKeyword) {
            if check(.ifKeyword) {
                let nested = parseStatement()
                elseBody = nested.map { [$0] }
            } else {
                elseBody = parseStatementBlock()
            }
        }

        return .if(
            condition: condition,
            thenBody: thenBody,
            elseBody: elseBody
        )
    }

    private func parseSwitch() -> Statement {
        let expression = parseExpression()
        _ = expect(.leftBrace, "expected '{' after switch expression")
        var cases: [SwitchCase] = []

        while !check(.rightBrace) && !isAtEnd {
            var pattern: Expression?

            if match(.caseKeyword) {
                pattern = parseExpression()
            } else if match(.defaultKeyword) {
                pattern = nil
            } else {
                diagnose("expected 'case' or 'default'")
                synchronize()
                continue
            }

            _ = expect(.colon, "expected ':' after switch case")
            var body: [Statement] = []

            while !check(.caseKeyword) &&
                  !check(.defaultKeyword) &&
                  !check(.rightBrace) &&
                  !isAtEnd {
                if let statement = parseStatement() {
                    body.append(statement)
                } else {
                    synchronize()
                }
            }

            cases.append(SwitchCase(pattern: pattern, body: body))
        }

        _ = expect(.rightBrace, "expected '}' after switch")
        return .switchStatement(expression: expression, cases: cases)
    }

    private func parseExpression() -> Expression {
        parseAssignment()
    }

    private func parseAssignment() -> Expression {
        let left = parseConditional()

        if match(
            .equal,
            .plusEqual,
            .minusEqual,
            .starEqual,
            .slashEqual,
            .percentEqual
        ) {
            let op = previous.kind
            let right = parseAssignment()
            return .assignment(left: left, operator: op, right: right)
        }

        return left
    }

    private func parseConditional() -> Expression {
        var expression = parseLogicalOr()

        if match(.question) {
            let thenExpression = parseExpression()
            _ = expect(.colon, "expected ':' in conditional expression")
            let elseExpression = parseExpression()
            expression = .conditional(
                condition: expression,
                thenBranch: thenExpression,
                elseBranch: elseExpression
            )
        }

        return expression
    }

    private func parseLogicalOr() -> Expression {
        var expression = parseLogicalAnd()
        while match(.logicalOr) {
            let op = previous.kind
            let right = parseLogicalAnd()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseLogicalAnd() -> Expression {
        var expression = parseEquality()
        while match(.logicalAnd) {
            let op = previous.kind
            let right = parseEquality()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseEquality() -> Expression {
        var expression = parseComparison()
        while match(.doubleEqual, .notEqual, .approximateEqual) {
            let op = previous.kind
            let right = parseComparison()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseComparison() -> Expression {
        var expression = parseRange()
        while match(.less, .lessEqual, .greater, .greaterEqual) {
            let op = previous.kind
            let right = parseRange()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseRange() -> Expression {
        var expression = parseTerm()

        if match(.range, .closedRange) {
            let closed = previous.kind == .closedRange
            let upper = parseTerm()
            expression = .range(lower: expression, upper: upper, closed: closed)
        }

        return expression
    }

    private func parseTerm() -> Expression {
        var expression = parseFactor()
        while match(.plus, .minus, .pipe, .caret) {
            let op = previous.kind
            let right = parseFactor()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseFactor() -> Expression {
        var expression = parseUnary()
        while match(.star, .slash, .percent, .ampersand) {
            let op = previous.kind
            let right = parseUnary()
            expression = .binary(left: expression, operator: op, right: right)
        }
        return expression
    }

    private func parseUnary() -> Expression {
        if match(.minus, .plus, .exclamation, .tilde) {
            let op = previous.kind
            return .unary(operator: op, operand: parseUnary())
        }

        return parsePostfix()
    }

    private func parsePostfix() -> Expression {
        var expression = parsePrimary()

        while true {
            if match(.dot) {
                let name = parseIdentifier("expected member name after '.'")
                expression = .member(base: expression, name: name)
                continue
            }

            if match(.leftParen) {
                let arguments = parseArguments(afterOpeningParen: true)
                expression = .call(callee: expression, arguments: arguments)
                continue
            }

            if match(.leftBracket) {
                var arguments: [Expression] = []
                while !check(.rightBracket) && !isAtEnd {
                    arguments.append(parseExpression())
                    if !match(.comma) { break }
                }
                _ = expect(.rightBracket, "expected ']'")
                expression = .subscriptExpression(base: expression, arguments: arguments)
                continue
            }

            if match(.asKeyword) {
                let type = parseType()
                expression = .cast(expression, type)
                continue
            }

            break
        }

        return expression
    }

    private func parsePrimary() -> Expression {
        switch current.kind {
        case .identifier:
            return .identifier(advance().lexeme)

        case .integerLiteral:
            return .integer(advance().lexeme)

        case .floatingLiteral:
            return .floating(advance().lexeme)

        case .stringLiteral:
            return .string(advance().lexeme)

        case .characterLiteral:
            return .character(advance().lexeme)

        case .trueKeyword:
            advance()
            return .boolean(true)

        case .falseKeyword:
            advance()
            return .boolean(false)

        case .nilKeyword:
            advance()
            return .nilLiteral

        case .leftParen:
            advance()
            var values: [Expression] = []

            if !check(.rightParen) {
                repeat {
                    values.append(parseExpression())
                } while match(.comma)
            }

            _ = expect(.rightParen, "expected ')'")

            if values.count == 1 {
                return values[0]
            }
            return .tuple(values)

        case .leftBracket:
            advance()
            var values: [Expression] = []

            if !check(.rightBracket) {
                repeat {
                    values.append(parseExpression())
                } while match(.comma)
            }

            _ = expect(.rightBracket, "expected ']'")
            return .array(values)

        case .funcKeyword:
            advance()
            let parameters = parseParameters()
            let body = parseStatementBlock()
            return .closure(parameters: parameters, body: body)

        default:
            diagnose("expected expression")
            let token = advance()
            return .identifier(token.lexeme)
        }
    }

    private func parseParameters() -> [Parameter] {
        guard match(.leftParen) else {
            diagnose("expected '(' before parameter list")
            return []
        }
        return parseParameterList(afterOpeningParen: true)
    }

    private func parseParameterList(afterOpeningParen: Bool) -> [Parameter] {
        var parameters: [Parameter] = []

        if !afterOpeningParen {
            _ = expect(.leftParen, "expected '('")
        }

        while !check(.rightParen) && !isAtEnd {
            var firstName: String?
            var secondName: String?

            if check(.identifier) {
                firstName = advance().lexeme
                if check(.identifier) {
                    secondName = advance().lexeme
                }
            } else {
                diagnose("expected parameter name")
                advance()
                continue
            }

            let localName = secondName ?? firstName!
            let externalName = secondName == nil ? nil : firstName

            var type: TypeSyntax?
            if match(.colon) {
                type = parseType()
            }

            var defaultValue: Expression?
            if match(.equal) {
                defaultValue = parseExpression()
            }

            parameters.append(
                Parameter(
                    externalName: externalName,
                    localName: localName,
                    type: type,
                    defaultValue: defaultValue
                )
            )

            if !match(.comma) { break }
        }

        _ = expect(.rightParen, "expected ')' after parameters")
        return parameters
    }

    private func parseArguments(afterOpeningParen: Bool) -> [Argument] {
        var arguments: [Argument] = []

        while !check(.rightParen) && !isAtEnd {
            var label: String?

            if check(.identifier) &&
               index + 1 < tokens.count &&
               tokens[index + 1].kind == .colon {
                label = advance().lexeme
                _ = advance()
            }

            let value = parseExpression()
            arguments.append(Argument(label: label, value: value))

            if !match(.comma) { break }
        }

        _ = expect(.rightParen, "expected ')' after arguments")
        return arguments
    }

    private func parseType() -> TypeSyntax {
        if match(.intKeyword) {
            return .named("Int")
        }

        if match(.numKeyword) {
            return .named("Num")
        }

        if match(.stringKeyword) {
            return .named("String")
        }

        if match(.bytesKeyword) {
            return .named("Bytes")
        }

        if match(.nilKeyword) {
            return .named("Nil")
        }

        if match(.leftBracket) {
            let element = parseType()
            _ = expect(.rightBracket, "expected ']' in array type")
            return .array(element)
        }

        if match(.leftParen) {
            var types: [TypeSyntax] = []

            if !check(.rightParen) {
                repeat {
                    types.append(parseType())
                } while match(.comma)
            }

            _ = expect(.rightParen, "expected ')' in tuple type")

            if match(.arrow) {
                return .function(parameters: types, result: parseType())
            }

            return .tuple(types)
        }

        if check(.identifier) {
            let name = advance().lexeme
            var type: TypeSyntax = .named(name)

            if match(.question) {
                type = .optional(type)
            }

            return type
        }

        diagnose("expected type")
        return .inferred
    }

    private func parseModifiers() -> [DeclarationModifier] {
        var modifiers: [DeclarationModifier] = []

        while true {
            switch current.kind {
            case .staticKeyword:
                advance()
                modifiers.append(.static)
            case .finalKeyword:
                advance()
                modifiers.append(.final)
            case .classKeyword:
                advance()
                modifiers.append(.class)
            case .actorKeyword:
                advance()
                modifiers.append(.actor)
            case .overrideKeyword:
                advance()
                modifiers.append(.override)
            case .requiredKeyword:
                advance()
                modifiers.append(.required)
            case .convenienceKeyword:
                advance()
                modifiers.append(.convenience)
            case .mutatingKeyword:
                advance()
                modifiers.append(.mutating)
            case .nonmutatingKeyword:
                advance()
                modifiers.append(.nonmutating)
            case .consumingKeyword:
                advance()
                modifiers.append(.consuming)
            case .borrowingKeyword:
                advance()
                modifiers.append(.borrowing)
            case .lazyKeyword:
                advance()
                modifiers.append(.lazy)
            case .weakKeyword:
                advance()
                modifiers.append(.weak)
            case .unownedKeyword:
                advance()
                modifiers.append(.unowned)
            case .isolatedKeyword:
                advance()
                modifiers.append(.isolated)
            case .nonisolatedKeyword:
                advance()
                modifiers.append(.nonisolated)
            case .indirectKeyword:
                advance()
                modifiers.append(.indirect)
            default:
                return modifiers
            }
        }
    }

    private func parseAttributes() -> [Attribute] {
        var attributes: [Attribute] = []

        while match(.at) {
            let name = parseIdentifier("expected attribute name")
            var arguments: [Expression] = []

            if match(.leftParen) {
                while !check(.rightParen) && !isAtEnd {
                    arguments.append(parseExpression())
                    if !match(.comma) { break }
                }
                _ = expect(.rightParen, "expected ')' after attribute")
            }

            attributes.append(Attribute(name: name, arguments: arguments))
        }

        return attributes
    }

    private func parseIdentifier(_ message: String) -> String {
        if current.kind == .identifier {
            return advance().lexeme
        }

        if let keyword = keywordSpelling(current.kind) {
            advance()
            return keyword
        }

        diagnose(message)
        return "<missing>"
    }

    private func consumeStatementTerminator() {
        if match(.semicolon) {
            return