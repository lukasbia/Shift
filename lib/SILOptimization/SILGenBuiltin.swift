enum SILGenBuiltin {
    static let printFunctionName = "print"

    static func isBuiltin(_ name: String) -> Bool {
        name == printFunctionName
    }
}
