cmake_minimum_required(VERSION 3.20)
project(ShiftCodeGeneration LANGUAGES NONE)

add_custom_target(ShiftCodeGeneration ALL
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)
