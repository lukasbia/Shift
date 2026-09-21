extension SILGenContext {
    func makeFunction(
        name: String,
        parameters: [SILFunction.Parameter],
        returnType: SILType
    ) -> SILGenFunction {
        let function = SILFunction(
            name: name,
            parameters: parameters,
            returnType: returnType
        )
        module.addFunction(function)
        return SILGenFunction(function: function)
    }
}
