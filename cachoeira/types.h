#pragma once
#include <string>
#include <vector>
#include <map>
#include <chrono>
#include "result.h"
#include "timing.h"

namespace cc {

    // redirecting some commonly used types
    using std::string;
    using std::stringbuf;
    using std::string_view;
    using std::vector;
    using std::map;
    using std::hash;
    using std::shared_ptr;
    using std::make_shared;
    using std::unique_ptr;
    using std::make_unique;

    // bringing some neat literals to the namespace
    using namespace std::string_literals;
    using namespace std::chrono_literals;

}