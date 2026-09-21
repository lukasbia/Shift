enum SILGenError: Error, CustomStringConvertible {
    case unsupportedDeclaration(String)
    case unsupportedExpression(String)
    case unsupportedStatement(String)
    case missingFunction(String)
    case invalidControlFlow
    case invalidLValue
    case typeMismatch(expected: SILType, actual: SILType)

    var description: String {
        switch self {
        case .unsupportedDeclaration(let name):
            return "SILGen does not support declaration: \(name)"
        case .unsupportedExpression(let name):
            return "SILGen does not support expression: \(name)"
        case .unsupportedStatement(let name):
            return "SILGen does not support statement: \(name)"
        case .missingFunction(let name):
            return "SILGen cannot find function: \(name)"
        case .invalidControlFlow:
            return "SILGen encountered invalid control flow"
        case .invalidLValue:
            return "expression is not an assignable l-value"
        case .typeMismatch(let expected, let actual):
            return "SIL type mismatch: expected \(expected.description), got \(actual.description)"
        }
    }
}
