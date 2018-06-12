// cachoeira.cpp : Defines the entry point for the console application.
//

#include "stdafx.h"
#include "console.h"
#include "engine_system.h"
#include "event.h"
#include "entity.h"

using namespace cc;
int main()
{
    
    SystemManager::get().add_system(make_shared<EntityManager>());
    SystemManager::get().add_system(make_shared<ConsoleContext>());
    return 0;
}

