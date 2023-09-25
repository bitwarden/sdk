#include "CommandRunner.h"
#include <nlohmann/json.hpp>
#include <iostream>
#include <string>


CommandRunner::CommandRunner(BitwardenLibrary* library, void* client) : library(library), client(client) {}

// Function to recursively filter out objects with all null values
nlohmann::json CommandRunner::filterNullObjects(const nlohmann::json& input) {
    nlohmann::json result;
    
    for (auto it = input.begin(); it != input.end(); ++it) {
        if (!it.value().is_null()) {
            if (it.value().is_object()) {
                // Recursively filter nested objects
                json nestedFiltered = filterNullObjects(it.value());
                if (!nestedFiltered.empty()) {
                    result[it.key()] = nestedFiltered;
                }
            } else {
                result[it.key()] = it.value();
            }
        }
    }
    
    return result;
}

// Implement the commandToString function
std::string CommandRunner::commandToString(const Command& command) {
    try {
        // Create an nlohmann::json object from the Command object
        nlohmann::json jsonCommand;
        nlohmann::json filteredJsonCommand;

        quicktype::to_json(jsonCommand, command);

        filteredJsonCommand = filterNullObjects(jsonCommand);
        
        // Convert the JSON to a string
        std::string jsonCommandString = filteredJsonCommand.dump();

        return jsonCommandString;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        throw ex;
    }
}
