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
    // Serialize the Command object to a JSON string
    std::string jsonString = commandToString(command);
    const char* jsonCStr = jsonString.c_str();
    const char* response = library->run_command(jsonCStr, client);

    // Deserialize the response using the provided deserializer function
    T deserialized = deserializer(response);

    // Unwrap the response and throw an exception if it was not successful
    if (!deserialized.get_success()) {
        throw std::runtime_error(*deserialized.get_error_message());
    }

    return deserialized;
}

