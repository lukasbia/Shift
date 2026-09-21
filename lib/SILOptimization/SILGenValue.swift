struct SILGenValue {
    let value: SILValue

    var type: SILType {
        value.type
    }
}
