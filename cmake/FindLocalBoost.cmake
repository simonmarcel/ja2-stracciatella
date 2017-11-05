set(Boost_INCLUDE_DIRS ${CMAKE_INSTALL_PREFIX}/dependencies/lib-boost/include)

find_library(
    Boost_LIBRARIES
    NAMES boost
    PATH_SUFFIXES lib
    PATHS ${CMAKE_INSTALL_PREFIX}/dependencies/lib-boost
)

INCLUDE(FindPackageHandleStandardArgs)

FIND_PACKAGE_HANDLE_STANDARD_ARGS(Boost REQUIRED_VARS Boost_LIBRARIES Boost_INCLUDE_DIRS)
