#pragma once
#include <string>
#include <map>
#include "types.h"

namespace cc {

    // Interface for a system to be registered in Cachoeira
    class IEngineSystem {
    public:
        virtual ~IEngineSystem() {}
        virtual string get_name() const = 0;
        virtual Result<void, string> setup() = 0;
        virtual Result<void, string> setup_console_vars() {
            return Ok();
        }
        virtual Result<void, string> teardown() = 0;
    };

    class SystemManager : public non_copyable {
        map<const string, shared_ptr<IEngineSystem>> systems;
        static unique_ptr<SystemManager> instance;
        SystemManager();
        ~SystemManager();
    public:
        Result<void, string> add_system(shared_ptr<IEngineSystem> system);
        Result<void, string> remove_system_by_name(string_view name);
        Result<void, string> remove_system(IEngineSystem* const system_ptr) {
            if (system_ptr == nullptr) {
                return Err<string>("System is null!");
            }
            return remove_system_by_name(system_ptr->get_name());
        }
        SystemManager& get();
    };
}


