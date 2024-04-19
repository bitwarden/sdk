cmake_minimum_required(VERSION 3.15)
project(BitwardenClient)

set(CMAKE_CXX_STANDARD 20)

# Set placeholders to be passed from command line
set(NLOHMANN_JSON_INCLUDE_DIR_PLACEHOLDER ${NLOHMANN})
set(BOOST_INCLUDE_DIR_PLACEHOLDER ${BOOST})
set(TARGET_INCLUDE_DIR_PLACEHOLDER ${TARGET})

# Specify the locations of nlohmann.json and Boost libraries
find_path(NLOHMANN_JSON_INCLUDE_DIR nlohmann/json.hpp HINTS ${NLOHMANN_JSON_INCLUDE_DIR_PLACEHOLDER})
find_path(BOOST_INCLUDE_DIR boost/optional.hpp HINTS ${BOOST_INCLUDE_DIR_PLACEHOLDER})

# Include directories for library
include_directories(include ${NLOHMANN_JSON_INCLUDE_DIR} ${BOOST_INCLUDE_DIR})

# Add library source files
file(GLOB SOURCES "src/*.cpp")

# Add library source files along with the schemas.cpp file
add_library(BitwardenClient SHARED ${SOURCES} ${SCHEMAS_SOURCE})

# Set path for native library loading
set(LIB_BITWARDEN_C "${CMAKE_SOURCE_DIR}/${TARGET}")

# Copy the library to the build directory before building
add_custom_command(
    TARGET BitwardenClient PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
        ${LIB_BITWARDEN_C}
        $<TARGET_FILE_DIR:BitwardenClient>
)

# Link libraries
target_link_libraries(BitwardenClient PRIVATE ${LIB_BITWARDEN_C})
