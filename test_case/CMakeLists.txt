# These two lines are required for all CMake projects!
cmake_minimum_required(VERSION 3.12)
project(gfc)

# This project is more like a library with a custom-written `gfc_entrypoint.c` file.
# So first, we add the `gfc.c` library and header file in `./include/`
add_library(gfc src/gfc.c)
target_include_directories(gfc PUBLIC "${PROJECT_SOURCE_DIR}/include")
# This project also requires the math library.
target_link_libraries(gfc PRIVATE m)

# Add the `gfc_entrypoint.c` (which has `int main(int argc, char** argv)`) as a binary executable.
add_executable(driver src/gfc_entrypoint.c)
# Link the executable to the original library code (so it can use `#include <gfc/gfc.h>`)
target_link_libraries(driver PRIVATE gfc)
