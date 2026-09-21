final class SILGenFunction {
    let function: SILFunction
    let builder: SILBuilder
    var variables: [String: SILValue] = [:]
    var returnType: SILType

    init(function: SILFunction) {
        self.function = function
        self.builder = SILBuilder()
        self.returnType = function.returnType
    }

    func bind(_ name: String, to value: SILValue) {
        variables[name] = value
    }

    func lookup(_ name: String) -> SILValue? {
        variables[name]
    }
}
