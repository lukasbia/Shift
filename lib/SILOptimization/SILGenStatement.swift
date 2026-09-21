final class SILGenStatement {
    private let context: SILGenContext

    init(context: SILGenContext) {
        self.context = context
    }

    func emit(_ block: CodeBlock, into function: SILGenFunction) throws {
        for statement in block.statements {
            try emit(statement, into: function)
        }
    }

    func emit(_ statement: Statement, into function: SILGenFunction) throws {
        switch statement {
        case .expression(let expression):
            _ = try SILGenExpression(context: context).emit(expression, into: function)

        case .variable(let declaration):
            try emitVariable(declaration, into: function)

        case .returnStatement(let statement):
            if let expression = statement.value {
                let value = try SILGenExpression(context: context).emit(
                    expression,
                    into: function
                )
                function.builder.createReturn(value)
            } else {
                function.builder.createReturn()
            }

        case .ifStatement(let statement):
            try SILGenControlFlow(context: context).emit(
                statement,
                into: function
            )

        case .whileStatement(let statement):
            try SILGenControlFlow(context: context).emit(
                statement,
                into: function
            )

        case .forStatement(let statement):
            try SILGenControlFlow(context: context).emit(
                statement,
                into: function
            )

        case .breakStatement:
            throw SILGenError.unsupportedStatement("break requires loop target stack")

        case .continueStatement:
            throw SILGenError.unsupportedStatement("continue requires loop target stack")

        case .switchStatement:
            throw SILGenError.unsupportedStatement(
                "switch lowering requires switch CFG support"
            )
        }
    }

    private func emitVariable(
        _ declaration: VariableDeclaration,
        into function: SILGenFunction
    ) throws {
        let type = try context.typeConverter.convert(declaration.type)
        let address = function.builder.createAllocStack(type: type)
        function.bind(declaration.name.name, to: address)

        if let initializer = declaration.initializer {
            let value = try SILGenExpression(context: context).emit(
                initializer,
                into: function
            )
            function.builder.createStore(value: value, to: address)
        }
    }
}
