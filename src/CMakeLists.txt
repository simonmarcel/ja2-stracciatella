cmake_minimum_required (VERSION 2.8)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_CURRENT_SOURCE_DIR}/../cmake")

project(ja2)
set(JA2_BINARY "ja2")
set(LAUNCHER_BINARY "ja2-launcher")
set(CMAKE_CXX_STANDARD 98)

set(GAME_VERSION "v${ja2-stracciatella_VERSION}")
if (NOT (VERSION_TAG STREQUAL ""))
    set(GAME_VERSION "${GAME_VERSION}-${VERSION_TAG}")
endif()

message(STATUS "Setting game version to '${GAME_VERSION}'")
add_definitions(-DGAME_VERSION="\\"${GAME_VERSION}\\"")

message(STATUS "Setting extra data dir to '${EXTRA_DATA_DIR}'")
add_definitions(-DEXTRA_DATA_DIR="${EXTRA_DATA_DIR}")

if (WITH_FIXMES)
    message(STATUS "Building with fixme messages" )
    add_definitions(-DWITH_FIXMES)
endif()

if (WITH_MAEMO)
    message(STATUS "Building with right click mapped to F4 (menu button)" )
    add_definitions(-DWITH_MAEMO)
endif()

if (WITH_SOUND_DEBUG)
    message(STATUS "Building with sound debug" )
    add_definitions(-DWITH_SOUND_DEBUG)
endif()

if (WITH_UNITTESTS)
    message(STATUS "Building with unittests" )
endif()

if (LOCAL_SDL_LIB)
    set(ENV{SDL2DIR} "${CMAKE_INSTALL_DIR}/dependencies/lib-sdl2")
endif()
find_package(SDL2 REQUIRED)
message(STATUS "SDL2 Libraries: ${SDL2_LIBRARY} SDL2 Includes: ${SDL2_INCLUDE_DIR}")

if (LOCAL_BOOST_LIB)
    find_package(LocalBoost REQUIRED)
else()
    find_package(Boost REQUIRED COMPONENTS filesystem system)
endif()
message(STATUS "Boost Libraries: ${Boost_LIBRARIES} Boost Includes: ${Boost_INCLUDE_DIRS}")

find_package(RapidJSON REQUIRED)
message(STATUS "RapidJSON Includes: ${RAPIDJSON_INCLUDE_DIRS}")

find_package(LocalUtf8Cpp REQUIRED)
message(STATUS "Utf8Cpp Includes: ${UTF8CPP_INCLUDE_DIRS}")

find_package(LocalSmacker REQUIRED)
message(STATUS "Smacker Libraries: ${SMACKER_LIBRARIES} Smacker Includes: ${SMACKER_INCLUDE_DIRS}")

if (WITH_UNITTESTS)
    if (LOCAL_GTEST_LIB)
        set(GTEST_ROOT "${CMAKE_INSTALL_PREFIX}/dependencies/lib-gtest")
    endif()
    find_package(GTest REQUIRED)
    message(STATUS "GTest Libraries: ${GTEST_LIBRARIES} GTest Includes: ${GTEST_INCLUDE_DIRS}")
endif()


find_package(Stracciatella REQUIRED)
message(STATUS "Libstracciatella library: ${STRACCIATELLA_LIBRARY}")

if (BUILD_LAUNCHER)
    if (LOCAL_FLTK_LIB)
        set(FLTK_SKIP_OPENGL ON)
        set(FLTK_FLUID_EXECUTABLE "${CMAKE_INSTALL_PREFIX}/dependencies/lib-fltk/bin/fluid")
    endif()

    find_package(FLTK)
    if(NOT FLTK_FOUND)
        message(STATUS "FLTK not found. Disabling launcher.")
        set(BUILD_LAUNCHER OFF)
    else()
        message(STATUS "Fltk Libraries: ${FLTK_LIBRARIES} Fltk Includes: ${FLTK_INCLUDE_DIR}")
    endif()
endif()

set(JA2_INCLUDES "")
set(JA2_SOURCES "")
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/externalized")
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/game")
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/sgp")
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/slog")

if(BUILD_LAUNCHER)
    set(LAUNCHER_INCLUDES ${FLTK_INCLUDE_DIR})
    set(LAUNCHER_SOURCES "")
    add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/launcher")
endif()

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${JA2_INCLUDES}
    ${Boost_INCLUDE_DIRS}
    ${RAPIDJSON_INCLUDE_DIRS}
    ${SDL2_INCLUDE_DIR}
    ${SMACKER_INCLUDE_DIRS}
    ${UTF8CPP_INCLUDE_DIRS}
)

if(BUILD_LAUNCHER)
    include_directories(${LAUNCHER_INCLUDES})
endif()

foreach(FILE ${JA2_SOURCES})
  get_filename_component(PARENT_DIR "${FILE}" PATH)
  file(RELATIVE_PATH GROUP "${CMAKE_CURRENT_SOURCE_DIR}" "${PARENT_DIR}")
  string(REPLACE "/" "\\" GROUP "${GROUP}")
  source_group("${GROUP}" FILES "${FILE}")
endforeach()

if (WITH_UNITTESTS)
    include_directories(${GTEST_INCLUDE_DIRS})
    add_definitions("-DWITH_UNITTESTS")
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${CFLAGS}")


add_executable(${JA2_BINARY} ${JA2_SOURCES})
set_target_properties(${JA2_BINARY} PROPERTIES INSTALL_RPATH_USE_LINK_PATH TRUE)
target_link_libraries(${JA2_BINARY} ${SDL2_LIBRARY} ${Boost_LIBRARIES} ${SMACKER_LIBRARIES} ${GTEST_LIBRARIES} ${STRACCIATELLA_LIBRARY} slog)
install(TARGETS ${JA2_BINARY} DESTINATION .)

if(BUILD_LAUNCHER)
    add_executable(${LAUNCHER_BINARY} ${LAUNCHER_SOURCES})
    set_target_properties(${LAUNCHER_BINARY} PROPERTIES INSTALL_RPATH_USE_LINK_PATH TRUE)

    set(ADDITIONAL_LIBS "")
    if (UNIX AND NOT MINGW)
        set(ADDITIONAL_LIBS dl)
    endif()
    target_link_libraries(${LAUNCHER_BINARY} ${FLTK_LIBRARIES} ${STRACCIATELLA_LIBRARY} ${ADDITIONAL_LIBS} slog)
    install(TARGETS ${LAUNCHER_BINARY} DESTINATION .)
endif()
