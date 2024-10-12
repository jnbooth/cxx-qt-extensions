#include <cstddef>
#include <cstdint>

// https://github.com/dtolnay/cxx/blob/5a7f93bd1361857c1bcd41f203d55f13ad0ccdf9/src/cxx.cc#L391
static_assert(sizeof(::std::size_t) == sizeof(::std::uintptr_t),
              "unsupported size_t size");
static_assert(alignof(::std::size_t) == alignof(::std::uintptr_t),
              "unsupported size_t alignment");
