#pragma once

#include <iostream>
#include "engine_system.h"
#include "types.h"

namespace cc {
    namespace Logging {
        class Text: public non_copyable {
        public:
            explicit Text(char* owned_str): _str(owned_str) {}
            ~Text() { free(_str); }
            Text(Text&& t) {
                _str = t._str;
                t._str = nullptr;
            }
            void operator=(Text&& t) = delete;
            
        private:
            char* _str;
        };
    }
}

