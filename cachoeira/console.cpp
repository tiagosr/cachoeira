#include "console.h"

using namespace cc;

Result<void, string> ConsoleContext::setup() {
    return Ok();
}

Result<void, string> ConsoleContext::setup_console_vars() {
    return Ok();
}

Result<void, string> cc::ConsoleContext::teardown() {
    return Ok();
}
