#pragma once

#include <string>
#include <functional>
#include "BitwardenLibrary.h"
#include "schemas.hpp"
#include <iostream>

using namespace Bitwarden::Sdk;

class CommandRunner {
public:
    CommandRunner(BitwardenLibrary* library, void* client);

    template <typename T, typename Func>
    T runCommand(const Command& command, Func deserializer);



private:
    BitwardenLibrary* library;
    void* client;

    std::string commandToString(const Command& command);
    nlohmann::json filterNullObjects(const nlohmann::json& input);
};

template <typename T, typename Func>
T CommandRunner::runCommand(const Command& command, Func deserializer) {
    try {
        // Serialize the Command object to a JSON string
        std::string jsonString = commandToString(command);
        const char* jsonCStr = jsonString.c_str();
        const char* response = library->run_command(jsonCStr, client);

        // Deserialize the response using the provided deserializer function
        return deserializer(response);
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        throw ex;
    }
}
