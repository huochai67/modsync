cmake_minimum_required(VERSION 3.20)

project(update CXX)

set(CMAKE_CXX_STANDARD 23) 
set(CMAKE_CXX_STANDARD_REQUIRED True) 

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake/")

include(cmake/cpr.cmake)
include(cmake/spdlog.cmake)
include(cmake/imgui.cmake)

find_package(SDL2 REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(Vulkan REQUIRED)

set(SRC_DIR "${PROJECT_SOURCE_DIR}/srcs")

if(MSVC)
	add_compile_options("$<$<C_COMPILER_ID:MSVC>:/utf-8>")
	add_compile_options("$<$<CXX_COMPILER_ID:MSVC>:/utf-8>")
	add_executable(modsync WIN32  "${SRC_DIR}/modsync.cpp" "${SRC_DIR}/gui_service.cpp" "${SRC_DIR}/utfconvert.cpp")
	add_executable(listgenerater WIN32 "${SRC_DIR}/listgenerater.cpp" "${SRC_DIR}/gui_service.cpp" "${SRC_DIR}/utfconvert.cpp")
else()
	add_executable(modsync "${SRC_DIR}/modsync.cpp" "${SRC_DIR}/gui_service.cpp" "${SRC_DIR}/utfconvert.cpp")
	add_executable(listgenerater "${SRC_DIR}/listgenerater.cpp" "${SRC_DIR}/gui_service.cpp" "${SRC_DIR}/utfconvert.cpp")
endif()

target_include_directories(modsync PRIVATE
	"${imgui_SOURCE_DIR}"
	"${imgui_SOURCE_DIR}/backends"
	)
target_link_libraries(modsync PRIVATE cpr::cpr OpenSSL::SSL OpenSSL::Crypto)
target_link_libraries(modsync PRIVATE imgui ${SDL2_LIBRARY} Vulkan::Vulkan Vulkan::Headers)

target_link_libraries(listgenerater PRIVATE OpenSSL::SSL OpenSSL::Crypto)
target_link_libraries(listgenerater PRIVATE Vulkan::Vulkan Vulkan::Headers)