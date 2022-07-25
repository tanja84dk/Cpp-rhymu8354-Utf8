# CMakeLists.txt for Utf8
# Original © 2018 by Richard Walters https://github.com/tanja84dk/Cpp-rhymu8354-Utf8
# Recreated and updated 2022 by Tanja84dk
# © 2022 by Tanja84dk

cmake_minimum_required(VERSION 3.8)
project(Utf8)

set(Headers
    include/Utf8/Utf8.hpp
)

set(Sources
    src/Utf8.cpp
)

add_library(${PROJECT_NAME}
	${Sources}
)

add_library(libs::${PROJECT_NAME} ALIAS ${PROJECT_NAME})

target_link_libraries(${PROJECT_NAME} ${Shared_Libraries} ${Static_Libraries})

target_include_directories(${PROJECT_NAME}
	PUBLIC
		$<INSTALL_INTERFACE:include>
		$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
	PRIVATE
		${CMAKE_CURRENT_SOURCE_DIR}
)

target_compile_features(${PROJECT_NAME}
	PRIVATE
		cxx_std_14
)

set_target_properties(${PROJECT_NAME}
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
	VERSION ${PROJECT_VERSION}
	SOVERSION ${PROJECT_VERSION_MAJOR}
    FOLDER Libraries
)

if(BUILD_TESTING)
	add_subdirectory(test)
endif()