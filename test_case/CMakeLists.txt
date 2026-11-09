cmake_minimum_required(VERSION 3.12)
project(gfc)

add_library(gfc src/gfc.c)
target_include_directories(gfc PUBLIC "${PROJECT_SOURCE_DIR}/include")
target_link_libraries(gfc PRIVATE m)

add_executable(driver src/gfc_entrypoint.c)
target_link_libraries(driver PRIVATE gfc)
