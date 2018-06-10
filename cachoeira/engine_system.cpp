
#include "engine_system.h"
#include "logging.h"


using namespace cc;

SystemManager::SystemManager() {

}

SystemManager::~SystemManager() {

}

Result<void, string> SystemManager::add_system(shared_ptr<IEngineSystem> system) {
    if (systems.find(system->get_name()) != systems.end())
        return Err<string>("System with this name already in place!");
    systems[system->get_name()] = system;
    return Ok();
}

Result<void, string> SystemManager::remove_system_by_name(string_view name) {
    auto it = systems.find(std::string(name));
    if (it == systems.end()) {
        return Err<string>("No system with this name");
    }
    // do the teardown for this system
    it->second->teardown();
    systems.erase(string(name));
    return Ok();
}

SystemManager& SystemManager::get() {
    if (!instance) {
        instance = make_unique<SystemManager>();
    }
    return *instance;
}


