include(FetchContent)

FetchContent_Declare(
    cpr
    GIT_REPOSITORY https://github.com/libcpr/cpr.git
    GIT_TAG        6b3e3531d3e5781b4b19e7cf64ed76035bf59d37
    GIT_PROGRESS TRUE
) 
message("cpr")
FetchContent_MakeAvailable(cpr)

set(BUILD_SHARED_LIBS OFF)
set(BUILD_TESTING ${BUILD_TESTING_BEFORE} CACHE INTERNAL "" FORCE)