final class SILGenDiagnostics {
    private(set) var messages: [String] = []

    func error(_ message: String) {
        messages.append(message)
    }

    var hasErrors: Bool {
        !messages.isEmpty
    }
}
