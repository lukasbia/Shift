final class SILGenControlFlow {
    private let context: SILGenContext

    init(context: SILGenContext) {
        self.context = context
    }

    func emit(_ statement: IfStatement, into function: SILGenFunction) throws {
        let condition = try SILGenExpression(context: context).emit(
            statement.condition,
            into: function
        )

        let thenBlock = function.function.createBlock()
        let mergeBlock = function.function.createBlock()

        let elseBlock: SILBasicBlock?
        switch statement.elseBody {
        case .none:
            elseBlock = nil
        case .block:
            elseBlock = function.function.createBlock()
        case .ifStatement:
            elseBlock = function.function.createBlock()
        }

        function.builder.createConditionalBranch(
            condition: condition,
            trueBlock: thenBlock,
            falseBlock: elseBlock ?? mergeBlock
        )

        function.builder.position(at: thenBlock)
        try SILGenStatement(context: context).emit(
            statement.body,
            into: function
        )

        if thenBlock.terminator == nil {
            function.builder.createBranch(to: mergeBlock)
        }

        if let elseBlock {
            function.builder.position(at: elseBlock)

            switch statement.elseBody {
            case .block(let block):
                try SILGenStatement(context: context).emit(block, into: function)
            case .ifStatement(let nested):
                try emit(nested, into: function)
            case .none:
                break
            }

            if elseBlock.terminator == nil {
                function.builder.createBranch(to: mergeBlock)
            }
        }

        function.builder.position(at: mergeBlock)
    }

    func emit(_ statement: WhileStatement, into function: SILGenFunction) throws {
        let conditionBlock = function.function.createBlock()
        let bodyBlock = function.function.createBlock()
        let exitBlock = function.function.createBlock()

        function.builder.createBranch(to: conditionBlock)
        function.builder.position(at: conditionBlock)

        let condition = try SILGenExpression(context: context).emit(
            statement.condition,
            into: function
        )

        function.builder.createConditionalBranch(
            condition: condition,
            trueBlock: bodyBlock,
            falseBlock: exitBlock
        )

        function.builder.position(at: bodyBlock)

        try SILGenStatement(context: context).emit(
            statement.body,
            into: function
        )

        if bodyBlock.terminator == nil {
            function.builder.createBranch(to: conditionBlock)
        }

        function.builder.position(at: exitBlock)
    }

    func emit(_ statement: ForStatement, into function: SILGenFunction) throws {
        // Lower `for pattern in sequence` through the canonical loop CFG.
        // Iteration protocol lowering is deliberately isolated here so the
        // rest of SILGen does not need to know how arrays/iterators are represented.
        throw SILGenError.unsupportedStatement(
            "for-in iteration lowering requires iterator protocol SIL operations"
        )
    }
}
