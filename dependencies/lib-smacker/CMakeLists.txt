cmake_minimum_required(VERSION 2.8)

project(libsmacker)

set(source_files
    libsmacker/smacker.c
    libsmacker/smk_hufftree.c
    libsmacker/smk_bitstream.c
)

add_library(smacker ${source_files})

file(GLOB HEADERS libsmacker/*.h)
install(TARGETS smacker DESTINATION lib)
install(FILES ${HEADERS} DESTINATION include)
