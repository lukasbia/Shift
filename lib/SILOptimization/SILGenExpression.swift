final class SILGenExpression {
    private let context: SILGenContext

    init(context: SILGenContext) {
        self.context = context
    }

    func emit(_ expression: Expression, into function: SILGenFunction) throws -> SILValue {
        switch expression {
        case .identifier(let identifier):
            guard let value = function.lookup(identifier.name) else {
                if context.scope.hasGlobal(identifier.name) {
                    throw SILGenError.unsupportedExpression(
                        "global load requires global-address lowering: \(identifier.name)"
                    )
                }

                throw SILGenError.unsupportedExpression(
                    "unresolved identifier \(identifier.name)"
                )
            }

            if case .address = value.type {
                return function.builder.createLoad(from: value)
            }

            return value

        case .integerLiteral(let text, _):
            guard let value = Int64(text) else {
                throw SILGenError.unsupportedExpression("invalid integer literal")
            }
            return function.builder.createIntegerLiteral(value)

        case .floatingLiteral(let text, _):
            guard let value = Double(text) else {
                throw SILGenError.unsupportedExpression("invalid floating literal")
            }
            return function.builder.createFloatingLiteral(value)

        case .stringLiteral(let value, _):
            return function.builder.createStringLiteral(value)

        case .characterLiteral(let value, _):
            let result = function.builder.createValue(type: .character)
            function.builder.appendCharacterLiteral(result: result, value: value)
            return result

        case .booleanLiteral(let value, _):
            return function.builder.createBooleanLiteral(value)

        case .parenthesized(let value, _):
            return try emit(value, into: function)

        case .unary(let operation, let operand, _):
            let value = try emit(operand, into: function)

            switch operation {
            case .plus:
                return value

            case .minus:
                let result = function.builder.createValue(type: value.type)
                function.builder.append(.negate(result: result, operand: value))
                return result

            case .logicalNot:
                let result = function.builder.createValue(type: .bool)
                function.builder.append(.logicalNot(result: result, operand: value))
                return result

            case .addressOf:
                let result = function.builder.createValue(type: .pointer(value.type))
                function.builder.append(.addressOf(result: result, value: value))
                return result

            case .dereference:
                guard case .pointer(let object) = value.type else {
                    throw SILGenError.invalidLValue
                }
                let result = function.builder.createValue(type: object)
                function.builder.append(.pointerToAddress(result: result, pointer: value))
                return function.builder.createLoad(from: result)
            }

        case .binary(let left, let operation, let right, _):
            let lhs = try emit(left, into: function)
            let rhs = try emit(right, into: function)

            switch operation {
            case .add:
                return function.builder.createAdd(lhs, rhs)
            case .subtract:
                return function.builder.createSubtract(lhs, rhs)
            case .multiply:
                return function.builder.createMultiply(lhs, rhs)
            case .divide:
                return function.builder.createDivide(lhs, rhs)

            case .remainder:
                let result = function.builder.createValue(type: lhs.type)
                function.builder.append(.remainder(result: result, left: lhs, right: rhs))
                return result

            case .equal:
                return function.builder.createEqual(lhs, rhs)

            case .less:
                return function.builder.createLessThan(lhs, rhs)

            case .notEqual, .lessEqual, .greater, .greaterEqual,
                 .logicalAnd, .logicalOr,
                 .bitwiseAnd, .bitwiseOr, .bitwiseXor:
                return try emitExtendedBinary(operation, lhs: lhs, rhs: rhs, into: function)
            }

        case .assignment(let target, let value, _):
            let rhs = try emit(value, into: function)
            let address = try emitAddress(target, into: function)
            function.builder.createStore(value: rhs, to: address)
            return rhs

        case .call(let callee, let arguments, _):
            let functionValue = try emitFunctionReference(callee, into: function)
            let values = try arguments.map {
                try emit($0.expression, into: function)
            }

            let resultType: SILType
            if case .function(_, let result) = functionValue.type {
                resultType = result
            } else {
                resultType = .void
            }

            let result = resultType == .void
                ? nil
                : function.builder.createValue(type: resultType)

            function.builder.append(
                .apply(
                    result: result,
                    function: functionValue,
                    arguments: values
                )
            )

            return result ?? function.builder.createValue(type: .void)

        case .member, .subscriptExpression, .arrayLiteral, .tuple:
            throw SILGenError.unsupportedExpression(
                "aggregate/member lowering requires layout metadata"
            )
        }
    }

    private func emitFunctionReference(
        _ expression: Expression,
        into function: SILGenFunction
    ) throws -> SILValue {
        guard case .identifier(let identifier) = expression else {
            return try emit(expression, into: function)
        }

        guard let target = context.module.function(named: identifier.name) else {
            throw SILGenError.missingFunction(identifier.name)
        }

        let type = SILType.function(
            parameters: target.parameters.map { $0.value.type },
            result: target.returnType
        )

        let result = function.builder.createValue(type: type)
        function.builder.append(
            .functionRef(
                result: result,
                name: target.name,
                type: type
            )
        )
        return result
    }

    private func emitAddress(
        _ expression: Expression,
        into function: SILGenFunction
    ) throws -> SILValue {
        guard case .identifier(let identifier) = expression else {
            throw SILGenError.invalidLValue
        }

        guard let value = function.lookup(identifier.name) else {
            throw SILGenError.invalidLValue
        }

        guard case .address = value.type else {
            throw SILGenError.invalidLValue
        }

        return value
    }

    private func emitExtendedBinary(
        _ operation: BinaryOperator,
        lhs: SILValue,
        rhs: SILValue,
        into function: SILGenFunction
    ) throws -> SILValue {
        let builder = function.builder

        switch operation {
        case .notEqual:
            let equal = builder.createEqual(lhs, rhs)
            let result = builder.createValue(type: .bool)
            builder.append(.logicalNot(result: result, operand: equal))
            return result

        case .lessEqual:
            let less = builder.createLessThan(lhs, rhs)
            let equal = builder.createEqual(lhs, rhs)
            let result = builder.createValue(type: .bool)
            builder.append(.logicalOr(result: result, left: less, right: equal))
            return result

        case .greater:
            let result = builder.createValue(type: .bool)
            builder.append(.greaterThan(result: result, left: lhs, right: rhs))
            return result

        case .greaterEqual:
            let greater = builder.createValue(type: .bool)
            builder.append(.greaterThan(result: greater, left: lhs, right: rhs))
            let equal = builder.createEqual(lhs, rhs)
            let result = builder.createValue(type: .bool)
            builder.append(.logicalOr(result: result, left: greater, right: equal))
            return result

        case .logicalAnd:
            let result = builder.createValue(type: .bool)
            builder.append(.logicalAnd(result: result, left: lhs, right: rhs))
            return result

        case .logicalOr:
            let result = builder.createValue(type: .bool)
            builder.append(.logicalOr(result: result, left: lhs, right: rhs))
            return result

        case .bitwiseAnd:
            let result = builder.createValue(type: lhs.type)
            builder.append(.bitwiseAnd(result: result, left: lhs, right: rhs))
            return result

        case .bitwiseOr:
            let result = builder.createValue(type: lhs.type)
            builder.append(.bitwiseOr(result: result, left: lhs, right: rhs))
            return result

        case .bitwiseXor:
            let result = builder.createValue(type: lhs.type)
            builder.append(.bitwiseXor(result: result, left: lhs, right: rhs))
            return result

        default:
            throw SILGenError.unsupportedExpression("binary operator")
        }
    }
}
