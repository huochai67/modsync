include(FetchContent)
FetchContent_Declare(
    imgui
    GIT_REPOSITORY https://github.com/ocornut/imgui.git
    GIT_TAG        94c46d74869ec991c101c187088da0f25d6c8e40
    GIT_PROGRESS TRUE
)
message("ImGui")
FetchContent_GetProperties(imgui)

find_package(SDL2 REQUIRED)
find_package(Vulkan REQUIRED)

if(NOT imgui_POPULATED)
    FetchContent_Populate(imgui)

    file(GLOB SRC_IMGUI
        "${imgui_SOURCE_DIR}/*.cpp"
        "${imgui_SOURCE_DIR}/backends/imgui_impl_vulkan.cpp"
        "${imgui_SOURCE_DIR}/backends/imgui_impl_sdl2.cpp"
        "${imgui_SOURCE_DIR}/misc/cpp/imgui_stdlib.cpp"
    )

    add_library(imgui STATIC ${SRC_IMGUI} )
    source_group(TREE ${imgui_SOURCE_DIR} PREFIX "imgui" FILES ${SRC_IMGUI})
    target_include_directories(imgui PRIVATE
        "${imgui_SOURCE_DIR}"
        "${imgui_SOURCE_DIR}/backends"
        "${imgui_SOURCE_DIR}/misc/cpp"
        "${SDL2_INCLUDE_DIR}"
    )
    target_link_libraries(imgui PRIVATE Vulkan::Vulkan Vulkan::Headers)
endif()
set_property(TARGET imgui PROPERTY CXX_STANDARD 23)