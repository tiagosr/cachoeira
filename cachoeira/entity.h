#pragma once
#include "engine_system.h"
#include "types.h"

namespace cc {


    class EntityManager: public IEngineSystem {
    public:
        virtual ~EntityManager() {}
        virtual string get_name() const override;
        virtual Result<void, string> setup() override;
        virtual Result<void, string> setup_console_vars() override;
        virtual Result<void, string> teardown() override;
    };
}
