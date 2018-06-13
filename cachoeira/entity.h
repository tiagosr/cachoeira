#pragma once
#include "engine_system.h"
#include "types.h"
#include "event.h"

namespace cc {

    class BaseEntitySystem : public non_copyable {
    public:
        using Family = size_t;
        virtual ~BaseEntitySystem() {}

        virtual void configure() {

        }

        static Family s_family_counter;
    };

    template <typename Derived>
    class System : public BaseEntitySystem {
    public:
        virtual ~System() {}

    private:
        friend class SystemManager;
        static Family family() {
            static Family family = s_family_counter++;
            return family;
        }
    };

    class UpdateManager : public non_copyable {

    };

    class SystemManager : public IEngineSystem, public non_copyable {
    public:
        template <typename System>
        void add(shared_ptr<System> system) {
            m_systems.insert(make_pair(System::family(), system));
        }

        template <typename System, typename... Args>
        shared_ptr<System> add(Args &&... args) {
            shared_ptr<System> system = make_shared(std::forward<Args>(args)...);
            add(system);
            return system;
        }

    private:
        unordered_map<BaseEntitySystem::Family, std::shared_ptr<BaseEntitySystem>> m_systems;
    };

    class EntityManager : public IEngineSystem, public non_copyable {
    public:
        virtual ~EntityManager() {}
        virtual string get_name() const override {
            return "EntityManager";
        }
        virtual Result<void, string> setup() override;
        virtual Result<void, string> setup_console_vars() override;
        virtual Result<void, string> teardown() override;
        explicit EntityManager(EventManager &event_manager);
    };
}
