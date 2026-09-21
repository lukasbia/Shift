final class SILGen {
    private let moduleName: String
    private let module: SILModule
    private let context: SILGenContext

    init(moduleName: String) {
        self.moduleName = moduleName
        self.module = SILModule(name: moduleName)
        self.context = SILGenContext(module: module)
    }

    func generate(_ sourceFile: SourceFile) throws -> SILModule {
        try context.registerTopLevelDeclarations(sourceFile.declarations)

        for declaration in sourceFile.declarations {
            try generate(declaration)
        }

        return module
    }

    private func generate(_ declaration: Declaration) throws {
        switch declaration {
        case .variable(let declaration):
            try SILGenDeclaration(context: context).emitGlobal(declaration)

        case .function(let declaration):
            try SILGenDeclaration(context: context).emitFunction(declaration)

        case .structDecl:
            break

        case .classDecl:
            break

        case .enumDecl:
            break

        case .protocolDecl:
            break

        case .extensionDecl:
            break
        }
    }
}
