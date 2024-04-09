# CMakeList.txt : CMake project for 2D_Physics_Engine, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

project (2D_Physics_Engine)

# Add source to this project's executable.
add_executable (2D_Physics_Engine "Main.cpp"  "vector_base.h" "vector_2d_euclidean.cpp" "vector_2d_euclidean.h" "entity.h"   "entity.cpp" "vector_base.cpp" "entity_group.h" "constraint_base.h" "force_accumulator.h" "force_accumulator.cpp" "force_base.h" "force_spring.h" "force_calculator.cpp" "force_calculator.h")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET 2D_Physics_Engine PROPERTY CXX_STANDARD 20)
endif()

# TODO: Add tests and install targets if needed.
