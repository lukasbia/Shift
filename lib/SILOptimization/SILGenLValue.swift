struct SILGenLValue {
    let address: SILValue

    var objectType: SILType {
        guard case .address(let type) = address.type else {
            return address.type
        }
        return type
    }
}
