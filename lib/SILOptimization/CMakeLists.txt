add_library(ShiftSILGen STATIC
    SILGen.swift
    SILGenContext.swift
    SILGenError.swift
    SILGenFunction.swift
    SILGenModule.swift
    SILGenScope.swift
    SILGenTypeConverter.swift
    SILGenExpression.swift
    SILGenStatement.swift
    SILGenDeclaration.swift
    SILGenControlFlow.swift
    SILGenValue.swift
    SILGenLValue.swift
    SILGenBuiltin.swift
    SILGenDiagnostics.swift
    SILGenBuilderExtensions.swift
)

target_link_libraries(ShiftSILGen
    PRIVATE
    ShiftAST
    ShiftSIL
)
