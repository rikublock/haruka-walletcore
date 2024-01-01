# Copyright © 2017-2022 Trust Wallet.
#
# This file is part of Trust. The full Trust copyright notice, including
# terms governing use, modification, and redistribution, is contained in the
# file LICENSE at the root of the source code distribution tree.

file(GLOB wasm_sources src/*.cpp src/generated/*.cpp)
file(GLOB wasm_headers src/*.h src/generated/*.h)
set(TARGET_NAME wallet-core)
set(CMAKE_EXECUTABLE_SUFFIX ".js")
add_executable(${TARGET_NAME} ${wasm_sources} ${wasm_headers})

target_link_libraries(${TARGET_NAME} TrustWalletCore)
target_include_directories(${TARGET_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/src ${CMAKE_SOURCE_DIR}/trezor-crypto/include)
target_compile_options(${TARGET_NAME} PRIVATE "-Wall")

set_target_properties(${TARGET_NAME}
        PROPERTIES
        CXX_STANDARD 20
        CXX_STANDARD_REQUIRED ON
)

# We use a set of COMPILE_FLAGS and LINK_FLAGS, see below links for more details.
# - https://emscripten.org/docs/optimizing/Optimizing-Code.html#how-to-optimize-code
# - https://emscripten.org/docs/porting/exceptions.html#emscripten-javascript-based-exception-support
# - https://github.com/emscripten-core/emscripten/blob/main/src/settings.js
# - https://emscripten.org/docs/porting/connecting_cpp_and_javascript/embind.html?highlight=lembind#embind

# STRICT: Emscripten 'strict' build mode, disables AUTO_NATIVE_LIBRARIES and AUTO_JS_LIBRARIES etc.
# ASSERTIONS: Enable runtime assertions.

# MODULARIZE=1: Emit generated JavaScript code wrapped in a function that returns a promise.
# ALLOW_MEMORY_GROWTH=1: Allowing allocating more memory from the system as necessary.
# DYNAMIC_EXECUTION=0: Do not emit eval() and new Function() in the generated JavaScript code.
# DISABLE_EXCEPTION_CATCHING=0: Enable Emscripten’s JavaScript-based exceptions support.
# EXPORT_EXCEPTION_HANDLING_HELPERS=1: Make the exception message printing function 'getExceptionMessage' available.

# -O2: good old code optimization level for release.
# -fexceptions: allow exception handling
# --bind: Link Embind library.
# --no-entry: Skip main entry point because it's built as a library.
# --closure 1: Enable Emscripten closure compiler to minimize generated JavaScript code size.

set_target_properties(${TARGET_NAME} 
        PROPERTIES 
        COMPILE_FLAGS "-O2 -fexceptions -sSTRICT -sUSE_BOOST_HEADERS=1 -sDISABLE_EXCEPTION_CATCHING=0"
        LINK_FLAGS "--bind --no-entry --closure 1 -O2 -fexceptions -sSTRICT -sDISABLE_EXCEPTION_CATCHING=0 -sASSERTIONS -sMODULARIZE=1 -sALLOW_MEMORY_GROWTH=1 -sDYNAMIC_EXECUTION=0 -sEXPORT_EXCEPTION_HANDLING_HELPERS=1 -sEXPORTED_FUNCTIONS=['_setThrew']"
)
