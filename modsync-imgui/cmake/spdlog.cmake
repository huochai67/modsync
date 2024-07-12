include(FetchContent)

FetchContent_Declare(
    spdlog
    GIT_REPOSITORY https://github.com/gabime/spdlog.git
    GIT_TAG        ddce42155e67589a8b1534c4935242f759c07646
    GIT_PROGRESS TRUE
) 
message("spdlog")
FetchContent_MakeAvailable(spdlog)

set(BUILD_TESTING ${BUILD_TESTING_BEFORE} CACHE INTERNAL "" FORCE)