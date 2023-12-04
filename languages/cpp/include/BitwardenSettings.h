#pragma once

#include <string>

class BitwardenSettings {
public:
    BitwardenSettings() = default;
    ~BitwardenSettings() = default;

    const std::string& get_api_url() const { return api_url; }
    void set_api_url(const std::string& value) { api_url = value; }

    const std::string& get_identity_url() const { return identity_url; }
    void set_identity_url(const std::string& value) { identity_url = value; }

private:
    std::string api_url;
    std::string identity_url;
};
