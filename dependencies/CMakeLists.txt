include(ExternalProject)

externalproject_add(fltk-internal
    EXCLUDE_FROM_ALL 1
    URL "http://fltk.org/pub/fltk/1.3.4/fltk-1.3.4-2-source.tar.gz"
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/dependencies/lib-fltk
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DOPTION_USE_GL=OFF -DOPTION_USE_SYSTEM_ZLIB=OFF -DOPTION_USE_SYSTEM_LIBJPEG=OFF
        -DOPTION_USE_SYSTEM_LIBPNG=OFF -DOPTION_USE_XINERAMA=OFF -DOPTION_USE_XFIXES=OFF
        -DOPTION_USE_XCURSOR=OFF -DOPTION_USE_XFT=OFF -DOPTION_USE_XRENDER=OFF
        -DOPTION_USE_XDBE=OFF -DOPTION_BUILD_EXAMPLES=OFF
)

externalproject_add(boost-internal
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-boost
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/dependencies/lib-boost -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
)

externalproject_add(gtest-internal
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-gtest
    CMAKE_ARGS -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
    INSTALL_COMMAND
        ${CMAKE_COMMAND} -E make_directory ${CMAKE_BINARY_DIR}/dependencies/lib-gtest/lib
    COMMAND
        ${CMAKE_COMMAND} -E copy ${CMAKE_STATIC_LIBRARY_PREFIX}gtest${CMAKE_STATIC_LIBRARY_SUFFIX} ${CMAKE_BINARY_DIR}/dependencies/lib-gtest/lib/
    COMMAND
        ${CMAKE_COMMAND} -E copy ${CMAKE_STATIC_LIBRARY_PREFIX}gtest_main${CMAKE_STATIC_LIBRARY_SUFFIX} ${CMAKE_BINARY_DIR}/dependencies/lib-gtest/lib/
    COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-gtest/include ${CMAKE_BINARY_DIR}/dependencies/lib-gtest/include
)

externalproject_add(rapidjson-internal
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-rapidjson
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-rapidjson/rapidjson-1.1.0/include ${CMAKE_BINARY_DIR}/dependencies/lib-rapidjson/include
)

externalproject_add(libsmacker-internal
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-smacker
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/dependencies/lib-smacker -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
)

externalproject_add(utf8cpp-internal
    EXCLUDE_FROM_ALL 1
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-utf8cpp
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND
        ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-utf8cpp/source ${CMAKE_BINARY_DIR}/dependencies/lib-utf8cpp/include
)

if (APPLE)
    externalproject_add(sdl2-internal
        EXCLUDE_FROM_ALL 1
        SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-macos
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ""
        INSTALL_COMMAND
            ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-macos ${CMAKE_BINARY_DIR}/dependencies/lib-sdl2
    )
elseif(MSVC)
    externalproject_add(sdl2-internal
        EXCLUDE_FROM_ALL 1
        SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-VC
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ""
        INSTALL_COMMAND
            ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-VC ${CMAKE_BINARY_DIR}/dependencies/lib-sdl2
    )
elseif(MINGW)
    externalproject_add(sdl2-internal
        EXCLUDE_FROM_ALL 1
        SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-mingw
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ""
        INSTALL_COMMAND
            ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/lib-SDL2-2.0.4-mingw ${CMAKE_BINARY_DIR}/dependencies/lib-sdl2
    )
else()
    externalproject_add(sdl2-internal
        EXCLUDE_FROM_ALL 1
        URL "https://www.libsdl.org/release/SDL2-2.0.7.tar.gz"
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/dependencies/lib-sdl2 -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
    )
endif()
