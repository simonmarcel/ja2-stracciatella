cmake_minimum_required (VERSION 2.8)

include(ExternalProject)

## Some macros

macro(add_custom_templated_target NAME)
    configure_file(
            "${CMAKE_CURRENT_SOURCE_DIR}/cmake/target-${NAME}.cmake.in"
            "${CMAKE_CURRENT_BINARY_DIR}/cmake/target-${NAME}.cmake"
            IMMEDIATE @ONLY)

    add_custom_target(${NAME}
            COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake/target-${NAME}.cmake)
endmacro()

## Project Setup

project(ja2-stracciatella)

## Versioning

set(ja2-stracciatella_VERSION_MAJOR "0")
set(ja2-stracciatella_VERSION_MINOR "15")
set(ja2-stracciatella_VERSION_PATCH "x")
set(ja2-stracciatella_VERSION "${ja2-stracciatella_VERSION_MAJOR}.${ja2-stracciatella_VERSION_MINOR}.${ja2-stracciatella_VERSION_PATCH}")

## Meta Information
set(CONTACT "JA2 Stracciatella Team <no-email@ja2-stracciatella.github.io>")
set(DESCRIPTION "An improved, cross-platform, stable Jagged Alliance 2 runtime.")

## Options
# Default CMAKE_BUILD_TYPE to RelWithDebInfo
if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "RelWithDebInfo" CACHE STRING "Choose the type of build, options are: Debug RelWithDebInfo." FORCE)
endif()
set(VERSION_TAG "" CACHE STRING "Build date for nightly versioning")
set(EXTRA_DATA_DIR "" CACHE STRING "Directory for externalized data")
option(LOCAL_SDL_LIB "Use local SDL lib" OFF)
option(LOCAL_FLTK_LIB "Build with local fltk lib" OFF)
option(LOCAL_BOOST_LIB "Build with local boost lib" OFF)
option(LOCAL_RAPIDJSON_LIB "Build with local rapidjson lib" ON)
option(LOCAL_GTEST_LIB "Build with local gtest lib" ON)
option(WITH_UNITTESTS "Build with unittests" ON)
option(WITH_FIXMES "Build with fixme messages" OFF)
option(WITH_MAEMO "Build with right click mapped to F4 (menu button)" OFF)
option(BUILD_LAUNCHER "Build the ja2 launcher application" ON)

add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/dependencies")

externalproject_add(stracciatella-assets
    EXCLUDE_FROM_ALL 1
    BUILD_ALWAYS 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/assets
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets/externalized ${CMAKE_BINARY_DIR}/externalized
    COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets/unittests ${CMAKE_BINARY_DIR}/unittests
    COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/assets/mods ${CMAKE_BINARY_DIR}/mods
)

externalproject_add(libstracciatella
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR} -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
    BUILD_ALWAYS 1
)

set(JA2_DEPENDENCIES libstracciatella libsmacker-internal utf8cpp-internal)
if (LOCAL_SDL_LIB)
    set(JA2_DEPENDENCIES ${JA2_DEPENDENCIES} sdl2-internal)
endif()
if (LOCAL_FLTK_LIB)
    set(JA2_DEPENDENCIES ${JA2_DEPENDENCIES} fltk-internal)
endif()
if (LOCAL_BOOST_LIB)
    set(JA2_DEPENDENCIES ${JA2_DEPENDENCIES} boost-internal)
endif()
if (LOCAL_RAPIDJSON_LIB)
    set(JA2_DEPENDENCIES ${JA2_DEPENDENCIES} rapidjson-internal)
endif()
if (LOCAL_GTEST_LIB)
    set(JA2_DEPENDENCIES ${JA2_DEPENDENCIES} gtest-internal)
endif()

externalproject_add(ja2
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/src
    CMAKE_ARGS
        -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}
        -Dja2-stracciatella_VERSION=${ja2-stracciatella_VERSION}
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DVERSION_TAG=${VERSION_TAG}
        -DEXTRA_DATA_DIR=${EXTRA_DATA_DIR}
        -DLOCAL_SDL_LIB=${LOCAL_SDL_LIB}
        -DLOCAL_FLTK_LIB=${LOCAL_FLTK_LIB}
        -DLOCAL_BOOST_LIB=${LOCAL_BOOST_LIB}
        -DLOCAL_RAPIDJSON_LIB=${LOCAL_RAPIDJSON_LIB}
        -DLOCAL_GTEST_LIB=${LOCAL_GTEST_LIB}
        -DWITH_UNITTESTS=${WITH_UNITTESTS}
        -DWITH_FIXMES=${WITH_FIXMES}
        -DWITH_MAEMO=${WITH_MAEMO}
        -DBUILD_LAUNCHER=${BUILD_LAUNCHER}
    BUILD_ALWAYS 1
    DEPENDS ${JA2_DEPENDENCIES}
)

add_custom_target(
    ja2-stracciatella
    ALL
    DEPENDS ja2 stracciatella-assets
)

add_custom_templated_target("uninstall")

## Release building

macro(add_vagrant_build_target NAME BOX TARGET ARGS)
    add_custom_target(${NAME})
    add_custom_command(TARGET ${NAME}
            COMMAND vagrant up && vagrant ssh -c "sh /tmp/ja2-stracciatella/release/common/build.sh '${TARGET}' '${ARGS}'" && vagrant halt
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/release/${BOX})
endmacro(add_vagrant_build_target)

add_vagrant_build_target(
        "build-deb-package-on-u1404_amd64"
        "u1404_amd64"
        "package"
        "-DCMAKE_INSTALL_PREFIX=/usr -DEXTRA_DATA_DIR=/usr/share/ja2 -DLOCAL_BOOST_LIB=ON -DCPACK_GENERATOR=DEB"
)
add_vagrant_build_target(
        "build-deb-package-on-u1404_i386"
        "u1404_i386"
        "package"
        "-DCMAKE_INSTALL_PREFIX=/usr -DEXTRA_DATA_DIR=/usr/share/ja2 -DLOCAL_BOOST_LIB=ON -DCPACK_GENERATOR=DEB"
)
add_vagrant_build_target(
        "build-win-release-on-u1404_amd64"
        "u1404_amd64"
        "package"
        "-DCMAKE_TOOLCHAIN_FILE=./cmake/toolchain-mingw.cmake -DCPACK_GENERATOR=\"ZIP;NSIS\""
)
if(APPLE)
    # This can only be executed on macOS since it is not legally allowed to run the VM on any other OS
    add_vagrant_build_target(
            "build-macos-release"
            "macos"
            "package"
            "-DCMAKE_TOOLCHAIN_FILE=./cmake/toolchain-macos.cmake -DCPACK_GENERATOR=\"ZIP\""
    )
endif()
add_vagrant_build_target(
        "check-compilation-on-freebsd10"
        "freebsd-10.2"
        "all"
        ""
)
add_vagrant_build_target(
        "check-compilation-on-openbsd59"
        "openbsd-5.9"
        "all"
        "-DCMAKE_C_COMPILER=egcc -DCMAKE_CXX_COMPILER=eg++"
)


add_custom_target(build-releases)
add_custom_command(
    TARGET build-releases
    COMMAND make build-deb-package-on-u1404_amd64
    COMMAND make build-deb-package-on-u1404_i386
    COMMAND make build-win-release-on-u1404_amd64
    COMMAND make check-compilation-on-freebsd10
    COMMAND make check-compilation-on-openbsd59
)

## Rebuilding contributors.txt

add_custom_templated_target("rebuild-contributors-list")
