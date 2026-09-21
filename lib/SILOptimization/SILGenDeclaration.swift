final class SILGenDeclaration {
    private let context: SILGenContext

    init(context: SILGenContext) {
        self.context = context
    }

    func emitGlobal(_ declaration: VariableDeclaration) throws {
        let type = try context.typeConverter.convert(declaration.type)
        let initializer = try constant(declaration.initializer)

        context.module.addGlobal(
            SILGlobalVariable(
                name: declaration.name.name,
                type: type,
                isMutable: declaration.isMutable,
                initializer: initializer
            )
        )
    }

    func emitFunction(_ declaration: FunctionDeclaration) throws {
        let converter = context.typeConverter

        let parameters = try declaration.parameters.enumerated().map { index, parameter in
            let type = try converter.convert(parameter.type)
            return SILFunction.Parameter(
                name: parameter.name.name,
                value: SILValue(id: index, type: type)
            )
        }

        let returnType = try converter.convert(declaration.returnType)
        let generated = context.makeFunction(
            name: declaration.name.name,
            parameters: parameters,
            returnType: returnType
        )

        let entry = generated.function.createBlock()
        generated.builder.position(at: entry)

        for parameter in declaration.parameters {
            if let value = generated.function.parameters.first(where: {
                $0.name == parameter.name.name
            })?.value {
                generated.bind(parameter.name.name, to: value)
            }
        }

        try SILGenStatement(context: context).emit(
            declaration.body,
            into: generated
        )

        if entry.instructions.last == nil {
            if returnType == .void {
                generated.builder.createReturn()
            }
        }
    }

    private func constant(_ expression: Expression?) throws -> SILConstant? {
        guard let expression else { return nil }

        switch expression {
        case .integerLiteral(let value, _):
            guard let integer = Int64(value) else { return nil }
            return .integer(integer)

        case .floatingLiteral(let value, _):
            guard let floating = Double(value) else { return nil }
            return .floating(floating)

        case .stringLiteral(let value, _):
            return .string(value)

        case .characterLiteral(let value, _):
            return .character(value)

        case .booleanLiteral(let value, _):
            return .boolean(value)

        default:
            return nil
        }
    }
}
