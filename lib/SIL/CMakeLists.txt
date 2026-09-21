cmake_minimum_required(VERSION 3.20)
project(shift-sil LANGUAGES NONE)
add_custom_target(shift-sil ALL COMMAND cargo build WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
