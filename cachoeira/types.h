#pragma once
#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <chrono>
#include <cstdint>
#include "result.h"
#include "timing.h"

namespace cc {

    // redirecting some commonly used types
    using std::string;
    using std::stringbuf;
    using std::string_view;
    using std::vector;
    using std::pair;
    using std::make_pair;
    using std::map;
    using std::unordered_map;
    using std::hash;
    using std::shared_ptr;
    using std::make_shared;
    using std::unique_ptr;
    using std::make_unique;
    using std::weak_ptr;
    using std::bad_weak_ptr;
    using std::size_t;
    using std::ptrdiff_t;


    // bringing some neat literals to the namespace
    using namespace std::string_literals;
    using namespace std::chrono_literals;


    class non_copyable {
    protected:
        non_copyable() = default;
        ~non_copyable() = default;
        non_copyable(const non_copyable&) = delete;
        non_copyable& operator=(const non_copyable&) = delete;
    };
}