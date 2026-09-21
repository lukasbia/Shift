extension SILBuilder {
    func append(_ instruction: SILInstruction) {
        guard let block = currentBlock else {
            fatalError("SILBuilder has no insertion point")
        }
        block.append(instruction)
    }

    func appendCharacterLiteral(result: SILValue, value: String) {
        append(.characterLiteral(result: result, value: value))
    }
}
