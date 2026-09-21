final class SILGenContext {
    let module: SILModule
    let typeConverter: SILGenTypeConverter
    let diagnostics: SILGenDiagnostics
    let scope: SILGenScope

    init(module: SILModule) {
        self.module = module
        self.typeConverter = SILGenTypeConverter()
        self.diagnostics = SILGenDiagnostics()
        self.scope = SILGenScope()
    }

    func registerTopLevelDeclarations(_ declarations: [Declaration]) throws {
        for declaration in declarations {
            switch declaration {
            case .function(let function):
                scope.declareFunction(function.name.name)
            case .variable(let variable):
                scope.declareGlobal(variable.name.name)
            default:
                break
            }
        }
    }
}
