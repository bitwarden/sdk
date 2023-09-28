cmake_minimum_required(VERSION 3.15)
project(BitwardenClient)

set(CMAKE_CXX_STANDARD 20)

# Specify the locations of nlohmann.json and Boost libraries
find_path(NLOHMANN_JSON_INCLUDE_DIR nlohmann/json.hpp HINTS /usr/local/include/nlohmann)
find_path(BOOST_INCLUDE_DIR boost/version.hpp HINTS /opt/homebrew/include)

# Include directories for library
include_directories(include ${NLOHMANN_JSON_INCLUDE_DIR} ${BOOST_INCLUDE_DIR} ${NLOHMANN_JSON_INCLUDE_DIR}/nlohmann)

# Add library source files
file(GLOB SOURCES "src/*.cpp")

# Add the schemas.cpp file explicitly from the schemas directory
set(SCHEMAS_SOURCE "include/schemas.cpp")

# Add library source files along with the schemas.cpp file
add_library(BitwardenClient SHARED ${SOURCES} ${SCHEMAS_SOURCE})

# Specify the path to the libbitwarden_c.dylib
# set(LIB_BITWARDEN_C "/path/to/libbitwarden_c.dylib")

# Copy the library to the build directory before building
add_custom_command(
    TARGET BitwardenClient PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_SOURCE_DIR}/../../target/debug/libbitwarden_c.dylib
        $<TARGET_FILE_DIR:BitwardenClient>
)

# Link libraries
target_link_libraries(BitwardenClient PRIVATE ${LIB_BITWARDEN_C})

