#include "entity.h"

using namespace cc;

Result<void, string> EntityManager::setup() {
    return Ok();
}

Result<void, string> EntityManager::setup_console_vars() {
    return Ok();
}

Result<void, string> EntityManager::teardown() {
    return Ok();
}
