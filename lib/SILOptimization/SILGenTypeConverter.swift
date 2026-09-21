final class SILGenTypeConverter {
    func convert(_ type: TypeSyntax?) throws -> SILType {
        guard let type else {
            return .void
        }

        switch type {
        case .named(let identifier):
            switch identifier.name {
            case "Int": return .int
            case "UInt": return .uint
            case "Int8": return .int8
            case "Int16": return .int16
            case "Int32": return .int32
            case "Int64": return .int64
            case "UInt8": return .uint8
            case "UInt16": return .uint16
            case "UInt32": return .uint32
            case "UInt64": return .uint64
            case "Float": return .float
            case "Double": return .double
            case "Bool": return .bool
            case "String": return .string
            case "Character": return .character
            case "Void": return .void
            default: return .named(identifier.name)
            }

        case .array(let element, _):
            return .array(element: try convert(element))

        case .pointer(let element, _):
            return .pointer(try convert(element))

        case .optional(let wrapped, _):
            return .optional(try convert(wrapped))

        case .tuple(let elements, _):
            return .tuple(try elements.map(convert))

        case .function(let parameters, let returnType, _):
            return .function(
                parameters: try parameters.map(convert),
                result: try convert(returnType)
            )
        }
    }
}
