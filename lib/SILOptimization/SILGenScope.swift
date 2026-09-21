final class SILGenScope {
    private var functions: Set<String> = []
    private var globals: Set<String> = []

    func declareFunction(_ name: String) {
        functions.insert(name)
    }

    func declareGlobal(_ name: String) {
        globals.insert(name)
    }

    func hasFunction(_ name: String) -> Bool {
        functions.contains(name)
    }

    func hasGlobal(_ name: String) -> Bool {
        globals.contains(name)
    }
}
