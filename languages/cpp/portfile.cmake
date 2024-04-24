vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO bitwarden/sdk
    REF "${VERSION}"
    SHA512 7c2fb18261ce9185d29b690ccb7694d7926abe3af0619dbe42b7ab43b400ee71c1eb79c31f892aea2fbdb55036225f31ee393287cce91afd17f20cff8f6cb949
    HEAD_REF main
)

vcpkg_extract_source_archive_ex(
    OUT_SOURCE_PATH SOURCE_PATH
    ARCHIVE "${ARCHIVE}"
)

if(VCPKG_HOST_IS_WINDOWS)
    vcpkg_download_distfile(ARCHIVE
        URLS "https://github.com/bitwarden/sm-sdk-go/releases/download/v${VERSION}/libbitwarden_c_files-aarch64-apple-darwin-${VERSION}.zip"
        FILENAME "libbitwarden_c_files-aarch64-apple-darwin-${VERSION}.zip"
        SHA512 9392e227009a0c3692e48df8547131e5c79729ba10e1e56c4b84d1023322fa51dff6e0cfc513857ca8d210f76663cfe21bdfe35a30b10a40ef4a3288ef1eacb4
    )
endif()

if(VCPKG_HOST_IS_OSX)
    vcpkg_download_distfile(ARCHIVE
    URLS "https://github.com/bitwarden/sm-sdk-go/releases/download/v${VERSION}/libbitwarden_c_files-x86_64-pc-windows-msvc-${VERSION}.zip"
    FILENAME "libbitwarden_c_files-x86_64-pc-windows-msvc-${VERSION}.zip"
    SHA512 9392e227009a0c3692e48df8547131e5c79729ba10e1e56c4b84d1023322fa51dff6e0cfc513857ca8d210f76663cfe21bdfe35a30b10a40ef4a3288ef1eacb4
)
endif()

if(VCPKG_HOST_IS_LINUX)
    vcpkg_download_distfile(ARCHIVE
    URLS "https://github.com/bitwarden/sm-sdk-go/releases/download/v${VERSION}/libbitwarden_c_files-x86_64-unknown-linux-gnu-${VERSION}.zip"
    FILENAME "libbitwarden_c_files-x86_64-pc-windows-msvc-${VERSION}.zip"
    SHA512 9392e227009a0c3692e48df8547131e5c79729ba10e1e56c4b84d1023322fa51dff6e0cfc513857ca8d210f76663cfe21bdfe35a30b10a40ef4a3288ef1eacb4
)
endif()

vcpkg_extract_source_archive_ex(
    OUT_SOURCE_PATH SOURCE_PATH
    ARCHIVE "${ARCHIVE}"
)

file(COPY "${CMAKE_CURRENT_LIST_DIR}/CMakeLists.txt" DESTINATION "${SOURCE_PATH}")
file(COPY "${CMAKE_CURRENT_LIST_DIR}/7zip-config.cmake.in" DESTINATION "${SOURCE_PATH}")

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}/languages/cpp"
    OPTIONS
        -DNLOHMANN=DNLOHMANN_PATH
        -DBOOST=DBOOST_PATH
        -DTARGET=DTARGET_PATH
)

vcpkg_cmake_install()
vcpkg_copy_pdbs()

vcpkg_install_copyright(FILE_LIST "${SOURCE_PATH}/LICENSE")

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug/include")