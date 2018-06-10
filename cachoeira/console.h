#pragma once

#include <string>
#include <optional>
#include <map>
#include "result.h"
#include "engine_system.h"

namespace cc {
    using ConsoleVarResult = Result<void, std::string>;
    class IConsoleVar {
    public:
        virtual ~IConsoleVar() {}
        virtual ConsoleVarResult set(std::string_view) = 0;
        virtual std::string_view get() const = 0;
    };

    class ConsoleVarString : public IConsoleVar {
        std::string str;
    public:
        ConsoleVarResult set(std::string_view view) override {
            str = view;
            return Ok();
        }
        std::string_view get() const override {
            return str;
        }
    };

    class ConsoleContext : public IEngineSystem {
        using VarMap = std::map<const std::string, std::shared_ptr<IConsoleVar>>;
        VarMap vars;
    public:

        ConsoleContext() {

        }

        void add_var(std::string_view key, std::shared_ptr<IConsoleVar> var) {
            vars.emplace(key, var);
        }

        using VarAddResult = std::optional<ConsoleVarResult>;
        
        VarAddResult write_var(std::string_view key, std::string_view val) {
            VarMap::iterator it = vars.find(std::string(key));
            if (it != vars.end()) {
                return VarAddResult(it->second->set(val));
            }
            else {
                return VarAddResult();
            }
        }

        using VarQueryResult = std::optional<std::string_view>;

        VarQueryResult query_var(std::string_view key) {
            VarMap::iterator it = vars.find(std::string(key));
            if (it == vars.end()) return VarQueryResult();
            return VarQueryResult(it->second->get());
        }

    };
}
