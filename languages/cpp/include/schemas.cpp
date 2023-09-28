//  To parse this JSON data, first install
//
//      Boost     http://www.boost.org
//      json.hpp  https://github.com/nlohmann/json
//
//  Then include this file, and then do
//
//     ClientSettings data = nlohmann::json::parse(jsonString);
//     Command data = nlohmann::json::parse(jsonString);
//     DocRef data = nlohmann::json::parse(jsonString);
//     ResponseForApiKeyLoginResponse data = nlohmann::json::parse(jsonString);
//     ResponseForFingerprintResponse data = nlohmann::json::parse(jsonString);
//     ResponseForPasswordLoginResponse data = nlohmann::json::parse(jsonString);
//     ResponseForProjectResponse data = nlohmann::json::parse(jsonString);
//     ResponseForProjectsDeleteResponse data = nlohmann::json::parse(jsonString);
//     ResponseForProjectsResponse data = nlohmann::json::parse(jsonString);
//     ResponseForSecretIdentifiersResponse data = nlohmann::json::parse(jsonString);
//     ResponseForSecretResponse data = nlohmann::json::parse(jsonString);
//     ResponseForSecretsDeleteResponse data = nlohmann::json::parse(jsonString);
//     ResponseForSecretsResponse data = nlohmann::json::parse(jsonString);
//     ResponseForSyncResponse data = nlohmann::json::parse(jsonString);
//     ResponseForUserApiKeyResponse data = nlohmann::json::parse(jsonString);

#pragma once

#include <boost/optional.hpp>
#include <boost/variant.hpp>
#include "json.hpp"

#include <boost/optional.hpp>
#include <stdexcept>
#include <regex>
#include <unordered_map>

#ifndef NLOHMANN_OPT_HELPER
#define NLOHMANN_OPT_HELPER
namespace nlohmann {
    template <typename T>
    struct adl_serializer<std::shared_ptr<T>> {
        static void to_json(json & j, const std::shared_ptr<T> & opt) {
            if (!opt) j = nullptr; else j = *opt;
        }

        static std::shared_ptr<T> from_json(const json & j) {
            if (j.is_null()) return std::make_shared<T>(); else return std::make_shared<T>(j.get<T>());
        }
    };
    template <typename T>
    struct adl_serializer<boost::optional<T>> {
        static void to_json(json & j, const boost::optional<T> & opt) {
            if (!opt) j = nullptr; else j = *opt;
        }

        static boost::optional<T> from_json(const json & j) {
            if (j.is_null()) return boost::optional<T>(); else return boost::optional<T>(j.get<T>());
        }
    };
}
#endif

namespace quicktype {
    using nlohmann::json;

    class ClassMemberConstraints {
        private:
        boost::optional<int64_t> min_int_value;
        boost::optional<int64_t> max_int_value;
        boost::optional<double> min_double_value;
        boost::optional<double> max_double_value;
        boost::optional<size_t> min_length;
        boost::optional<size_t> max_length;
        boost::optional<std::string> pattern;

        public:
        ClassMemberConstraints(
            boost::optional<int64_t> min_int_value,
            boost::optional<int64_t> max_int_value,
            boost::optional<double> min_double_value,
            boost::optional<double> max_double_value,
            boost::optional<size_t> min_length,
            boost::optional<size_t> max_length,
            boost::optional<std::string> pattern
        ) : min_int_value(min_int_value), max_int_value(max_int_value), min_double_value(min_double_value), max_double_value(max_double_value), min_length(min_length), max_length(max_length), pattern(pattern) {}
        ClassMemberConstraints() = default;
        virtual ~ClassMemberConstraints() = default;

        void set_min_int_value(int64_t min_int_value) { this->min_int_value = min_int_value; }
        auto get_min_int_value() const { return min_int_value; }

        void set_max_int_value(int64_t max_int_value) { this->max_int_value = max_int_value; }
        auto get_max_int_value() const { return max_int_value; }

        void set_min_double_value(double min_double_value) { this->min_double_value = min_double_value; }
        auto get_min_double_value() const { return min_double_value; }

        void set_max_double_value(double max_double_value) { this->max_double_value = max_double_value; }
        auto get_max_double_value() const { return max_double_value; }

        void set_min_length(size_t min_length) { this->min_length = min_length; }
        auto get_min_length() const { return min_length; }

        void set_max_length(size_t max_length) { this->max_length = max_length; }
        auto get_max_length() const { return max_length; }

        void set_pattern(const std::string &  pattern) { this->pattern = pattern; }
        auto get_pattern() const { return pattern; }
    };

    class ClassMemberConstraintException : public std::runtime_error {
        public:
        ClassMemberConstraintException(const std::string &  msg) : std::runtime_error(msg) {}
    };

    class ValueTooLowException : public ClassMemberConstraintException {
        public:
        ValueTooLowException(const std::string &  msg) : ClassMemberConstraintException(msg) {}
    };

    class ValueTooHighException : public ClassMemberConstraintException {
        public:
        ValueTooHighException(const std::string &  msg) : ClassMemberConstraintException(msg) {}
    };

    class ValueTooShortException : public ClassMemberConstraintException {
        public:
        ValueTooShortException(const std::string &  msg) : ClassMemberConstraintException(msg) {}
    };

    class ValueTooLongException : public ClassMemberConstraintException {
        public:
        ValueTooLongException(const std::string &  msg) : ClassMemberConstraintException(msg) {}
    };

    class InvalidPatternException : public ClassMemberConstraintException {
        public:
        InvalidPatternException(const std::string &  msg) : ClassMemberConstraintException(msg) {}
    };

    inline void CheckConstraint(const std::string &  name, const ClassMemberConstraints & c, int64_t value) {
        if (c.get_min_int_value() != boost::none && value < *c.get_min_int_value()) {
            throw ValueTooLowException ("Value too low for " + name + " (" + std::to_string(value) + "<" + std::to_string(*c.get_min_int_value()) + ")");
        }

        if (c.get_max_int_value() != boost::none && value > *c.get_max_int_value()) {
            throw ValueTooHighException ("Value too high for " + name + " (" + std::to_string(value) + ">" + std::to_string(*c.get_max_int_value()) + ")");
        }
    }

    inline void CheckConstraint(const std::string &  name, const ClassMemberConstraints & c, double value) {
        if (c.get_min_double_value() != boost::none && value < *c.get_min_double_value()) {
            throw ValueTooLowException ("Value too low for " + name + " (" + std::to_string(value) + "<" + std::to_string(*c.get_min_double_value()) + ")");
        }

        if (c.get_max_double_value() != boost::none && value > *c.get_max_double_value()) {
            throw ValueTooHighException ("Value too high for " + name + " (" + std::to_string(value) + ">" + std::to_string(*c.get_max_double_value()) + ")");
        }
    }

    inline void CheckConstraint(const std::string &  name, const ClassMemberConstraints & c, const std::string &  value) {
        if (c.get_min_length() != boost::none && value.length() < *c.get_min_length()) {
            throw ValueTooShortException ("Value too short for " + name + " (" + std::to_string(value.length()) + "<" + std::to_string(*c.get_min_length()) + ")");
        }

        if (c.get_max_length() != boost::none && value.length() > *c.get_max_length()) {
            throw ValueTooLongException ("Value too long for " + name + " (" + std::to_string(value.length()) + ">" + std::to_string(*c.get_max_length()) + ")");
        }

        if (c.get_pattern() != boost::none) {
            std::smatch result;
            std::regex_search(value, result, std::regex( *c.get_pattern() ));
            if (result.empty()) {
                throw InvalidPatternException ("Value doesn't match pattern for " + name + " (" + value +" != " + *c.get_pattern() + ")");
            }
        }
    }

    #ifndef NLOHMANN_UNTYPED_quicktype_HELPER
    #define NLOHMANN_UNTYPED_quicktype_HELPER
    inline json get_untyped(const json & j, const char * property) {
        if (j.find(property) != j.end()) {
            return j.at(property).get<json>();
        }
        return json();
    }

    inline json get_untyped(const json & j, std::string property) {
        return get_untyped(j, property.data());
    }
    #endif

    #ifndef NLOHMANN_OPTIONAL_quicktype_HELPER
    #define NLOHMANN_OPTIONAL_quicktype_HELPER
    template <typename T>
    inline std::shared_ptr<T> get_heap_optional(const json & j, const char * property) {
        auto it = j.find(property);
        if (it != j.end() && !it->is_null()) {
            return j.at(property).get<std::shared_ptr<T>>();
        }
        return std::shared_ptr<T>();
    }

    template <typename T>
    inline std::shared_ptr<T> get_heap_optional(const json & j, std::string property) {
        return get_heap_optional<T>(j, property.data());
    }
    template <typename T>
    inline boost::optional<T> get_stack_optional(const json & j, const char * property) {
        auto it = j.find(property);
        if (it != j.end() && !it->is_null()) {
            return j.at(property).get<boost::optional<T>>();
        }
        return boost::optional<T>();
    }

    template <typename T>
    inline boost::optional<T> get_stack_optional(const json & j, std::string property) {
        return get_stack_optional<T>(j, property.data());
    }
    #endif

    /**
     * Device type to send to Bitwarden. Defaults to SDK
     */
    enum class DeviceType : int { ANDROID, ANDROID_AMAZON, CHROME_BROWSER, CHROME_EXTENSION, EDGE_BROWSER, EDGE_EXTENSION, FIREFOX_BROWSER, FIREFOX_EXTENSION, IE_BROWSER, I_OS, LINUX_DESKTOP, MAC_OS_DESKTOP, OPERA_BROWSER, OPERA_EXTENSION, SAFARI_BROWSER, SAFARI_EXTENSION, SDK, UNKNOWN_BROWSER, UWP, VIVALDI_BROWSER, VIVALDI_EXTENSION, WINDOWS_DESKTOP };

    /**
     * Basic client behavior settings. These settings specify the various targets and behavior
     * of the Bitwarden Client. They are optional and uneditable once the client is
     * initialized.
     *
     * Defaults to
     *
     * ``` # use bitwarden::client::client_settings::{ClientSettings, DeviceType}; # use
     * assert_matches::assert_matches; let settings = ClientSettings { identity_url:
     * "https://identity.bitwarden.com".to_string(), api_url:
     * "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(),
     * device_type: DeviceType::SDK, }; let default = ClientSettings::default();
     * assert_matches!(settings, default); ```
     *
     * Targets `localhost:8080` for debug builds.
     */
    class ClientSettings {
        public:
        ClientSettings() = default;
        virtual ~ClientSettings() = default;

        private:
        std::string api_url;
        DeviceType device_type;
        std::string identity_url;
        std::string user_agent;

        public:
        /**
         * The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`
         */
        const std::string & get_api_url() const { return api_url; }
        std::string & get_mutable_api_url() { return api_url; }
        void set_api_url(const std::string & value) { this->api_url = value; }

        /**
         * Device type to send to Bitwarden. Defaults to SDK
         */
        const DeviceType & get_device_type() const { return device_type; }
        DeviceType & get_mutable_device_type() { return device_type; }
        void set_device_type(const DeviceType & value) { this->device_type = value; }

        /**
         * The identity url of the targeted Bitwarden instance. Defaults to
         * `https://identity.bitwarden.com`
         */
        const std::string & get_identity_url() const { return identity_url; }
        std::string & get_mutable_identity_url() { return identity_url; }
        void set_identity_url(const std::string & value) { this->identity_url = value; }

        /**
         * The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`
         */
        const std::string & get_user_agent() const { return user_agent; }
        std::string & get_mutable_user_agent() { return user_agent; }
        void set_user_agent(const std::string & value) { this->user_agent = value; }
    };

    /**
     * Login to Bitwarden with access token
     */
    class AccessTokenLoginRequest {
        public:
        AccessTokenLoginRequest() = default;
        virtual ~AccessTokenLoginRequest() = default;

        private:
        std::string access_token;

        public:
        /**
         * Bitwarden service API access token
         */
        const std::string & get_access_token() const { return access_token; }
        std::string & get_mutable_access_token() { return access_token; }
        void set_access_token(const std::string & value) { this->access_token = value; }
    };

    /**
     * Login to Bitwarden with Api Key
     */
    class ApiKeyLoginRequest {
        public:
        ApiKeyLoginRequest() = default;
        virtual ~ApiKeyLoginRequest() = default;

        private:
        std::string client_id;
        std::string client_secret;
        std::string password;

        public:
        /**
         * Bitwarden account client_id
         */
        const std::string & get_client_id() const { return client_id; }
        std::string & get_mutable_client_id() { return client_id; }
        void set_client_id(const std::string & value) { this->client_id = value; }

        /**
         * Bitwarden account client_secret
         */
        const std::string & get_client_secret() const { return client_secret; }
        std::string & get_mutable_client_secret() { return client_secret; }
        void set_client_secret(const std::string & value) { this->client_secret = value; }

        /**
         * Bitwarden account master password
         */
        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }
    };

    class FingerprintRequest {
        public:
        FingerprintRequest() = default;
        virtual ~FingerprintRequest() = default;

        private:
        std::string fingerprint_material;
        std::string public_key;

        public:
        /**
         * The input material, used in the fingerprint generation process.
         */
        const std::string & get_fingerprint_material() const { return fingerprint_material; }
        std::string & get_mutable_fingerprint_material() { return fingerprint_material; }
        void set_fingerprint_material(const std::string & value) { this->fingerprint_material = value; }

        /**
         * The user's public key encoded with base64.
         */
        const std::string & get_public_key() const { return public_key; }
        std::string & get_mutable_public_key() { return public_key; }
        void set_public_key(const std::string & value) { this->public_key = value; }
    };

    class SecretVerificationRequest {
        public:
        SecretVerificationRequest() = default;
        virtual ~SecretVerificationRequest() = default;

        private:
        boost::optional<std::string> master_password;
        boost::optional<std::string> otp;

        public:
        /**
         * The user's master password to use for user verification. If supplied, this will be used
         * for verification purposes.
         */
        boost::optional<std::string> get_master_password() const { return master_password; }
        void set_master_password(boost::optional<std::string> value) { this->master_password = value; }

        /**
         * Alternate user verification method through OTP. This is provided for users who have no
         * master password due to use of Customer Managed Encryption. Must be present and valid if
         * master_password is absent.
         */
        boost::optional<std::string> get_otp() const { return otp; }
        void set_otp(boost::optional<std::string> value) { this->otp = value; }
    };

    /**
     * Two-factor provider
     */
    enum class TwoFactorProvider : int { AUTHENTICATOR, DUO, EMAIL, ORGANIZATION_DUO, REMEMBER, U2_F, WEB_AUTHN, YUBIKEY };

    class TwoFactorRequest {
        public:
        TwoFactorRequest() = default;
        virtual ~TwoFactorRequest() = default;

        private:
        TwoFactorProvider provider;
        bool remember;
        std::string token;

        public:
        /**
         * Two-factor provider
         */
        const TwoFactorProvider & get_provider() const { return provider; }
        TwoFactorProvider & get_mutable_provider() { return provider; }
        void set_provider(const TwoFactorProvider & value) { this->provider = value; }

        /**
         * Two-factor remember
         */
        const bool & get_remember() const { return remember; }
        bool & get_mutable_remember() { return remember; }
        void set_remember(const bool & value) { this->remember = value; }

        /**
         * Two-factor Token
         */
        const std::string & get_token() const { return token; }
        std::string & get_mutable_token() { return token; }
        void set_token(const std::string & value) { this->token = value; }
    };

    /**
     * Login to Bitwarden with Username and Password
     */
    class PasswordLoginRequest {
        public:
        PasswordLoginRequest() = default;
        virtual ~PasswordLoginRequest() = default;

        private:
        std::string email;
        std::string password;
        boost::optional<TwoFactorRequest> two_factor;

        public:
        /**
         * Bitwarden account email address
         */
        const std::string & get_email() const { return email; }
        std::string & get_mutable_email() { return email; }
        void set_email(const std::string & value) { this->email = value; }

        /**
         * Bitwarden account master password
         */
        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }

        boost::optional<TwoFactorRequest> get_two_factor() const { return two_factor; }
        void set_two_factor(boost::optional<TwoFactorRequest> value) { this->two_factor = value; }
    };

    class ProjectCreateRequest {
        public:
        ProjectCreateRequest() = default;
        virtual ~ProjectCreateRequest() = default;

        private:
        std::string name;
        std::string organization_id;

        public:
        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        /**
         * Organization where the project will be created
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }
    };

    class ProjectGetRequest {
        public:
        ProjectGetRequest() = default;
        virtual ~ProjectGetRequest() = default;

        private:
        std::string id;

        public:
        /**
         * ID of the project to retrieve
         */
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }
    };

    class ProjectsListRequest {
        public:
        ProjectsListRequest() = default;
        virtual ~ProjectsListRequest() = default;

        private:
        std::string organization_id;

        public:
        /**
         * Organization to retrieve all the projects from
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }
    };

    class ProjectsDeleteRequest {
        public:
        ProjectsDeleteRequest() = default;
        virtual ~ProjectsDeleteRequest() = default;

        private:
        std::vector<std::string> ids;

        public:
        /**
         * IDs of the projects to delete
         */
        const std::vector<std::string> & get_ids() const { return ids; }
        std::vector<std::string> & get_mutable_ids() { return ids; }
        void set_ids(const std::vector<std::string> & value) { this->ids = value; }
    };

    class ProjectPutRequest {
        public:
        ProjectPutRequest() = default;
        virtual ~ProjectPutRequest() = default;

        private:
        std::string id;
        std::string name;
        std::string organization_id;

        public:
        /**
         * ID of the project to modify
         */
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        /**
         * Organization ID of the project to modify
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }
    };

    /**
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Retrieve a project by the provided identifier
     *
     * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Creates a new project in the provided organization using the given data
     *
     * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Lists all projects of the given organization
     *
     * Returns: [ProjectsResponse](bitwarden::secrets_manager::projects::ProjectsResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Updates an existing project with the provided ID using the given data
     *
     * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Deletes all the projects whose IDs match the provided ones
     *
     * Returns:
     * [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
     */
    class ProjectsCommand {
        public:
        ProjectsCommand() = default;
        virtual ~ProjectsCommand() = default;

        private:
        boost::optional<ProjectGetRequest> get;
        boost::optional<ProjectCreateRequest> create;
        boost::optional<ProjectsListRequest> list;
        boost::optional<ProjectPutRequest> update;
        boost::optional<ProjectsDeleteRequest> projects_command_delete;

        public:
        boost::optional<ProjectGetRequest> get_get() const { return get; }
        void set_get(boost::optional<ProjectGetRequest> value) { this->get = value; }

        boost::optional<ProjectCreateRequest> get_create() const { return create; }
        void set_create(boost::optional<ProjectCreateRequest> value) { this->create = value; }

        boost::optional<ProjectsListRequest> get_list() const { return list; }
        void set_list(boost::optional<ProjectsListRequest> value) { this->list = value; }

        boost::optional<ProjectPutRequest> get_update() const { return update; }
        void set_update(boost::optional<ProjectPutRequest> value) { this->update = value; }

        boost::optional<ProjectsDeleteRequest> get_projects_command_delete() const { return projects_command_delete; }
        void set_projects_command_delete(boost::optional<ProjectsDeleteRequest> value) { this->projects_command_delete = value; }
    };

    class SecretCreateRequest {
        public:
        SecretCreateRequest() = default;
        virtual ~SecretCreateRequest() = default;

        private:
        std::string key;
        std::string note;
        std::string organization_id;
        boost::optional<std::vector<std::string>> project_ids;
        std::string value;

        public:
        const std::string & get_key() const { return key; }
        std::string & get_mutable_key() { return key; }
        void set_key(const std::string & value) { this->key = value; }

        const std::string & get_note() const { return note; }
        std::string & get_mutable_note() { return note; }
        void set_note(const std::string & value) { this->note = value; }

        /**
         * Organization where the secret will be created
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        /**
         * IDs of the projects that this secret will belong to
         */
        boost::optional<std::vector<std::string>> get_project_ids() const { return project_ids; }
        void set_project_ids(boost::optional<std::vector<std::string>> value) { this->project_ids = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    class SecretGetRequest {
        public:
        SecretGetRequest() = default;
        virtual ~SecretGetRequest() = default;

        private:
        std::string id;

        public:
        /**
         * ID of the secret to retrieve
         */
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }
    };

    class SecretsGetRequest {
        public:
        SecretsGetRequest() = default;
        virtual ~SecretsGetRequest() = default;

        private:
        std::vector<std::string> ids;

        public:
        /**
         * IDs of the secrets to retrieve
         */
        const std::vector<std::string> & get_ids() const { return ids; }
        std::vector<std::string> & get_mutable_ids() { return ids; }
        void set_ids(const std::vector<std::string> & value) { this->ids = value; }
    };

    class SecretIdentifiersRequest {
        public:
        SecretIdentifiersRequest() = default;
        virtual ~SecretIdentifiersRequest() = default;

        private:
        std::string organization_id;

        public:
        /**
         * Organization to retrieve all the secrets from
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }
    };

    class SecretsDeleteRequest {
        public:
        SecretsDeleteRequest() = default;
        virtual ~SecretsDeleteRequest() = default;

        private:
        std::vector<std::string> ids;

        public:
        /**
         * IDs of the secrets to delete
         */
        const std::vector<std::string> & get_ids() const { return ids; }
        std::vector<std::string> & get_mutable_ids() { return ids; }
        void set_ids(const std::vector<std::string> & value) { this->ids = value; }
    };

    class SecretPutRequest {
        public:
        SecretPutRequest() = default;
        virtual ~SecretPutRequest() = default;

        private:
        std::string id;
        std::string key;
        std::string note;
        std::string organization_id;
        boost::optional<std::vector<std::string>> project_ids;
        std::string value;

        public:
        /**
         * ID of the secret to modify
         */
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_key() const { return key; }
        std::string & get_mutable_key() { return key; }
        void set_key(const std::string & value) { this->key = value; }

        const std::string & get_note() const { return note; }
        std::string & get_mutable_note() { return note; }
        void set_note(const std::string & value) { this->note = value; }

        /**
         * Organization ID of the secret to modify
         */
        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        boost::optional<std::vector<std::string>> get_project_ids() const { return project_ids; }
        void set_project_ids(boost::optional<std::vector<std::string>> value) { this->project_ids = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    /**
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Retrieve a secret by the provided identifier
     *
     * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Retrieve secrets by the provided identifiers
     *
     * Returns: [SecretsResponse](bitwarden::secrets_manager::secrets::SecretsResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Creates a new secret in the provided organization using the given data
     *
     * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Lists all secret identifiers of the given organization, to then retrieve each
     * secret, use `CreateSecret`
     *
     * Returns:
     * [SecretIdentifiersResponse](bitwarden::secrets_manager::secrets::SecretIdentifiersResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Updates an existing secret with the provided ID using the given data
     *
     * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
     *
     * > Requires Authentication > Requires using an Access Token for login or calling Sync at
     * least once Deletes all the secrets whose IDs match the provided ones
     *
     * Returns:
     * [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
     */
    class SecretsCommand {
        public:
        SecretsCommand() = default;
        virtual ~SecretsCommand() = default;

        private:
        boost::optional<SecretGetRequest> get;
        boost::optional<SecretsGetRequest> get_by_ids;
        boost::optional<SecretCreateRequest> create;
        boost::optional<SecretIdentifiersRequest> list;
        boost::optional<SecretPutRequest> update;
        boost::optional<SecretsDeleteRequest> secrets_command_delete;

        public:
        boost::optional<SecretGetRequest> get_get() const { return get; }
        void set_get(boost::optional<SecretGetRequest> value) { this->get = value; }

        boost::optional<SecretsGetRequest> get_get_by_ids() const { return get_by_ids; }
        void set_get_by_ids(boost::optional<SecretsGetRequest> value) { this->get_by_ids = value; }

        boost::optional<SecretCreateRequest> get_create() const { return create; }
        void set_create(boost::optional<SecretCreateRequest> value) { this->create = value; }

        boost::optional<SecretIdentifiersRequest> get_list() const { return list; }
        void set_list(boost::optional<SecretIdentifiersRequest> value) { this->list = value; }

        boost::optional<SecretPutRequest> get_update() const { return update; }
        void set_update(boost::optional<SecretPutRequest> value) { this->update = value; }

        boost::optional<SecretsDeleteRequest> get_secrets_command_delete() const { return secrets_command_delete; }
        void set_secrets_command_delete(boost::optional<SecretsDeleteRequest> value) { this->secrets_command_delete = value; }
    };

    class SyncRequest {
        public:
        SyncRequest() = default;
        virtual ~SyncRequest() = default;

        private:
        boost::optional<bool> exclude_subdomains;

        public:
        /**
         * Exclude the subdomains from the response, defaults to false
         */
        boost::optional<bool> get_exclude_subdomains() const { return exclude_subdomains; }
        void set_exclude_subdomains(boost::optional<bool> value) { this->exclude_subdomains = value; }
    };

    /**
     * Login with username and password
     *
     * This command is for initiating an authentication handshake with Bitwarden. Authorization
     * may fail due to requiring 2fa or captcha challenge completion despite accurate
     * credentials.
     *
     * This command is not capable of handling authentication requiring 2fa or captcha.
     *
     * Returns: [PasswordLoginResponse](bitwarden::auth::login::PasswordLoginResponse)
     *
     * Login with API Key
     *
     * This command is for initiating an authentication handshake with Bitwarden.
     *
     * Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
     *
     * Login with Secrets Manager Access Token
     *
     * This command is for initiating an authentication handshake with Bitwarden.
     *
     * Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
     *
     * > Requires Authentication Get the API key of the currently authenticated user
     *
     * Returns: [UserApiKeyResponse](bitwarden::platform::UserApiKeyResponse)
     *
     * Get the user's passphrase
     *
     * Returns: String
     *
     * > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
     * part of
     *
     * Returns: [SyncResponse](bitwarden::platform::SyncResponse)
     */
    class Command {
        public:
        Command() = default;
        virtual ~Command() = default;

        private:
        boost::optional<PasswordLoginRequest> password_login;
        boost::optional<ApiKeyLoginRequest> api_key_login;
        boost::optional<AccessTokenLoginRequest> access_token_login;
        boost::optional<SecretVerificationRequest> get_user_api_key;
        boost::optional<FingerprintRequest> fingerprint;
        boost::optional<SyncRequest> sync;
        boost::optional<SecretsCommand> secrets;
        boost::optional<ProjectsCommand> projects;

        public:
        boost::optional<PasswordLoginRequest> get_password_login() const { return password_login; }
        void set_password_login(boost::optional<PasswordLoginRequest> value) { this->password_login = value; }

        boost::optional<ApiKeyLoginRequest> get_api_key_login() const { return api_key_login; }
        void set_api_key_login(boost::optional<ApiKeyLoginRequest> value) { this->api_key_login = value; }

        boost::optional<AccessTokenLoginRequest> get_access_token_login() const { return access_token_login; }
        void set_access_token_login(boost::optional<AccessTokenLoginRequest> value) { this->access_token_login = value; }

        boost::optional<SecretVerificationRequest> get_get_user_api_key() const { return get_user_api_key; }
        void set_get_user_api_key(boost::optional<SecretVerificationRequest> value) { this->get_user_api_key = value; }

        boost::optional<FingerprintRequest> get_fingerprint() const { return fingerprint; }
        void set_fingerprint(boost::optional<FingerprintRequest> value) { this->fingerprint = value; }

        boost::optional<SyncRequest> get_sync() const { return sync; }
        void set_sync(boost::optional<SyncRequest> value) { this->sync = value; }

        boost::optional<SecretsCommand> get_secrets() const { return secrets; }
        void set_secrets(boost::optional<SecretsCommand> value) { this->secrets = value; }

        boost::optional<ProjectsCommand> get_projects() const { return projects; }
        void set_projects(boost::optional<ProjectsCommand> value) { this->projects = value; }
    };

    class Attachment {
        public:
        Attachment() = default;
        virtual ~Attachment() = default;

        private:
        boost::optional<std::string> file_name;
        boost::optional<std::string> id;
        boost::optional<std::string> key;
        boost::optional<std::string> size;
        boost::optional<std::string> size_name;
        boost::optional<std::string> url;

        public:
        boost::optional<std::string> get_file_name() const { return file_name; }
        void set_file_name(boost::optional<std::string> value) { this->file_name = value; }

        boost::optional<std::string> get_id() const { return id; }
        void set_id(boost::optional<std::string> value) { this->id = value; }

        boost::optional<std::string> get_key() const { return key; }
        void set_key(boost::optional<std::string> value) { this->key = value; }

        boost::optional<std::string> get_size() const { return size; }
        void set_size(boost::optional<std::string> value) { this->size = value; }

        /**
         * Readable size, ex: "4.2 KB" or "1.43 GB"
         */
        boost::optional<std::string> get_size_name() const { return size_name; }
        void set_size_name(boost::optional<std::string> value) { this->size_name = value; }

        boost::optional<std::string> get_url() const { return url; }
        void set_url(boost::optional<std::string> value) { this->url = value; }
    };

    class Card {
        public:
        Card() = default;
        virtual ~Card() = default;

        private:
        boost::optional<std::string> brand;
        boost::optional<std::string> cardholder_name;
        boost::optional<std::string> code;
        boost::optional<std::string> exp_month;
        boost::optional<std::string> exp_year;
        boost::optional<std::string> number;

        public:
        boost::optional<std::string> get_brand() const { return brand; }
        void set_brand(boost::optional<std::string> value) { this->brand = value; }

        boost::optional<std::string> get_cardholder_name() const { return cardholder_name; }
        void set_cardholder_name(boost::optional<std::string> value) { this->cardholder_name = value; }

        boost::optional<std::string> get_code() const { return code; }
        void set_code(boost::optional<std::string> value) { this->code = value; }

        boost::optional<std::string> get_exp_month() const { return exp_month; }
        void set_exp_month(boost::optional<std::string> value) { this->exp_month = value; }

        boost::optional<std::string> get_exp_year() const { return exp_year; }
        void set_exp_year(boost::optional<std::string> value) { this->exp_year = value; }

        boost::optional<std::string> get_number() const { return number; }
        void set_number(boost::optional<std::string> value) { this->number = value; }
    };

    enum class LinkedIdType : int { ADDRESS1, ADDRESS2, ADDRESS3, BRAND, CARDHOLDER_NAME, CITY, CODE, COMPANY, COUNTRY, EMAIL, EXP_MONTH, EXP_YEAR, FIRST_NAME, FULL_NAME, LAST_NAME, LICENSE_NUMBER, MIDDLE_NAME, NUMBER, PASSPORT_NUMBER, PASSWORD, PHONE, POSTAL_CODE, SSN, STATE, TITLE, USERNAME };

    enum class FieldType : int { BOOLEAN, HIDDEN, LINKED, TEXT };

    class Field {
        public:
        Field() = default;
        virtual ~Field() = default;

        private:
        boost::optional<LinkedIdType> linked_id;
        std::string name;
        FieldType type;
        std::string value;

        public:
        boost::optional<LinkedIdType> get_linked_id() const { return linked_id; }
        void set_linked_id(boost::optional<LinkedIdType> value) { this->linked_id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const FieldType & get_type() const { return type; }
        FieldType & get_mutable_type() { return type; }
        void set_type(const FieldType & value) { this->type = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    class Identity {
        public:
        Identity() = default;
        virtual ~Identity() = default;

        private:
        boost::optional<std::string> address1;
        boost::optional<std::string> address2;
        boost::optional<std::string> address3;
        boost::optional<std::string> city;
        boost::optional<std::string> company;
        boost::optional<std::string> country;
        boost::optional<std::string> email;
        boost::optional<std::string> first_name;
        boost::optional<std::string> last_name;
        boost::optional<std::string> license_number;
        boost::optional<std::string> middle_name;
        boost::optional<std::string> passport_number;
        boost::optional<std::string> phone;
        boost::optional<std::string> postal_code;
        boost::optional<std::string> ssn;
        boost::optional<std::string> state;
        boost::optional<std::string> title;
        boost::optional<std::string> username;

        public:
        boost::optional<std::string> get_address1() const { return address1; }
        void set_address1(boost::optional<std::string> value) { this->address1 = value; }

        boost::optional<std::string> get_address2() const { return address2; }
        void set_address2(boost::optional<std::string> value) { this->address2 = value; }

        boost::optional<std::string> get_address3() const { return address3; }
        void set_address3(boost::optional<std::string> value) { this->address3 = value; }

        boost::optional<std::string> get_city() const { return city; }
        void set_city(boost::optional<std::string> value) { this->city = value; }

        boost::optional<std::string> get_company() const { return company; }
        void set_company(boost::optional<std::string> value) { this->company = value; }

        boost::optional<std::string> get_country() const { return country; }
        void set_country(boost::optional<std::string> value) { this->country = value; }

        boost::optional<std::string> get_email() const { return email; }
        void set_email(boost::optional<std::string> value) { this->email = value; }

        boost::optional<std::string> get_first_name() const { return first_name; }
        void set_first_name(boost::optional<std::string> value) { this->first_name = value; }

        boost::optional<std::string> get_last_name() const { return last_name; }
        void set_last_name(boost::optional<std::string> value) { this->last_name = value; }

        boost::optional<std::string> get_license_number() const { return license_number; }
        void set_license_number(boost::optional<std::string> value) { this->license_number = value; }

        boost::optional<std::string> get_middle_name() const { return middle_name; }
        void set_middle_name(boost::optional<std::string> value) { this->middle_name = value; }

        boost::optional<std::string> get_passport_number() const { return passport_number; }
        void set_passport_number(boost::optional<std::string> value) { this->passport_number = value; }

        boost::optional<std::string> get_phone() const { return phone; }
        void set_phone(boost::optional<std::string> value) { this->phone = value; }

        boost::optional<std::string> get_postal_code() const { return postal_code; }
        void set_postal_code(boost::optional<std::string> value) { this->postal_code = value; }

        boost::optional<std::string> get_ssn() const { return ssn; }
        void set_ssn(boost::optional<std::string> value) { this->ssn = value; }

        boost::optional<std::string> get_state() const { return state; }
        void set_state(boost::optional<std::string> value) { this->state = value; }

        boost::optional<std::string> get_title() const { return title; }
        void set_title(boost::optional<std::string> value) { this->title = value; }

        boost::optional<std::string> get_username() const { return username; }
        void set_username(boost::optional<std::string> value) { this->username = value; }
    };

    class LocalData {
        public:
        LocalData() = default;
        virtual ~LocalData() = default;

        private:
        boost::optional<int64_t> last_launched;
        boost::optional<int64_t> last_used_date;

        public:
        boost::optional<int64_t> get_last_launched() const { return last_launched; }
        void set_last_launched(boost::optional<int64_t> value) { this->last_launched = value; }

        boost::optional<int64_t> get_last_used_date() const { return last_used_date; }
        void set_last_used_date(boost::optional<int64_t> value) { this->last_used_date = value; }
    };

    enum class UriMatchType : int { URI_DOMAIN, EXACT, HOST, NEVER, REGULAR_EXPRESSION, STARTS_WITH };

    class LoginUri {
        public:
        LoginUri() = default;
        virtual ~LoginUri() = default;

        private:
        boost::optional<UriMatchType> match;
        std::string uri;

        public:
        boost::optional<UriMatchType> get_match() const { return match; }
        void set_match(boost::optional<UriMatchType> value) { this->match = value; }

        const std::string & get_uri() const { return uri; }
        std::string & get_mutable_uri() { return uri; }
        void set_uri(const std::string & value) { this->uri = value; }
    };

    class Login {
        public:
        Login() = default;
        virtual ~Login() = default;

        private:
        boost::optional<bool> autofill_on_page_load;
        std::string password;
        boost::optional<std::string> password_revision_date;
        boost::optional<std::string> totp;
        std::vector<LoginUri> uris;
        std::string username;

        public:
        boost::optional<bool> get_autofill_on_page_load() const { return autofill_on_page_load; }
        void set_autofill_on_page_load(boost::optional<bool> value) { this->autofill_on_page_load = value; }

        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }

        boost::optional<std::string> get_password_revision_date() const { return password_revision_date; }
        void set_password_revision_date(boost::optional<std::string> value) { this->password_revision_date = value; }

        boost::optional<std::string> get_totp() const { return totp; }
        void set_totp(boost::optional<std::string> value) { this->totp = value; }

        const std::vector<LoginUri> & get_uris() const { return uris; }
        std::vector<LoginUri> & get_mutable_uris() { return uris; }
        void set_uris(const std::vector<LoginUri> & value) { this->uris = value; }

        const std::string & get_username() const { return username; }
        std::string & get_mutable_username() { return username; }
        void set_username(const std::string & value) { this->username = value; }
    };

    class PasswordHistory {
        public:
        PasswordHistory() = default;
        virtual ~PasswordHistory() = default;

        private:
        std::string last_used_date;
        std::string password;

        public:
        const std::string & get_last_used_date() const { return last_used_date; }
        std::string & get_mutable_last_used_date() { return last_used_date; }
        void set_last_used_date(const std::string & value) { this->last_used_date = value; }

        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }
    };

    enum class CipherRepromptType : int { NONE, PASSWORD };

    enum class SecureNoteType : int { GENERIC };

    class SecureNote {
        public:
        SecureNote() = default;
        virtual ~SecureNote() = default;

        private:
        SecureNoteType type;

        public:
        const SecureNoteType & get_type() const { return type; }
        SecureNoteType & get_mutable_type() { return type; }
        void set_type(const SecureNoteType & value) { this->type = value; }
    };

    enum class CipherType : int { CARD, IDENTITY, LOGIN, SECURE_NOTE };

    class Cipher {
        public:
        Cipher() = default;
        virtual ~Cipher() = default;

        private:
        std::vector<Attachment> attachments;
        boost::optional<Card> card;
        std::vector<std::string> collection_ids;
        std::string creation_date;
        boost::optional<std::string> deleted_date;
        bool edit;
        bool favorite;
        std::vector<Field> fields;
        boost::optional<std::string> folder_id;
        boost::optional<std::string> id;
        boost::optional<Identity> identity;
        boost::optional<LocalData> local_data;
        boost::optional<Login> login;
        std::string name;
        std::string notes;
        boost::optional<std::string> organization_id;
        bool organization_use_totp;
        std::vector<PasswordHistory> password_history;
        CipherRepromptType reprompt;
        std::string revision_date;
        boost::optional<SecureNote> secure_note;
        CipherType type;
        bool view_password;

        public:
        const std::vector<Attachment> & get_attachments() const { return attachments; }
        std::vector<Attachment> & get_mutable_attachments() { return attachments; }
        void set_attachments(const std::vector<Attachment> & value) { this->attachments = value; }

        boost::optional<Card> get_card() const { return card; }
        void set_card(boost::optional<Card> value) { this->card = value; }

        const std::vector<std::string> & get_collection_ids() const { return collection_ids; }
        std::vector<std::string> & get_mutable_collection_ids() { return collection_ids; }
        void set_collection_ids(const std::vector<std::string> & value) { this->collection_ids = value; }

        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        boost::optional<std::string> get_deleted_date() const { return deleted_date; }
        void set_deleted_date(boost::optional<std::string> value) { this->deleted_date = value; }

        const bool & get_edit() const { return edit; }
        bool & get_mutable_edit() { return edit; }
        void set_edit(const bool & value) { this->edit = value; }

        const bool & get_favorite() const { return favorite; }
        bool & get_mutable_favorite() { return favorite; }
        void set_favorite(const bool & value) { this->favorite = value; }

        const std::vector<Field> & get_fields() const { return fields; }
        std::vector<Field> & get_mutable_fields() { return fields; }
        void set_fields(const std::vector<Field> & value) { this->fields = value; }

        boost::optional<std::string> get_folder_id() const { return folder_id; }
        void set_folder_id(boost::optional<std::string> value) { this->folder_id = value; }

        boost::optional<std::string> get_id() const { return id; }
        void set_id(boost::optional<std::string> value) { this->id = value; }

        boost::optional<Identity> get_identity() const { return identity; }
        void set_identity(boost::optional<Identity> value) { this->identity = value; }

        boost::optional<LocalData> get_local_data() const { return local_data; }
        void set_local_data(boost::optional<LocalData> value) { this->local_data = value; }

        boost::optional<Login> get_login() const { return login; }
        void set_login(boost::optional<Login> value) { this->login = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_notes() const { return notes; }
        std::string & get_mutable_notes() { return notes; }
        void set_notes(const std::string & value) { this->notes = value; }

        boost::optional<std::string> get_organization_id() const { return organization_id; }
        void set_organization_id(boost::optional<std::string> value) { this->organization_id = value; }

        const bool & get_organization_use_totp() const { return organization_use_totp; }
        bool & get_mutable_organization_use_totp() { return organization_use_totp; }
        void set_organization_use_totp(const bool & value) { this->organization_use_totp = value; }

        const std::vector<PasswordHistory> & get_password_history() const { return password_history; }
        std::vector<PasswordHistory> & get_mutable_password_history() { return password_history; }
        void set_password_history(const std::vector<PasswordHistory> & value) { this->password_history = value; }

        const CipherRepromptType & get_reprompt() const { return reprompt; }
        CipherRepromptType & get_mutable_reprompt() { return reprompt; }
        void set_reprompt(const CipherRepromptType & value) { this->reprompt = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }

        boost::optional<SecureNote> get_secure_note() const { return secure_note; }
        void set_secure_note(boost::optional<SecureNote> value) { this->secure_note = value; }

        const CipherType & get_type() const { return type; }
        CipherType & get_mutable_type() { return type; }
        void set_type(const CipherType & value) { this->type = value; }

        const bool & get_view_password() const { return view_password; }
        bool & get_mutable_view_password() { return view_password; }
        void set_view_password(const bool & value) { this->view_password = value; }
    };

    class AttachmentView {
        public:
        AttachmentView() = default;
        virtual ~AttachmentView() = default;

        private:
        boost::optional<std::string> file_name;
        boost::optional<std::string> id;
        boost::optional<std::string> key;
        boost::optional<std::string> size;
        boost::optional<std::string> size_name;
        boost::optional<std::string> url;

        public:
        boost::optional<std::string> get_file_name() const { return file_name; }
        void set_file_name(boost::optional<std::string> value) { this->file_name = value; }

        boost::optional<std::string> get_id() const { return id; }
        void set_id(boost::optional<std::string> value) { this->id = value; }

        boost::optional<std::string> get_key() const { return key; }
        void set_key(boost::optional<std::string> value) { this->key = value; }

        boost::optional<std::string> get_size() const { return size; }
        void set_size(boost::optional<std::string> value) { this->size = value; }

        boost::optional<std::string> get_size_name() const { return size_name; }
        void set_size_name(boost::optional<std::string> value) { this->size_name = value; }

        boost::optional<std::string> get_url() const { return url; }
        void set_url(boost::optional<std::string> value) { this->url = value; }
    };

    class CardView {
        public:
        CardView() = default;
        virtual ~CardView() = default;

        private:
        boost::optional<std::string> brand;
        boost::optional<std::string> cardholder_name;
        boost::optional<std::string> code;
        boost::optional<std::string> exp_month;
        boost::optional<std::string> exp_year;
        boost::optional<std::string> number;

        public:
        boost::optional<std::string> get_brand() const { return brand; }
        void set_brand(boost::optional<std::string> value) { this->brand = value; }

        boost::optional<std::string> get_cardholder_name() const { return cardholder_name; }
        void set_cardholder_name(boost::optional<std::string> value) { this->cardholder_name = value; }

        boost::optional<std::string> get_code() const { return code; }
        void set_code(boost::optional<std::string> value) { this->code = value; }

        boost::optional<std::string> get_exp_month() const { return exp_month; }
        void set_exp_month(boost::optional<std::string> value) { this->exp_month = value; }

        boost::optional<std::string> get_exp_year() const { return exp_year; }
        void set_exp_year(boost::optional<std::string> value) { this->exp_year = value; }

        boost::optional<std::string> get_number() const { return number; }
        void set_number(boost::optional<std::string> value) { this->number = value; }
    };

    class FieldView {
        public:
        FieldView() = default;
        virtual ~FieldView() = default;

        private:
        boost::optional<LinkedIdType> linked_id;
        std::string name;
        FieldType type;
        std::string value;

        public:
        boost::optional<LinkedIdType> get_linked_id() const { return linked_id; }
        void set_linked_id(boost::optional<LinkedIdType> value) { this->linked_id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const FieldType & get_type() const { return type; }
        FieldType & get_mutable_type() { return type; }
        void set_type(const FieldType & value) { this->type = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    class IdentityView {
        public:
        IdentityView() = default;
        virtual ~IdentityView() = default;

        private:
        boost::optional<std::string> address1;
        boost::optional<std::string> address2;
        boost::optional<std::string> address3;
        boost::optional<std::string> city;
        boost::optional<std::string> company;
        boost::optional<std::string> country;
        boost::optional<std::string> email;
        boost::optional<std::string> first_name;
        boost::optional<std::string> last_name;
        boost::optional<std::string> license_number;
        boost::optional<std::string> middle_name;
        boost::optional<std::string> passport_number;
        boost::optional<std::string> phone;
        boost::optional<std::string> postal_code;
        boost::optional<std::string> ssn;
        boost::optional<std::string> state;
        boost::optional<std::string> title;
        boost::optional<std::string> username;

        public:
        boost::optional<std::string> get_address1() const { return address1; }
        void set_address1(boost::optional<std::string> value) { this->address1 = value; }

        boost::optional<std::string> get_address2() const { return address2; }
        void set_address2(boost::optional<std::string> value) { this->address2 = value; }

        boost::optional<std::string> get_address3() const { return address3; }
        void set_address3(boost::optional<std::string> value) { this->address3 = value; }

        boost::optional<std::string> get_city() const { return city; }
        void set_city(boost::optional<std::string> value) { this->city = value; }

        boost::optional<std::string> get_company() const { return company; }
        void set_company(boost::optional<std::string> value) { this->company = value; }

        boost::optional<std::string> get_country() const { return country; }
        void set_country(boost::optional<std::string> value) { this->country = value; }

        boost::optional<std::string> get_email() const { return email; }
        void set_email(boost::optional<std::string> value) { this->email = value; }

        boost::optional<std::string> get_first_name() const { return first_name; }
        void set_first_name(boost::optional<std::string> value) { this->first_name = value; }

        boost::optional<std::string> get_last_name() const { return last_name; }
        void set_last_name(boost::optional<std::string> value) { this->last_name = value; }

        boost::optional<std::string> get_license_number() const { return license_number; }
        void set_license_number(boost::optional<std::string> value) { this->license_number = value; }

        boost::optional<std::string> get_middle_name() const { return middle_name; }
        void set_middle_name(boost::optional<std::string> value) { this->middle_name = value; }

        boost::optional<std::string> get_passport_number() const { return passport_number; }
        void set_passport_number(boost::optional<std::string> value) { this->passport_number = value; }

        boost::optional<std::string> get_phone() const { return phone; }
        void set_phone(boost::optional<std::string> value) { this->phone = value; }

        boost::optional<std::string> get_postal_code() const { return postal_code; }
        void set_postal_code(boost::optional<std::string> value) { this->postal_code = value; }

        boost::optional<std::string> get_ssn() const { return ssn; }
        void set_ssn(boost::optional<std::string> value) { this->ssn = value; }

        boost::optional<std::string> get_state() const { return state; }
        void set_state(boost::optional<std::string> value) { this->state = value; }

        boost::optional<std::string> get_title() const { return title; }
        void set_title(boost::optional<std::string> value) { this->title = value; }

        boost::optional<std::string> get_username() const { return username; }
        void set_username(boost::optional<std::string> value) { this->username = value; }
    };

    class LocalDataView {
        public:
        LocalDataView() = default;
        virtual ~LocalDataView() = default;

        private:
        boost::optional<int64_t> last_launched;
        boost::optional<int64_t> last_used_date;

        public:
        boost::optional<int64_t> get_last_launched() const { return last_launched; }
        void set_last_launched(boost::optional<int64_t> value) { this->last_launched = value; }

        boost::optional<int64_t> get_last_used_date() const { return last_used_date; }
        void set_last_used_date(boost::optional<int64_t> value) { this->last_used_date = value; }
    };

    class LoginUriView {
        public:
        LoginUriView() = default;
        virtual ~LoginUriView() = default;

        private:
        boost::optional<UriMatchType> match;
        std::string uri;

        public:
        boost::optional<UriMatchType> get_match() const { return match; }
        void set_match(boost::optional<UriMatchType> value) { this->match = value; }

        const std::string & get_uri() const { return uri; }
        std::string & get_mutable_uri() { return uri; }
        void set_uri(const std::string & value) { this->uri = value; }
    };

    class LoginView {
        public:
        LoginView() = default;
        virtual ~LoginView() = default;

        private:
        boost::optional<bool> autofill_on_page_load;
        std::string password;
        boost::optional<std::string> password_revision_date;
        boost::optional<std::string> totp;
        std::vector<LoginUriView> uris;
        std::string username;

        public:
        boost::optional<bool> get_autofill_on_page_load() const { return autofill_on_page_load; }
        void set_autofill_on_page_load(boost::optional<bool> value) { this->autofill_on_page_load = value; }

        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }

        boost::optional<std::string> get_password_revision_date() const { return password_revision_date; }
        void set_password_revision_date(boost::optional<std::string> value) { this->password_revision_date = value; }

        boost::optional<std::string> get_totp() const { return totp; }
        void set_totp(boost::optional<std::string> value) { this->totp = value; }

        const std::vector<LoginUriView> & get_uris() const { return uris; }
        std::vector<LoginUriView> & get_mutable_uris() { return uris; }
        void set_uris(const std::vector<LoginUriView> & value) { this->uris = value; }

        const std::string & get_username() const { return username; }
        std::string & get_mutable_username() { return username; }
        void set_username(const std::string & value) { this->username = value; }
    };

    class PasswordHistoryView {
        public:
        PasswordHistoryView() = default;
        virtual ~PasswordHistoryView() = default;

        private:
        std::string last_used_date;
        std::string password;

        public:
        const std::string & get_last_used_date() const { return last_used_date; }
        std::string & get_mutable_last_used_date() { return last_used_date; }
        void set_last_used_date(const std::string & value) { this->last_used_date = value; }

        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }
    };

    class SecureNoteView {
        public:
        SecureNoteView() = default;
        virtual ~SecureNoteView() = default;

        private:
        SecureNoteType type;

        public:
        const SecureNoteType & get_type() const { return type; }
        SecureNoteType & get_mutable_type() { return type; }
        void set_type(const SecureNoteType & value) { this->type = value; }
    };

    class CipherView {
        public:
        CipherView() = default;
        virtual ~CipherView() = default;

        private:
        std::vector<AttachmentView> attachments;
        boost::optional<CardView> card;
        std::vector<std::string> collection_ids;
        std::string creation_date;
        boost::optional<std::string> deleted_date;
        bool edit;
        bool favorite;
        std::vector<FieldView> fields;
        boost::optional<std::string> folder_id;
        boost::optional<std::string> id;
        boost::optional<IdentityView> identity;
        boost::optional<LocalDataView> local_data;
        boost::optional<LoginView> login;
        std::string name;
        std::string notes;
        boost::optional<std::string> organization_id;
        bool organization_use_totp;
        std::vector<PasswordHistoryView> password_history;
        CipherRepromptType reprompt;
        std::string revision_date;
        boost::optional<SecureNoteView> secure_note;
        CipherType type;
        bool view_password;

        public:
        const std::vector<AttachmentView> & get_attachments() const { return attachments; }
        std::vector<AttachmentView> & get_mutable_attachments() { return attachments; }
        void set_attachments(const std::vector<AttachmentView> & value) { this->attachments = value; }

        boost::optional<CardView> get_card() const { return card; }
        void set_card(boost::optional<CardView> value) { this->card = value; }

        const std::vector<std::string> & get_collection_ids() const { return collection_ids; }
        std::vector<std::string> & get_mutable_collection_ids() { return collection_ids; }
        void set_collection_ids(const std::vector<std::string> & value) { this->collection_ids = value; }

        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        boost::optional<std::string> get_deleted_date() const { return deleted_date; }
        void set_deleted_date(boost::optional<std::string> value) { this->deleted_date = value; }

        const bool & get_edit() const { return edit; }
        bool & get_mutable_edit() { return edit; }
        void set_edit(const bool & value) { this->edit = value; }

        const bool & get_favorite() const { return favorite; }
        bool & get_mutable_favorite() { return favorite; }
        void set_favorite(const bool & value) { this->favorite = value; }

        const std::vector<FieldView> & get_fields() const { return fields; }
        std::vector<FieldView> & get_mutable_fields() { return fields; }
        void set_fields(const std::vector<FieldView> & value) { this->fields = value; }

        boost::optional<std::string> get_folder_id() const { return folder_id; }
        void set_folder_id(boost::optional<std::string> value) { this->folder_id = value; }

        boost::optional<std::string> get_id() const { return id; }
        void set_id(boost::optional<std::string> value) { this->id = value; }

        boost::optional<IdentityView> get_identity() const { return identity; }
        void set_identity(boost::optional<IdentityView> value) { this->identity = value; }

        boost::optional<LocalDataView> get_local_data() const { return local_data; }
        void set_local_data(boost::optional<LocalDataView> value) { this->local_data = value; }

        boost::optional<LoginView> get_login() const { return login; }
        void set_login(boost::optional<LoginView> value) { this->login = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_notes() const { return notes; }
        std::string & get_mutable_notes() { return notes; }
        void set_notes(const std::string & value) { this->notes = value; }

        boost::optional<std::string> get_organization_id() const { return organization_id; }
        void set_organization_id(boost::optional<std::string> value) { this->organization_id = value; }

        const bool & get_organization_use_totp() const { return organization_use_totp; }
        bool & get_mutable_organization_use_totp() { return organization_use_totp; }
        void set_organization_use_totp(const bool & value) { this->organization_use_totp = value; }

        const std::vector<PasswordHistoryView> & get_password_history() const { return password_history; }
        std::vector<PasswordHistoryView> & get_mutable_password_history() { return password_history; }
        void set_password_history(const std::vector<PasswordHistoryView> & value) { this->password_history = value; }

        const CipherRepromptType & get_reprompt() const { return reprompt; }
        CipherRepromptType & get_mutable_reprompt() { return reprompt; }
        void set_reprompt(const CipherRepromptType & value) { this->reprompt = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }

        boost::optional<SecureNoteView> get_secure_note() const { return secure_note; }
        void set_secure_note(boost::optional<SecureNoteView> value) { this->secure_note = value; }

        const CipherType & get_type() const { return type; }
        CipherType & get_mutable_type() { return type; }
        void set_type(const CipherType & value) { this->type = value; }

        const bool & get_view_password() const { return view_password; }
        bool & get_mutable_view_password() { return view_password; }
        void set_view_password(const bool & value) { this->view_password = value; }
    };

    class Collection {
        public:
        Collection() = default;
        virtual ~Collection() = default;

        private:
        boost::optional<std::string> external_id;
        bool hide_passwords;
        std::string id;
        std::string name;
        std::string organization_id;
        bool read_only;

        public:
        boost::optional<std::string> get_external_id() const { return external_id; }
        void set_external_id(boost::optional<std::string> value) { this->external_id = value; }

        const bool & get_hide_passwords() const { return hide_passwords; }
        bool & get_mutable_hide_passwords() { return hide_passwords; }
        void set_hide_passwords(const bool & value) { this->hide_passwords = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        const bool & get_read_only() const { return read_only; }
        bool & get_mutable_read_only() { return read_only; }
        void set_read_only(const bool & value) { this->read_only = value; }
    };

    class EncryptedJson {
        public:
        EncryptedJson() = default;
        virtual ~EncryptedJson() = default;

        private:
        std::string password;

        public:
        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }
    };

    class ExportFormatClass {
        public:
        ExportFormatClass() = default;
        virtual ~ExportFormatClass() = default;

        private:
        EncryptedJson encrypted_json;

        public:
        const EncryptedJson & get_encrypted_json() const { return encrypted_json; }
        EncryptedJson & get_mutable_encrypted_json() { return encrypted_json; }
        void set_encrypted_json(const EncryptedJson & value) { this->encrypted_json = value; }
    };

    enum class ExportFormatEnum : int { ACCOUNT_ENCRYPTED_JSON, CSV, JSON };

    using ExportFormat = boost::variant<ExportFormatClass, ExportFormatEnum>;

    class Folder {
        public:
        Folder() = default;
        virtual ~Folder() = default;

        private:
        std::string id;
        std::string name;
        std::string revision_date;

        public:
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }
    };

    class FolderView {
        public:
        FolderView() = default;
        virtual ~FolderView() = default;

        private:
        std::string id;
        std::string name;
        std::string revision_date;

        public:
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }
    };

    class Argon2Id {
        public:
        Argon2Id() :
            iterations_constraint(1, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none),
            memory_constraint(1, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none),
            parallelism_constraint(1, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none)
        {}
        virtual ~Argon2Id() = default;

        private:
        int64_t iterations;
        ClassMemberConstraints iterations_constraint;
        int64_t memory;
        ClassMemberConstraints memory_constraint;
        int64_t parallelism;
        ClassMemberConstraints parallelism_constraint;

        public:
        const int64_t & get_iterations() const { return iterations; }
        int64_t & get_mutable_iterations() { return iterations; }
        void set_iterations(const int64_t & value) { CheckConstraint("iterations", iterations_constraint, value); this->iterations = value; }

        const int64_t & get_memory() const { return memory; }
        int64_t & get_mutable_memory() { return memory; }
        void set_memory(const int64_t & value) { CheckConstraint("memory", memory_constraint, value); this->memory = value; }

        const int64_t & get_parallelism() const { return parallelism; }
        int64_t & get_mutable_parallelism() { return parallelism; }
        void set_parallelism(const int64_t & value) { CheckConstraint("parallelism", parallelism_constraint, value); this->parallelism = value; }
    };

    class PBkdf2 {
        public:
        PBkdf2() :
            iterations_constraint(1, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none)
        {}
        virtual ~PBkdf2() = default;

        private:
        int64_t iterations;
        ClassMemberConstraints iterations_constraint;

        public:
        const int64_t & get_iterations() const { return iterations; }
        int64_t & get_mutable_iterations() { return iterations; }
        void set_iterations(const int64_t & value) { CheckConstraint("iterations", iterations_constraint, value); this->iterations = value; }
    };

    /**
     * The user's KDF parameters, as received from the prelogin request
     */
    class Kdf {
        public:
        Kdf() = default;
        virtual ~Kdf() = default;

        private:
        boost::optional<PBkdf2> p_bkdf2;
        boost::optional<Argon2Id> argon2_id;

        public:
        boost::optional<PBkdf2> get_p_bkdf2() const { return p_bkdf2; }
        void set_p_bkdf2(boost::optional<PBkdf2> value) { this->p_bkdf2 = value; }

        boost::optional<Argon2Id> get_argon2_id() const { return argon2_id; }
        void set_argon2_id(boost::optional<Argon2Id> value) { this->argon2_id = value; }
    };

    class InitCryptoRequest {
        public:
        InitCryptoRequest() = default;
        virtual ~InitCryptoRequest() = default;

        private:
        std::string email;
        Kdf kdf_params;
        std::map<std::string, std::string> organization_keys;
        std::string password;
        std::string private_key;
        std::string user_key;

        public:
        /**
         * The user's email address
         */
        const std::string & get_email() const { return email; }
        std::string & get_mutable_email() { return email; }
        void set_email(const std::string & value) { this->email = value; }

        /**
         * The user's KDF parameters, as received from the prelogin request
         */
        const Kdf & get_kdf_params() const { return kdf_params; }
        Kdf & get_mutable_kdf_params() { return kdf_params; }
        void set_kdf_params(const Kdf & value) { this->kdf_params = value; }

        /**
         * The encryption keys for all the organizations the user is a part of
         */
        const std::map<std::string, std::string> & get_organization_keys() const { return organization_keys; }
        std::map<std::string, std::string> & get_mutable_organization_keys() { return organization_keys; }
        void set_organization_keys(const std::map<std::string, std::string> & value) { this->organization_keys = value; }

        /**
         * The user's master password
         */
        const std::string & get_password() const { return password; }
        std::string & get_mutable_password() { return password; }
        void set_password(const std::string & value) { this->password = value; }

        /**
         * The user's encryptred private key
         */
        const std::string & get_private_key() const { return private_key; }
        std::string & get_mutable_private_key() { return private_key; }
        void set_private_key(const std::string & value) { this->private_key = value; }

        /**
         * The user's encrypted symmetric crypto key
         */
        const std::string & get_user_key() const { return user_key; }
        std::string & get_mutable_user_key() { return user_key; }
        void set_user_key(const std::string & value) { this->user_key = value; }
    };

    class MasterPasswordPolicyOptions {
        public:
        MasterPasswordPolicyOptions() :
            min_complexity_constraint(boost::none, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none),
            min_length_constraint(boost::none, boost::none, boost::none, boost::none, boost::none, boost::none, boost::none)
        {}
        virtual ~MasterPasswordPolicyOptions() = default;

        private:
        bool enforce_on_login;
        int64_t min_complexity;
        ClassMemberConstraints min_complexity_constraint;
        int64_t min_length;
        ClassMemberConstraints min_length_constraint;
        bool require_lower;
        bool require_numbers;
        bool require_special;
        bool require_upper;

        public:
        /**
         * Flag to indicate if the policy should be enforced on login. If true, and the user's
         * password does not meet the policy requirements, the user will be forced to update their
         * password.
         */
        const bool & get_enforce_on_login() const { return enforce_on_login; }
        bool & get_mutable_enforce_on_login() { return enforce_on_login; }
        void set_enforce_on_login(const bool & value) { this->enforce_on_login = value; }

        const int64_t & get_min_complexity() const { return min_complexity; }
        int64_t & get_mutable_min_complexity() { return min_complexity; }
        void set_min_complexity(const int64_t & value) { CheckConstraint("min_complexity", min_complexity_constraint, value); this->min_complexity = value; }

        const int64_t & get_min_length() const { return min_length; }
        int64_t & get_mutable_min_length() { return min_length; }
        void set_min_length(const int64_t & value) { CheckConstraint("min_length", min_length_constraint, value); this->min_length = value; }

        const bool & get_require_lower() const { return require_lower; }
        bool & get_mutable_require_lower() { return require_lower; }
        void set_require_lower(const bool & value) { this->require_lower = value; }

        const bool & get_require_numbers() const { return require_numbers; }
        bool & get_mutable_require_numbers() { return require_numbers; }
        void set_require_numbers(const bool & value) { this->require_numbers = value; }

        const bool & get_require_special() const { return require_special; }
        bool & get_mutable_require_special() { return require_special; }
        void set_require_special(const bool & value) { this->require_special = value; }

        const bool & get_require_upper() const { return require_upper; }
        bool & get_mutable_require_upper() { return require_upper; }
        void set_require_upper(const bool & value) { this->require_upper = value; }
    };

    /**
     * Passphrase generator request.
     *
     * The default separator is `-` and default number of words is 3.
     */
    class PassphraseGeneratorRequest {
        public:
        PassphraseGeneratorRequest() = default;
        virtual ~PassphraseGeneratorRequest() = default;

        private:
        boost::optional<bool> capitalize;
        boost::optional<bool> include_number;
        boost::optional<int64_t> num_words;
        boost::optional<std::string> word_separator;

        public:
        boost::optional<bool> get_capitalize() const { return capitalize; }
        void set_capitalize(boost::optional<bool> value) { this->capitalize = value; }

        boost::optional<bool> get_include_number() const { return include_number; }
        void set_include_number(boost::optional<bool> value) { this->include_number = value; }

        boost::optional<int64_t> get_num_words() const { return num_words; }
        void set_num_words(boost::optional<int64_t> value) { this->num_words = value; }

        boost::optional<std::string> get_word_separator() const { return word_separator; }
        void set_word_separator(boost::optional<std::string> value) { this->word_separator = value; }
    };

    /**
     * Password generator request. If all options are false, the default is to generate a
     * password with: - lowercase - uppercase - numbers
     *
     * The default length is 16.
     */
    class PasswordGeneratorRequest {
        public:
        PasswordGeneratorRequest() = default;
        virtual ~PasswordGeneratorRequest() = default;

        private:
        boost::optional<bool> avoid_ambiguous;
        boost::optional<int64_t> length;
        bool lowercase;
        boost::optional<bool> min_lowercase;
        boost::optional<bool> min_number;
        boost::optional<bool> min_special;
        boost::optional<bool> min_uppercase;
        bool numbers;
        bool special;
        bool uppercase;

        public:
        boost::optional<bool> get_avoid_ambiguous() const { return avoid_ambiguous; }
        void set_avoid_ambiguous(boost::optional<bool> value) { this->avoid_ambiguous = value; }

        boost::optional<int64_t> get_length() const { return length; }
        void set_length(boost::optional<int64_t> value) { this->length = value; }

        const bool & get_lowercase() const { return lowercase; }
        bool & get_mutable_lowercase() { return lowercase; }
        void set_lowercase(const bool & value) { this->lowercase = value; }

        boost::optional<bool> get_min_lowercase() const { return min_lowercase; }
        void set_min_lowercase(boost::optional<bool> value) { this->min_lowercase = value; }

        boost::optional<bool> get_min_number() const { return min_number; }
        void set_min_number(boost::optional<bool> value) { this->min_number = value; }

        boost::optional<bool> get_min_special() const { return min_special; }
        void set_min_special(boost::optional<bool> value) { this->min_special = value; }

        boost::optional<bool> get_min_uppercase() const { return min_uppercase; }
        void set_min_uppercase(boost::optional<bool> value) { this->min_uppercase = value; }

        const bool & get_numbers() const { return numbers; }
        bool & get_mutable_numbers() { return numbers; }
        void set_numbers(const bool & value) { this->numbers = value; }

        const bool & get_special() const { return special; }
        bool & get_mutable_special() { return special; }
        void set_special(const bool & value) { this->special = value; }

        const bool & get_uppercase() const { return uppercase; }
        bool & get_mutable_uppercase() { return uppercase; }
        void set_uppercase(const bool & value) { this->uppercase = value; }
    };

    class DocRef {
        public:
        DocRef() = default;
        virtual ~DocRef() = default;

        private:
        boost::optional<Cipher> cipher;
        boost::optional<CipherView> cipher_view;
        boost::optional<Collection> collection;
        boost::optional<Folder> folder;
        boost::optional<FolderView> folder_view;
        boost::optional<InitCryptoRequest> init_crypto_request;
        boost::optional<PasswordGeneratorRequest> password_generator_request;
        boost::optional<PassphraseGeneratorRequest> passphrase_generator_request;
        boost::optional<ExportFormat> export_format;
        boost::optional<MasterPasswordPolicyOptions> master_password_policy_options;
        boost::optional<Kdf> kdf;

        public:
        boost::optional<Cipher> get_cipher() const { return cipher; }
        void set_cipher(boost::optional<Cipher> value) { this->cipher = value; }

        boost::optional<CipherView> get_cipher_view() const { return cipher_view; }
        void set_cipher_view(boost::optional<CipherView> value) { this->cipher_view = value; }

        boost::optional<Collection> get_collection() const { return collection; }
        void set_collection(boost::optional<Collection> value) { this->collection = value; }

        boost::optional<Folder> get_folder() const { return folder; }
        void set_folder(boost::optional<Folder> value) { this->folder = value; }

        boost::optional<FolderView> get_folder_view() const { return folder_view; }
        void set_folder_view(boost::optional<FolderView> value) { this->folder_view = value; }

        boost::optional<InitCryptoRequest> get_init_crypto_request() const { return init_crypto_request; }
        void set_init_crypto_request(boost::optional<InitCryptoRequest> value) { this->init_crypto_request = value; }

        boost::optional<PasswordGeneratorRequest> get_password_generator_request() const { return password_generator_request; }
        void set_password_generator_request(boost::optional<PasswordGeneratorRequest> value) { this->password_generator_request = value; }

        boost::optional<PassphraseGeneratorRequest> get_passphrase_generator_request() const { return passphrase_generator_request; }
        void set_passphrase_generator_request(boost::optional<PassphraseGeneratorRequest> value) { this->passphrase_generator_request = value; }

        boost::optional<ExportFormat> get_export_format() const { return export_format; }
        void set_export_format(boost::optional<ExportFormat> value) { this->export_format = value; }

        boost::optional<MasterPasswordPolicyOptions> get_master_password_policy_options() const { return master_password_policy_options; }
        void set_master_password_policy_options(boost::optional<MasterPasswordPolicyOptions> value) { this->master_password_policy_options = value; }

        boost::optional<Kdf> get_kdf() const { return kdf; }
        void set_kdf(boost::optional<Kdf> value) { this->kdf = value; }
    };

    class PurpleAuthenticator {
        public:
        PurpleAuthenticator() = default;
        virtual ~PurpleAuthenticator() = default;

        private:

        public:
    };

    class PurpleDuo {
        public:
        PurpleDuo() = default;
        virtual ~PurpleDuo() = default;

        private:
        std::string host;
        std::string signature;

        public:
        const std::string & get_host() const { return host; }
        std::string & get_mutable_host() { return host; }
        void set_host(const std::string & value) { this->host = value; }

        const std::string & get_signature() const { return signature; }
        std::string & get_mutable_signature() { return signature; }
        void set_signature(const std::string & value) { this->signature = value; }
    };

    class PurpleEmail {
        public:
        PurpleEmail() = default;
        virtual ~PurpleEmail() = default;

        private:
        std::string email;

        public:
        /**
         * The email to request a 2fa TOTP for
         */
        const std::string & get_email() const { return email; }
        std::string & get_mutable_email() { return email; }
        void set_email(const std::string & value) { this->email = value; }
    };

    class PurpleRemember {
        public:
        PurpleRemember() = default;
        virtual ~PurpleRemember() = default;

        private:

        public:
    };

    class PurpleWebAuthn {
        public:
        PurpleWebAuthn() = default;
        virtual ~PurpleWebAuthn() = default;

        private:

        public:
    };

    class PurpleYubiKey {
        public:
        PurpleYubiKey() = default;
        virtual ~PurpleYubiKey() = default;

        private:
        bool nfc;

        public:
        /**
         * Whether the stored yubikey supports near field communication
         */
        const bool & get_nfc() const { return nfc; }
        bool & get_mutable_nfc() { return nfc; }
        void set_nfc(const bool & value) { this->nfc = value; }
    };

    class ApiKeyLoginResponseTwoFactorProviders {
        public:
        ApiKeyLoginResponseTwoFactorProviders() = default;
        virtual ~ApiKeyLoginResponseTwoFactorProviders() = default;

        private:
        boost::optional<PurpleAuthenticator> authenticator;
        boost::optional<PurpleDuo> duo;
        boost::optional<PurpleEmail> email;
        boost::optional<PurpleDuo> organization_duo;
        boost::optional<PurpleRemember> remember;
        boost::optional<PurpleWebAuthn> web_authn;
        boost::optional<PurpleYubiKey> yubi_key;

        public:
        boost::optional<PurpleAuthenticator> get_authenticator() const { return authenticator; }
        void set_authenticator(boost::optional<PurpleAuthenticator> value) { this->authenticator = value; }

        /**
         * Duo-backed 2fa
         */
        boost::optional<PurpleDuo> get_duo() const { return duo; }
        void set_duo(boost::optional<PurpleDuo> value) { this->duo = value; }

        /**
         * Email 2fa
         */
        boost::optional<PurpleEmail> get_email() const { return email; }
        void set_email(boost::optional<PurpleEmail> value) { this->email = value; }

        /**
         * Duo-backed 2fa operated by an organization the user is a member of
         */
        boost::optional<PurpleDuo> get_organization_duo() const { return organization_duo; }
        void set_organization_duo(boost::optional<PurpleDuo> value) { this->organization_duo = value; }

        /**
         * Presence indicates the user has stored this device as bypassing 2fa
         */
        boost::optional<PurpleRemember> get_remember() const { return remember; }
        void set_remember(boost::optional<PurpleRemember> value) { this->remember = value; }

        /**
         * WebAuthn-backed 2fa
         */
        boost::optional<PurpleWebAuthn> get_web_authn() const { return web_authn; }
        void set_web_authn(boost::optional<PurpleWebAuthn> value) { this->web_authn = value; }

        /**
         * Yubikey-backed 2fa
         */
        boost::optional<PurpleYubiKey> get_yubi_key() const { return yubi_key; }
        void set_yubi_key(boost::optional<PurpleYubiKey> value) { this->yubi_key = value; }
    };

    class ApiKeyLoginResponse {
        public:
        ApiKeyLoginResponse() = default;
        virtual ~ApiKeyLoginResponse() = default;

        private:
        bool authenticated;
        bool force_password_reset;
        bool reset_master_password;
        boost::optional<ApiKeyLoginResponseTwoFactorProviders> two_factor;

        public:
        const bool & get_authenticated() const { return authenticated; }
        bool & get_mutable_authenticated() { return authenticated; }
        void set_authenticated(const bool & value) { this->authenticated = value; }

        /**
         * Whether or not the user is required to update their master password
         */
        const bool & get_force_password_reset() const { return force_password_reset; }
        bool & get_mutable_force_password_reset() { return force_password_reset; }
        void set_force_password_reset(const bool & value) { this->force_password_reset = value; }

        /**
         * TODO: What does this do?
         */
        const bool & get_reset_master_password() const { return reset_master_password; }
        bool & get_mutable_reset_master_password() { return reset_master_password; }
        void set_reset_master_password(const bool & value) { this->reset_master_password = value; }

        boost::optional<ApiKeyLoginResponseTwoFactorProviders> get_two_factor() const { return two_factor; }
        void set_two_factor(boost::optional<ApiKeyLoginResponseTwoFactorProviders> value) { this->two_factor = value; }
    };

    class ResponseForApiKeyLoginResponse {
        public:
        ResponseForApiKeyLoginResponse() = default;
        virtual ~ResponseForApiKeyLoginResponse() = default;

        private:
        boost::optional<ApiKeyLoginResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<ApiKeyLoginResponse> get_data() const { return data; }
        void set_data(boost::optional<ApiKeyLoginResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class FingerprintResponse {
        public:
        FingerprintResponse() = default;
        virtual ~FingerprintResponse() = default;

        private:
        std::string fingerprint;

        public:
        const std::string & get_fingerprint() const { return fingerprint; }
        std::string & get_mutable_fingerprint() { return fingerprint; }
        void set_fingerprint(const std::string & value) { this->fingerprint = value; }
    };

    class ResponseForFingerprintResponse {
        public:
        ResponseForFingerprintResponse() = default;
        virtual ~ResponseForFingerprintResponse() = default;

        private:
        boost::optional<FingerprintResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<FingerprintResponse> get_data() const { return data; }
        void set_data(boost::optional<FingerprintResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class CaptchaResponse {
        public:
        CaptchaResponse() = default;
        virtual ~CaptchaResponse() = default;

        private:
        std::string site_key;

        public:
        /**
         * hcaptcha site key
         */
        const std::string & get_site_key() const { return site_key; }
        std::string & get_mutable_site_key() { return site_key; }
        void set_site_key(const std::string & value) { this->site_key = value; }
    };

    class FluffyAuthenticator {
        public:
        FluffyAuthenticator() = default;
        virtual ~FluffyAuthenticator() = default;

        private:

        public:
    };

    class FluffyDuo {
        public:
        FluffyDuo() = default;
        virtual ~FluffyDuo() = default;

        private:
        std::string host;
        std::string signature;

        public:
        const std::string & get_host() const { return host; }
        std::string & get_mutable_host() { return host; }
        void set_host(const std::string & value) { this->host = value; }

        const std::string & get_signature() const { return signature; }
        std::string & get_mutable_signature() { return signature; }
        void set_signature(const std::string & value) { this->signature = value; }
    };

    class FluffyEmail {
        public:
        FluffyEmail() = default;
        virtual ~FluffyEmail() = default;

        private:
        std::string email;

        public:
        /**
         * The email to request a 2fa TOTP for
         */
        const std::string & get_email() const { return email; }
        std::string & get_mutable_email() { return email; }
        void set_email(const std::string & value) { this->email = value; }
    };

    class FluffyRemember {
        public:
        FluffyRemember() = default;
        virtual ~FluffyRemember() = default;

        private:

        public:
    };

    class FluffyWebAuthn {
        public:
        FluffyWebAuthn() = default;
        virtual ~FluffyWebAuthn() = default;

        private:

        public:
    };

    class FluffyYubiKey {
        public:
        FluffyYubiKey() = default;
        virtual ~FluffyYubiKey() = default;

        private:
        bool nfc;

        public:
        /**
         * Whether the stored yubikey supports near field communication
         */
        const bool & get_nfc() const { return nfc; }
        bool & get_mutable_nfc() { return nfc; }
        void set_nfc(const bool & value) { this->nfc = value; }
    };

    class PasswordLoginResponseTwoFactorProviders {
        public:
        PasswordLoginResponseTwoFactorProviders() = default;
        virtual ~PasswordLoginResponseTwoFactorProviders() = default;

        private:
        boost::optional<FluffyAuthenticator> authenticator;
        boost::optional<FluffyDuo> duo;
        boost::optional<FluffyEmail> email;
        boost::optional<FluffyDuo> organization_duo;
        boost::optional<FluffyRemember> remember;
        boost::optional<FluffyWebAuthn> web_authn;
        boost::optional<FluffyYubiKey> yubi_key;

        public:
        boost::optional<FluffyAuthenticator> get_authenticator() const { return authenticator; }
        void set_authenticator(boost::optional<FluffyAuthenticator> value) { this->authenticator = value; }

        /**
         * Duo-backed 2fa
         */
        boost::optional<FluffyDuo> get_duo() const { return duo; }
        void set_duo(boost::optional<FluffyDuo> value) { this->duo = value; }

        /**
         * Email 2fa
         */
        boost::optional<FluffyEmail> get_email() const { return email; }
        void set_email(boost::optional<FluffyEmail> value) { this->email = value; }

        /**
         * Duo-backed 2fa operated by an organization the user is a member of
         */
        boost::optional<FluffyDuo> get_organization_duo() const { return organization_duo; }
        void set_organization_duo(boost::optional<FluffyDuo> value) { this->organization_duo = value; }

        /**
         * Presence indicates the user has stored this device as bypassing 2fa
         */
        boost::optional<FluffyRemember> get_remember() const { return remember; }
        void set_remember(boost::optional<FluffyRemember> value) { this->remember = value; }

        /**
         * WebAuthn-backed 2fa
         */
        boost::optional<FluffyWebAuthn> get_web_authn() const { return web_authn; }
        void set_web_authn(boost::optional<FluffyWebAuthn> value) { this->web_authn = value; }

        /**
         * Yubikey-backed 2fa
         */
        boost::optional<FluffyYubiKey> get_yubi_key() const { return yubi_key; }
        void set_yubi_key(boost::optional<FluffyYubiKey> value) { this->yubi_key = value; }
    };

    class PasswordLoginResponse {
        public:
        PasswordLoginResponse() = default;
        virtual ~PasswordLoginResponse() = default;

        private:
        bool authenticated;
        boost::optional<CaptchaResponse> captcha;
        bool force_password_reset;
        bool reset_master_password;
        boost::optional<PasswordLoginResponseTwoFactorProviders> two_factor;

        public:
        const bool & get_authenticated() const { return authenticated; }
        bool & get_mutable_authenticated() { return authenticated; }
        void set_authenticated(const bool & value) { this->authenticated = value; }

        /**
         * The information required to present the user with a captcha challenge. Only present when
         * authentication fails due to requiring validation of a captcha challenge.
         */
        boost::optional<CaptchaResponse> get_captcha() const { return captcha; }
        void set_captcha(boost::optional<CaptchaResponse> value) { this->captcha = value; }

        /**
         * Whether or not the user is required to update their master password
         */
        const bool & get_force_password_reset() const { return force_password_reset; }
        bool & get_mutable_force_password_reset() { return force_password_reset; }
        void set_force_password_reset(const bool & value) { this->force_password_reset = value; }

        /**
         * TODO: What does this do?
         */
        const bool & get_reset_master_password() const { return reset_master_password; }
        bool & get_mutable_reset_master_password() { return reset_master_password; }
        void set_reset_master_password(const bool & value) { this->reset_master_password = value; }

        /**
         * The available two factor authentication options. Present only when authentication fails
         * due to requiring a second authentication factor.
         */
        boost::optional<PasswordLoginResponseTwoFactorProviders> get_two_factor() const { return two_factor; }
        void set_two_factor(boost::optional<PasswordLoginResponseTwoFactorProviders> value) { this->two_factor = value; }
    };

    class ResponseForPasswordLoginResponse {
        public:
        ResponseForPasswordLoginResponse() = default;
        virtual ~ResponseForPasswordLoginResponse() = default;

        private:
        boost::optional<PasswordLoginResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<PasswordLoginResponse> get_data() const { return data; }
        void set_data(boost::optional<PasswordLoginResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class ProjectResponse {
        public:
        ProjectResponse() = default;
        virtual ~ProjectResponse() = default;

        private:
        std::string creation_date;
        std::string id;
        std::string name;
        std::string object;
        std::string organization_id;
        std::string revision_date;

        public:
        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_object() const { return object; }
        std::string & get_mutable_object() { return object; }
        void set_object(const std::string & value) { this->object = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }
    };

    class ResponseForProjectResponse {
        public:
        ResponseForProjectResponse() = default;
        virtual ~ResponseForProjectResponse() = default;

        private:
        boost::optional<ProjectResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<ProjectResponse> get_data() const { return data; }
        void set_data(boost::optional<ProjectResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class ProjectDeleteResponse {
        public:
        ProjectDeleteResponse() = default;
        virtual ~ProjectDeleteResponse() = default;

        private:
        boost::optional<std::string> error;
        std::string id;

        public:
        boost::optional<std::string> get_error() const { return error; }
        void set_error(boost::optional<std::string> value) { this->error = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }
    };

    class ProjectsDeleteResponse {
        public:
        ProjectsDeleteResponse() = default;
        virtual ~ProjectsDeleteResponse() = default;

        private:
        std::vector<ProjectDeleteResponse> data;

        public:
        const std::vector<ProjectDeleteResponse> & get_data() const { return data; }
        std::vector<ProjectDeleteResponse> & get_mutable_data() { return data; }
        void set_data(const std::vector<ProjectDeleteResponse> & value) { this->data = value; }
    };

    class ResponseForProjectsDeleteResponse {
        public:
        ResponseForProjectsDeleteResponse() = default;
        virtual ~ResponseForProjectsDeleteResponse() = default;

        private:
        boost::optional<ProjectsDeleteResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<ProjectsDeleteResponse> get_data() const { return data; }
        void set_data(boost::optional<ProjectsDeleteResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class DatumElement {
        public:
        DatumElement() = default;
        virtual ~DatumElement() = default;

        private:
        std::string creation_date;
        std::string id;
        std::string name;
        std::string object;
        std::string organization_id;
        std::string revision_date;

        public:
        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::string & get_object() const { return object; }
        std::string & get_mutable_object() { return object; }
        void set_object(const std::string & value) { this->object = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }
    };

    class ProjectsResponse {
        public:
        ProjectsResponse() = default;
        virtual ~ProjectsResponse() = default;

        private:
        std::vector<DatumElement> data;

        public:
        const std::vector<DatumElement> & get_data() const { return data; }
        std::vector<DatumElement> & get_mutable_data() { return data; }
        void set_data(const std::vector<DatumElement> & value) { this->data = value; }
    };

    class ResponseForProjectsResponse {
        public:
        ResponseForProjectsResponse() = default;
        virtual ~ResponseForProjectsResponse() = default;

        private:
        boost::optional<ProjectsResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<ProjectsResponse> get_data() const { return data; }
        void set_data(boost::optional<ProjectsResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class SecretIdentifierResponse {
        public:
        SecretIdentifierResponse() = default;
        virtual ~SecretIdentifierResponse() = default;

        private:
        std::string id;
        std::string key;
        std::string organization_id;

        public:
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_key() const { return key; }
        std::string & get_mutable_key() { return key; }
        void set_key(const std::string & value) { this->key = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }
    };

    class SecretIdentifiersResponse {
        public:
        SecretIdentifiersResponse() = default;
        virtual ~SecretIdentifiersResponse() = default;

        private:
        std::vector<SecretIdentifierResponse> data;

        public:
        const std::vector<SecretIdentifierResponse> & get_data() const { return data; }
        std::vector<SecretIdentifierResponse> & get_mutable_data() { return data; }
        void set_data(const std::vector<SecretIdentifierResponse> & value) { this->data = value; }
    };

    class ResponseForSecretIdentifiersResponse {
        public:
        ResponseForSecretIdentifiersResponse() = default;
        virtual ~ResponseForSecretIdentifiersResponse() = default;

        private:
        boost::optional<SecretIdentifiersResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<SecretIdentifiersResponse> get_data() const { return data; }
        void set_data(boost::optional<SecretIdentifiersResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class SecretResponse {
        public:
        SecretResponse() = default;
        virtual ~SecretResponse() = default;

        private:
        std::string creation_date;
        std::string id;
        std::string key;
        std::string note;
        std::string object;
        std::string organization_id;
        boost::optional<std::string> project_id;
        std::string revision_date;
        std::string value;

        public:
        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_key() const { return key; }
        std::string & get_mutable_key() { return key; }
        void set_key(const std::string & value) { this->key = value; }

        const std::string & get_note() const { return note; }
        std::string & get_mutable_note() { return note; }
        void set_note(const std::string & value) { this->note = value; }

        const std::string & get_object() const { return object; }
        std::string & get_mutable_object() { return object; }
        void set_object(const std::string & value) { this->object = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        boost::optional<std::string> get_project_id() const { return project_id; }
        void set_project_id(boost::optional<std::string> value) { this->project_id = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    class ResponseForSecretResponse {
        public:
        ResponseForSecretResponse() = default;
        virtual ~ResponseForSecretResponse() = default;

        private:
        boost::optional<SecretResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<SecretResponse> get_data() const { return data; }
        void set_data(boost::optional<SecretResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class SecretDeleteResponse {
        public:
        SecretDeleteResponse() = default;
        virtual ~SecretDeleteResponse() = default;

        private:
        boost::optional<std::string> error;
        std::string id;

        public:
        boost::optional<std::string> get_error() const { return error; }
        void set_error(boost::optional<std::string> value) { this->error = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }
    };

    class SecretsDeleteResponse {
        public:
        SecretsDeleteResponse() = default;
        virtual ~SecretsDeleteResponse() = default;

        private:
        std::vector<SecretDeleteResponse> data;

        public:
        const std::vector<SecretDeleteResponse> & get_data() const { return data; }
        std::vector<SecretDeleteResponse> & get_mutable_data() { return data; }
        void set_data(const std::vector<SecretDeleteResponse> & value) { this->data = value; }
    };

    class ResponseForSecretsDeleteResponse {
        public:
        ResponseForSecretsDeleteResponse() = default;
        virtual ~ResponseForSecretsDeleteResponse() = default;

        private:
        boost::optional<SecretsDeleteResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<SecretsDeleteResponse> get_data() const { return data; }
        void set_data(boost::optional<SecretsDeleteResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class DatumClass {
        public:
        DatumClass() = default;
        virtual ~DatumClass() = default;

        private:
        std::string creation_date;
        std::string id;
        std::string key;
        std::string note;
        std::string object;
        std::string organization_id;
        boost::optional<std::string> project_id;
        std::string revision_date;
        std::string value;

        public:
        const std::string & get_creation_date() const { return creation_date; }
        std::string & get_mutable_creation_date() { return creation_date; }
        void set_creation_date(const std::string & value) { this->creation_date = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_key() const { return key; }
        std::string & get_mutable_key() { return key; }
        void set_key(const std::string & value) { this->key = value; }

        const std::string & get_note() const { return note; }
        std::string & get_mutable_note() { return note; }
        void set_note(const std::string & value) { this->note = value; }

        const std::string & get_object() const { return object; }
        std::string & get_mutable_object() { return object; }
        void set_object(const std::string & value) { this->object = value; }

        const std::string & get_organization_id() const { return organization_id; }
        std::string & get_mutable_organization_id() { return organization_id; }
        void set_organization_id(const std::string & value) { this->organization_id = value; }

        boost::optional<std::string> get_project_id() const { return project_id; }
        void set_project_id(boost::optional<std::string> value) { this->project_id = value; }

        const std::string & get_revision_date() const { return revision_date; }
        std::string & get_mutable_revision_date() { return revision_date; }
        void set_revision_date(const std::string & value) { this->revision_date = value; }

        const std::string & get_value() const { return value; }
        std::string & get_mutable_value() { return value; }
        void set_value(const std::string & value) { this->value = value; }
    };

    class SecretsResponse {
        public:
        SecretsResponse() = default;
        virtual ~SecretsResponse() = default;

        private:
        std::vector<DatumClass> data;

        public:
        const std::vector<DatumClass> & get_data() const { return data; }
        std::vector<DatumClass> & get_mutable_data() { return data; }
        void set_data(const std::vector<DatumClass> & value) { this->data = value; }
    };

    class ResponseForSecretsResponse {
        public:
        ResponseForSecretsResponse() = default;
        virtual ~ResponseForSecretsResponse() = default;

        private:
        boost::optional<SecretsResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<SecretsResponse> get_data() const { return data; }
        void set_data(boost::optional<SecretsResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class CipherDetailsResponse {
        public:
        CipherDetailsResponse() = default;
        virtual ~CipherDetailsResponse() = default;

        private:

        public:
    };

    class ProfileOrganizationResponse {
        public:
        ProfileOrganizationResponse() = default;
        virtual ~ProfileOrganizationResponse() = default;

        private:
        std::string id;

        public:
        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }
    };

    /**
     * Data about the user, including their encryption keys and the organizations they are a
     * part of
     */
    class ProfileResponse {
        public:
        ProfileResponse() = default;
        virtual ~ProfileResponse() = default;

        private:
        std::string email;
        std::string id;
        std::string name;
        std::vector<ProfileOrganizationResponse> organizations;

        public:
        const std::string & get_email() const { return email; }
        std::string & get_mutable_email() { return email; }
        void set_email(const std::string & value) { this->email = value; }

        const std::string & get_id() const { return id; }
        std::string & get_mutable_id() { return id; }
        void set_id(const std::string & value) { this->id = value; }

        const std::string & get_name() const { return name; }
        std::string & get_mutable_name() { return name; }
        void set_name(const std::string & value) { this->name = value; }

        const std::vector<ProfileOrganizationResponse> & get_organizations() const { return organizations; }
        std::vector<ProfileOrganizationResponse> & get_mutable_organizations() { return organizations; }
        void set_organizations(const std::vector<ProfileOrganizationResponse> & value) { this->organizations = value; }
    };

    class SyncResponse {
        public:
        SyncResponse() = default;
        virtual ~SyncResponse() = default;

        private:
        std::vector<CipherDetailsResponse> ciphers;
        ProfileResponse profile;

        public:
        /**
         * List of ciphers accesible by the user
         */
        const std::vector<CipherDetailsResponse> & get_ciphers() const { return ciphers; }
        std::vector<CipherDetailsResponse> & get_mutable_ciphers() { return ciphers; }
        void set_ciphers(const std::vector<CipherDetailsResponse> & value) { this->ciphers = value; }

        /**
         * Data about the user, including their encryption keys and the organizations they are a
         * part of
         */
        const ProfileResponse & get_profile() const { return profile; }
        ProfileResponse & get_mutable_profile() { return profile; }
        void set_profile(const ProfileResponse & value) { this->profile = value; }
    };

    class ResponseForSyncResponse {
        public:
        ResponseForSyncResponse() = default;
        virtual ~ResponseForSyncResponse() = default;

        private:
        boost::optional<SyncResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<SyncResponse> get_data() const { return data; }
        void set_data(boost::optional<SyncResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };

    class UserApiKeyResponse {
        public:
        UserApiKeyResponse() = default;
        virtual ~UserApiKeyResponse() = default;

        private:
        std::string api_key;

        public:
        /**
         * The user's API key, which represents the client_secret portion of an oauth request.
         */
        const std::string & get_api_key() const { return api_key; }
        std::string & get_mutable_api_key() { return api_key; }
        void set_api_key(const std::string & value) { this->api_key = value; }
    };

    class ResponseForUserApiKeyResponse {
        public:
        ResponseForUserApiKeyResponse() = default;
        virtual ~ResponseForUserApiKeyResponse() = default;

        private:
        boost::optional<UserApiKeyResponse> data;
        boost::optional<std::string> error_message;
        bool success;

        public:
        /**
         * The response data. Populated if `success` is true.
         */
        boost::optional<UserApiKeyResponse> get_data() const { return data; }
        void set_data(boost::optional<UserApiKeyResponse> value) { this->data = value; }

        /**
         * A message for any error that may occur. Populated if `success` is false.
         */
        boost::optional<std::string> get_error_message() const { return error_message; }
        void set_error_message(boost::optional<std::string> value) { this->error_message = value; }

        /**
         * Whether or not the SDK request succeeded.
         */
        const bool & get_success() const { return success; }
        bool & get_mutable_success() { return success; }
        void set_success(const bool & value) { this->success = value; }
    };
}

namespace quicktype {
void from_json(const json & j, ClientSettings & x);
void to_json(json & j, const ClientSettings & x);

void from_json(const json & j, AccessTokenLoginRequest & x);
void to_json(json & j, const AccessTokenLoginRequest & x);

void from_json(const json & j, ApiKeyLoginRequest & x);
void to_json(json & j, const ApiKeyLoginRequest & x);

void from_json(const json & j, FingerprintRequest & x);
void to_json(json & j, const FingerprintRequest & x);

void from_json(const json & j, SecretVerificationRequest & x);
void to_json(json & j, const SecretVerificationRequest & x);

void from_json(const json & j, TwoFactorRequest & x);
void to_json(json & j, const TwoFactorRequest & x);

void from_json(const json & j, PasswordLoginRequest & x);
void to_json(json & j, const PasswordLoginRequest & x);

void from_json(const json & j, ProjectCreateRequest & x);
void to_json(json & j, const ProjectCreateRequest & x);

void from_json(const json & j, ProjectGetRequest & x);
void to_json(json & j, const ProjectGetRequest & x);

void from_json(const json & j, ProjectsListRequest & x);
void to_json(json & j, const ProjectsListRequest & x);

void from_json(const json & j, ProjectsDeleteRequest & x);
void to_json(json & j, const ProjectsDeleteRequest & x);

void from_json(const json & j, ProjectPutRequest & x);
void to_json(json & j, const ProjectPutRequest & x);

void from_json(const json & j, ProjectsCommand & x);
void to_json(json & j, const ProjectsCommand & x);

void from_json(const json & j, SecretCreateRequest & x);
void to_json(json & j, const SecretCreateRequest & x);

void from_json(const json & j, SecretGetRequest & x);
void to_json(json & j, const SecretGetRequest & x);

void from_json(const json & j, SecretsGetRequest & x);
void to_json(json & j, const SecretsGetRequest & x);

void from_json(const json & j, SecretIdentifiersRequest & x);
void to_json(json & j, const SecretIdentifiersRequest & x);

void from_json(const json & j, SecretsDeleteRequest & x);
void to_json(json & j, const SecretsDeleteRequest & x);

void from_json(const json & j, SecretPutRequest & x);
void to_json(json & j, const SecretPutRequest & x);

void from_json(const json & j, SecretsCommand & x);
void to_json(json & j, const SecretsCommand & x);

void from_json(const json & j, SyncRequest & x);
void to_json(json & j, const SyncRequest & x);

void from_json(const json & j, Command & x);
void to_json(json & j, const Command & x);

void from_json(const json & j, Attachment & x);
void to_json(json & j, const Attachment & x);

void from_json(const json & j, Card & x);
void to_json(json & j, const Card & x);

void from_json(const json & j, Field & x);
void to_json(json & j, const Field & x);

void from_json(const json & j, Identity & x);
void to_json(json & j, const Identity & x);

void from_json(const json & j, LocalData & x);
void to_json(json & j, const LocalData & x);

void from_json(const json & j, LoginUri & x);
void to_json(json & j, const LoginUri & x);

void from_json(const json & j, Login & x);
void to_json(json & j, const Login & x);

void from_json(const json & j, PasswordHistory & x);
void to_json(json & j, const PasswordHistory & x);

void from_json(const json & j, SecureNote & x);
void to_json(json & j, const SecureNote & x);

void from_json(const json & j, Cipher & x);
void to_json(json & j, const Cipher & x);

void from_json(const json & j, AttachmentView & x);
void to_json(json & j, const AttachmentView & x);

void from_json(const json & j, CardView & x);
void to_json(json & j, const CardView & x);

void from_json(const json & j, FieldView & x);
void to_json(json & j, const FieldView & x);

void from_json(const json & j, IdentityView & x);
void to_json(json & j, const IdentityView & x);

void from_json(const json & j, LocalDataView & x);
void to_json(json & j, const LocalDataView & x);

void from_json(const json & j, LoginUriView & x);
void to_json(json & j, const LoginUriView & x);

void from_json(const json & j, LoginView & x);
void to_json(json & j, const LoginView & x);

void from_json(const json & j, PasswordHistoryView & x);
void to_json(json & j, const PasswordHistoryView & x);

void from_json(const json & j, SecureNoteView & x);
void to_json(json & j, const SecureNoteView & x);

void from_json(const json & j, CipherView & x);
void to_json(json & j, const CipherView & x);

void from_json(const json & j, Collection & x);
void to_json(json & j, const Collection & x);

void from_json(const json & j, EncryptedJson & x);
void to_json(json & j, const EncryptedJson & x);

void from_json(const json & j, ExportFormatClass & x);
void to_json(json & j, const ExportFormatClass & x);

void from_json(const json & j, Folder & x);
void to_json(json & j, const Folder & x);

void from_json(const json & j, FolderView & x);
void to_json(json & j, const FolderView & x);

void from_json(const json & j, Argon2Id & x);
void to_json(json & j, const Argon2Id & x);

void from_json(const json & j, PBkdf2 & x);
void to_json(json & j, const PBkdf2 & x);

void from_json(const json & j, Kdf & x);
void to_json(json & j, const Kdf & x);

void from_json(const json & j, InitCryptoRequest & x);
void to_json(json & j, const InitCryptoRequest & x);

void from_json(const json & j, MasterPasswordPolicyOptions & x);
void to_json(json & j, const MasterPasswordPolicyOptions & x);

void from_json(const json & j, PassphraseGeneratorRequest & x);
void to_json(json & j, const PassphraseGeneratorRequest & x);

void from_json(const json & j, PasswordGeneratorRequest & x);
void to_json(json & j, const PasswordGeneratorRequest & x);

void from_json(const json & j, DocRef & x);
void to_json(json & j, const DocRef & x);

void from_json(const json & j, PurpleAuthenticator & x);
void to_json(json & j, const PurpleAuthenticator & x);

void from_json(const json & j, PurpleDuo & x);
void to_json(json & j, const PurpleDuo & x);

void from_json(const json & j, PurpleEmail & x);
void to_json(json & j, const PurpleEmail & x);

void from_json(const json & j, PurpleRemember & x);
void to_json(json & j, const PurpleRemember & x);

void from_json(const json & j, PurpleWebAuthn & x);
void to_json(json & j, const PurpleWebAuthn & x);

void from_json(const json & j, PurpleYubiKey & x);
void to_json(json & j, const PurpleYubiKey & x);

void from_json(const json & j, ApiKeyLoginResponseTwoFactorProviders & x);
void to_json(json & j, const ApiKeyLoginResponseTwoFactorProviders & x);

void from_json(const json & j, ApiKeyLoginResponse & x);
void to_json(json & j, const ApiKeyLoginResponse & x);

void from_json(const json & j, ResponseForApiKeyLoginResponse & x);
void to_json(json & j, const ResponseForApiKeyLoginResponse & x);

void from_json(const json & j, FingerprintResponse & x);
void to_json(json & j, const FingerprintResponse & x);

void from_json(const json & j, ResponseForFingerprintResponse & x);
void to_json(json & j, const ResponseForFingerprintResponse & x);

void from_json(const json & j, CaptchaResponse & x);
void to_json(json & j, const CaptchaResponse & x);

void from_json(const json & j, FluffyAuthenticator & x);
void to_json(json & j, const FluffyAuthenticator & x);

void from_json(const json & j, FluffyDuo & x);
void to_json(json & j, const FluffyDuo & x);

void from_json(const json & j, FluffyEmail & x);
void to_json(json & j, const FluffyEmail & x);

void from_json(const json & j, FluffyRemember & x);
void to_json(json & j, const FluffyRemember & x);

void from_json(const json & j, FluffyWebAuthn & x);
void to_json(json & j, const FluffyWebAuthn & x);

void from_json(const json & j, FluffyYubiKey & x);
void to_json(json & j, const FluffyYubiKey & x);

void from_json(const json & j, PasswordLoginResponseTwoFactorProviders & x);
void to_json(json & j, const PasswordLoginResponseTwoFactorProviders & x);

void from_json(const json & j, PasswordLoginResponse & x);
void to_json(json & j, const PasswordLoginResponse & x);

void from_json(const json & j, ResponseForPasswordLoginResponse & x);
void to_json(json & j, const ResponseForPasswordLoginResponse & x);

void from_json(const json & j, ProjectResponse & x);
void to_json(json & j, const ProjectResponse & x);

void from_json(const json & j, ResponseForProjectResponse & x);
void to_json(json & j, const ResponseForProjectResponse & x);

void from_json(const json & j, ProjectDeleteResponse & x);
void to_json(json & j, const ProjectDeleteResponse & x);

void from_json(const json & j, ProjectsDeleteResponse & x);
void to_json(json & j, const ProjectsDeleteResponse & x);

void from_json(const json & j, ResponseForProjectsDeleteResponse & x);
void to_json(json & j, const ResponseForProjectsDeleteResponse & x);

void from_json(const json & j, DatumElement & x);
void to_json(json & j, const DatumElement & x);

void from_json(const json & j, ProjectsResponse & x);
void to_json(json & j, const ProjectsResponse & x);

void from_json(const json & j, ResponseForProjectsResponse & x);
void to_json(json & j, const ResponseForProjectsResponse & x);

void from_json(const json & j, SecretIdentifierResponse & x);
void to_json(json & j, const SecretIdentifierResponse & x);

void from_json(const json & j, SecretIdentifiersResponse & x);
void to_json(json & j, const SecretIdentifiersResponse & x);

void from_json(const json & j, ResponseForSecretIdentifiersResponse & x);
void to_json(json & j, const ResponseForSecretIdentifiersResponse & x);

void from_json(const json & j, SecretResponse & x);
void to_json(json & j, const SecretResponse & x);

void from_json(const json & j, ResponseForSecretResponse & x);
void to_json(json & j, const ResponseForSecretResponse & x);

void from_json(const json & j, SecretDeleteResponse & x);
void to_json(json & j, const SecretDeleteResponse & x);

void from_json(const json & j, SecretsDeleteResponse & x);
void to_json(json & j, const SecretsDeleteResponse & x);

void from_json(const json & j, ResponseForSecretsDeleteResponse & x);
void to_json(json & j, const ResponseForSecretsDeleteResponse & x);

void from_json(const json & j, DatumClass & x);
void to_json(json & j, const DatumClass & x);

void from_json(const json & j, SecretsResponse & x);
void to_json(json & j, const SecretsResponse & x);

void from_json(const json & j, ResponseForSecretsResponse & x);
void to_json(json & j, const ResponseForSecretsResponse & x);

void from_json(const json & j, CipherDetailsResponse & x);
void to_json(json & j, const CipherDetailsResponse & x);

void from_json(const json & j, ProfileOrganizationResponse & x);
void to_json(json & j, const ProfileOrganizationResponse & x);

void from_json(const json & j, ProfileResponse & x);
void to_json(json & j, const ProfileResponse & x);

void from_json(const json & j, SyncResponse & x);
void to_json(json & j, const SyncResponse & x);

void from_json(const json & j, ResponseForSyncResponse & x);
void to_json(json & j, const ResponseForSyncResponse & x);

void from_json(const json & j, UserApiKeyResponse & x);
void to_json(json & j, const UserApiKeyResponse & x);

void from_json(const json & j, ResponseForUserApiKeyResponse & x);
void to_json(json & j, const ResponseForUserApiKeyResponse & x);

void from_json(const json & j, DeviceType & x);
void to_json(json & j, const DeviceType & x);

void from_json(const json & j, TwoFactorProvider & x);
void to_json(json & j, const TwoFactorProvider & x);

void from_json(const json & j, LinkedIdType & x);
void to_json(json & j, const LinkedIdType & x);

void from_json(const json & j, FieldType & x);
void to_json(json & j, const FieldType & x);

void from_json(const json & j, UriMatchType & x);
void to_json(json & j, const UriMatchType & x);

void from_json(const json & j, CipherRepromptType & x);
void to_json(json & j, const CipherRepromptType & x);

void from_json(const json & j, SecureNoteType & x);
void to_json(json & j, const SecureNoteType & x);

void from_json(const json & j, CipherType & x);
void to_json(json & j, const CipherType & x);

void from_json(const json & j, ExportFormatEnum & x);
void to_json(json & j, const ExportFormatEnum & x);
}
namespace nlohmann {
template <>
struct adl_serializer<boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum>> {
    static void from_json(const json & j, boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum> & x);
    static void to_json(json & j, const boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum> & x);
};
}
namespace quicktype {
    inline void from_json(const json & j, ClientSettings& x) {
        x.set_api_url(j.at("apiUrl").get<std::string>());
        x.set_device_type(j.at("deviceType").get<DeviceType>());
        x.set_identity_url(j.at("identityUrl").get<std::string>());
        x.set_user_agent(j.at("userAgent").get<std::string>());
    }

    inline void to_json(json & j, const ClientSettings & x) {
        j = json::object();
        j["apiUrl"] = x.get_api_url();
        j["deviceType"] = x.get_device_type();
        j["identityUrl"] = x.get_identity_url();
        j["userAgent"] = x.get_user_agent();
    }

    inline void from_json(const json & j, AccessTokenLoginRequest& x) {
        x.set_access_token(j.at("accessToken").get<std::string>());
    }

    inline void to_json(json & j, const AccessTokenLoginRequest & x) {
        j = json::object();
        j["accessToken"] = x.get_access_token();
    }

    inline void from_json(const json & j, ApiKeyLoginRequest& x) {
        x.set_client_id(j.at("clientId").get<std::string>());
        x.set_client_secret(j.at("clientSecret").get<std::string>());
        x.set_password(j.at("password").get<std::string>());
    }

    inline void to_json(json & j, const ApiKeyLoginRequest & x) {
        j = json::object();
        j["clientId"] = x.get_client_id();
        j["clientSecret"] = x.get_client_secret();
        j["password"] = x.get_password();
    }

    inline void from_json(const json & j, FingerprintRequest& x) {
        x.set_fingerprint_material(j.at("fingerprintMaterial").get<std::string>());
        x.set_public_key(j.at("publicKey").get<std::string>());
    }

    inline void to_json(json & j, const FingerprintRequest & x) {
        j = json::object();
        j["fingerprintMaterial"] = x.get_fingerprint_material();
        j["publicKey"] = x.get_public_key();
    }

    inline void from_json(const json & j, SecretVerificationRequest& x) {
        x.set_master_password(get_stack_optional<std::string>(j, "masterPassword"));
        x.set_otp(get_stack_optional<std::string>(j, "otp"));
    }

    inline void to_json(json & j, const SecretVerificationRequest & x) {
        j = json::object();
        j["masterPassword"] = x.get_master_password();
        j["otp"] = x.get_otp();
    }

    inline void from_json(const json & j, TwoFactorRequest& x) {
        x.set_provider(j.at("provider").get<TwoFactorProvider>());
        x.set_remember(j.at("remember").get<bool>());
        x.set_token(j.at("token").get<std::string>());
    }

    inline void to_json(json & j, const TwoFactorRequest & x) {
        j = json::object();
        j["provider"] = x.get_provider();
        j["remember"] = x.get_remember();
        j["token"] = x.get_token();
    }

    inline void from_json(const json & j, PasswordLoginRequest& x) {
        x.set_email(j.at("email").get<std::string>());
        x.set_password(j.at("password").get<std::string>());
        x.set_two_factor(get_stack_optional<TwoFactorRequest>(j, "twoFactor"));
    }

    inline void to_json(json & j, const PasswordLoginRequest & x) {
        j = json::object();
        j["email"] = x.get_email();
        j["password"] = x.get_password();
        j["twoFactor"] = x.get_two_factor();
    }

    inline void from_json(const json & j, ProjectCreateRequest& x) {
        x.set_name(j.at("name").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
    }

    inline void to_json(json & j, const ProjectCreateRequest & x) {
        j = json::object();
        j["name"] = x.get_name();
        j["organizationId"] = x.get_organization_id();
    }

    inline void from_json(const json & j, ProjectGetRequest& x) {
        x.set_id(j.at("id").get<std::string>());
    }

    inline void to_json(json & j, const ProjectGetRequest & x) {
        j = json::object();
        j["id"] = x.get_id();
    }

    inline void from_json(const json & j, ProjectsListRequest& x) {
        x.set_organization_id(j.at("organizationId").get<std::string>());
    }

    inline void to_json(json & j, const ProjectsListRequest & x) {
        j = json::object();
        j["organizationId"] = x.get_organization_id();
    }

    inline void from_json(const json & j, ProjectsDeleteRequest& x) {
        x.set_ids(j.at("ids").get<std::vector<std::string>>());
    }

    inline void to_json(json & j, const ProjectsDeleteRequest & x) {
        j = json::object();
        j["ids"] = x.get_ids();
    }

    inline void from_json(const json & j, ProjectPutRequest& x) {
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
    }

    inline void to_json(json & j, const ProjectPutRequest & x) {
        j = json::object();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["organizationId"] = x.get_organization_id();
    }

    inline void from_json(const json & j, ProjectsCommand& x) {
        x.set_get(get_stack_optional<ProjectGetRequest>(j, "get"));
        x.set_create(get_stack_optional<ProjectCreateRequest>(j, "create"));
        x.set_list(get_stack_optional<ProjectsListRequest>(j, "list"));
        x.set_update(get_stack_optional<ProjectPutRequest>(j, "update"));
        x.set_projects_command_delete(get_stack_optional<ProjectsDeleteRequest>(j, "delete"));
    }

    inline void to_json(json & j, const ProjectsCommand & x) {
        j = json::object();
        j["get"] = x.get_get();
        j["create"] = x.get_create();
        j["list"] = x.get_list();
        j["update"] = x.get_update();
        j["delete"] = x.get_projects_command_delete();
    }

    inline void from_json(const json & j, SecretCreateRequest& x) {
        x.set_key(j.at("key").get<std::string>());
        x.set_note(j.at("note").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_project_ids(get_stack_optional<std::vector<std::string>>(j, "projectIds"));
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const SecretCreateRequest & x) {
        j = json::object();
        j["key"] = x.get_key();
        j["note"] = x.get_note();
        j["organizationId"] = x.get_organization_id();
        j["projectIds"] = x.get_project_ids();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, SecretGetRequest& x) {
        x.set_id(j.at("id").get<std::string>());
    }

    inline void to_json(json & j, const SecretGetRequest & x) {
        j = json::object();
        j["id"] = x.get_id();
    }

    inline void from_json(const json & j, SecretsGetRequest& x) {
        x.set_ids(j.at("ids").get<std::vector<std::string>>());
    }

    inline void to_json(json & j, const SecretsGetRequest & x) {
        j = json::object();
        j["ids"] = x.get_ids();
    }

    inline void from_json(const json & j, SecretIdentifiersRequest& x) {
        x.set_organization_id(j.at("organizationId").get<std::string>());
    }

    inline void to_json(json & j, const SecretIdentifiersRequest & x) {
        j = json::object();
        j["organizationId"] = x.get_organization_id();
    }

    inline void from_json(const json & j, SecretsDeleteRequest& x) {
        x.set_ids(j.at("ids").get<std::vector<std::string>>());
    }

    inline void to_json(json & j, const SecretsDeleteRequest & x) {
        j = json::object();
        j["ids"] = x.get_ids();
    }

    inline void from_json(const json & j, SecretPutRequest& x) {
        x.set_id(j.at("id").get<std::string>());
        x.set_key(j.at("key").get<std::string>());
        x.set_note(j.at("note").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_project_ids(get_stack_optional<std::vector<std::string>>(j, "projectIds"));
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const SecretPutRequest & x) {
        j = json::object();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["note"] = x.get_note();
        j["organizationId"] = x.get_organization_id();
        j["projectIds"] = x.get_project_ids();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, SecretsCommand& x) {
        x.set_get(get_stack_optional<SecretGetRequest>(j, "get"));
        x.set_get_by_ids(get_stack_optional<SecretsGetRequest>(j, "getByIds"));
        x.set_create(get_stack_optional<SecretCreateRequest>(j, "create"));
        x.set_list(get_stack_optional<SecretIdentifiersRequest>(j, "list"));
        x.set_update(get_stack_optional<SecretPutRequest>(j, "update"));
        x.set_secrets_command_delete(get_stack_optional<SecretsDeleteRequest>(j, "delete"));
    }

    inline void to_json(json & j, const SecretsCommand & x) {
        j = json::object();
        j["get"] = x.get_get();
        j["getByIds"] = x.get_get_by_ids();
        j["create"] = x.get_create();
        j["list"] = x.get_list();
        j["update"] = x.get_update();
        j["delete"] = x.get_secrets_command_delete();
    }

    inline void from_json(const json & j, SyncRequest& x) {
        x.set_exclude_subdomains(get_stack_optional<bool>(j, "excludeSubdomains"));
    }

    inline void to_json(json & j, const SyncRequest & x) {
        j = json::object();
        j["excludeSubdomains"] = x.get_exclude_subdomains();
    }

    inline void from_json(const json & j, Command& x) {
        x.set_password_login(get_stack_optional<PasswordLoginRequest>(j, "passwordLogin"));
        x.set_api_key_login(get_stack_optional<ApiKeyLoginRequest>(j, "apiKeyLogin"));
        x.set_access_token_login(get_stack_optional<AccessTokenLoginRequest>(j, "accessTokenLogin"));
        x.set_get_user_api_key(get_stack_optional<SecretVerificationRequest>(j, "getUserApiKey"));
        x.set_fingerprint(get_stack_optional<FingerprintRequest>(j, "fingerprint"));
        x.set_sync(get_stack_optional<SyncRequest>(j, "sync"));
        x.set_secrets(get_stack_optional<SecretsCommand>(j, "secrets"));
        x.set_projects(get_stack_optional<ProjectsCommand>(j, "projects"));
    }

    inline void to_json(json & j, const Command & x) {
        j = json::object();
        j["passwordLogin"] = x.get_password_login();
        j["apiKeyLogin"] = x.get_api_key_login();
        j["accessTokenLogin"] = x.get_access_token_login();
        j["getUserApiKey"] = x.get_get_user_api_key();
        j["fingerprint"] = x.get_fingerprint();
        j["sync"] = x.get_sync();
        j["secrets"] = x.get_secrets();
        j["projects"] = x.get_projects();
    }

    inline void from_json(const json & j, Attachment& x) {
        x.set_file_name(get_stack_optional<std::string>(j, "fileName"));
        x.set_id(get_stack_optional<std::string>(j, "id"));
        x.set_key(get_stack_optional<std::string>(j, "key"));
        x.set_size(get_stack_optional<std::string>(j, "size"));
        x.set_size_name(get_stack_optional<std::string>(j, "sizeName"));
        x.set_url(get_stack_optional<std::string>(j, "url"));
    }

    inline void to_json(json & j, const Attachment & x) {
        j = json::object();
        j["fileName"] = x.get_file_name();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["size"] = x.get_size();
        j["sizeName"] = x.get_size_name();
        j["url"] = x.get_url();
    }

    inline void from_json(const json & j, Card& x) {
        x.set_brand(get_stack_optional<std::string>(j, "brand"));
        x.set_cardholder_name(get_stack_optional<std::string>(j, "cardholderName"));
        x.set_code(get_stack_optional<std::string>(j, "code"));
        x.set_exp_month(get_stack_optional<std::string>(j, "expMonth"));
        x.set_exp_year(get_stack_optional<std::string>(j, "expYear"));
        x.set_number(get_stack_optional<std::string>(j, "number"));
    }

    inline void to_json(json & j, const Card & x) {
        j = json::object();
        j["brand"] = x.get_brand();
        j["cardholderName"] = x.get_cardholder_name();
        j["code"] = x.get_code();
        j["expMonth"] = x.get_exp_month();
        j["expYear"] = x.get_exp_year();
        j["number"] = x.get_number();
    }

    inline void from_json(const json & j, Field& x) {
        x.set_linked_id(get_stack_optional<LinkedIdType>(j, "linkedId"));
        x.set_name(j.at("name").get<std::string>());
        x.set_type(j.at("type").get<FieldType>());
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const Field & x) {
        j = json::object();
        j["linkedId"] = x.get_linked_id();
        j["name"] = x.get_name();
        j["type"] = x.get_type();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, Identity& x) {
        x.set_address1(get_stack_optional<std::string>(j, "address1"));
        x.set_address2(get_stack_optional<std::string>(j, "address2"));
        x.set_address3(get_stack_optional<std::string>(j, "address3"));
        x.set_city(get_stack_optional<std::string>(j, "city"));
        x.set_company(get_stack_optional<std::string>(j, "company"));
        x.set_country(get_stack_optional<std::string>(j, "country"));
        x.set_email(get_stack_optional<std::string>(j, "email"));
        x.set_first_name(get_stack_optional<std::string>(j, "firstName"));
        x.set_last_name(get_stack_optional<std::string>(j, "lastName"));
        x.set_license_number(get_stack_optional<std::string>(j, "licenseNumber"));
        x.set_middle_name(get_stack_optional<std::string>(j, "middleName"));
        x.set_passport_number(get_stack_optional<std::string>(j, "passportNumber"));
        x.set_phone(get_stack_optional<std::string>(j, "phone"));
        x.set_postal_code(get_stack_optional<std::string>(j, "postalCode"));
        x.set_ssn(get_stack_optional<std::string>(j, "ssn"));
        x.set_state(get_stack_optional<std::string>(j, "state"));
        x.set_title(get_stack_optional<std::string>(j, "title"));
        x.set_username(get_stack_optional<std::string>(j, "username"));
    }

    inline void to_json(json & j, const Identity & x) {
        j = json::object();
        j["address1"] = x.get_address1();
        j["address2"] = x.get_address2();
        j["address3"] = x.get_address3();
        j["city"] = x.get_city();
        j["company"] = x.get_company();
        j["country"] = x.get_country();
        j["email"] = x.get_email();
        j["firstName"] = x.get_first_name();
        j["lastName"] = x.get_last_name();
        j["licenseNumber"] = x.get_license_number();
        j["middleName"] = x.get_middle_name();
        j["passportNumber"] = x.get_passport_number();
        j["phone"] = x.get_phone();
        j["postalCode"] = x.get_postal_code();
        j["ssn"] = x.get_ssn();
        j["state"] = x.get_state();
        j["title"] = x.get_title();
        j["username"] = x.get_username();
    }

    inline void from_json(const json & j, LocalData& x) {
        x.set_last_launched(get_stack_optional<int64_t>(j, "lastLaunched"));
        x.set_last_used_date(get_stack_optional<int64_t>(j, "lastUsedDate"));
    }

    inline void to_json(json & j, const LocalData & x) {
        j = json::object();
        j["lastLaunched"] = x.get_last_launched();
        j["lastUsedDate"] = x.get_last_used_date();
    }

    inline void from_json(const json & j, LoginUri& x) {
        x.set_match(get_stack_optional<UriMatchType>(j, "match"));
        x.set_uri(j.at("uri").get<std::string>());
    }

    inline void to_json(json & j, const LoginUri & x) {
        j = json::object();
        j["match"] = x.get_match();
        j["uri"] = x.get_uri();
    }

    inline void from_json(const json & j, Login& x) {
        x.set_autofill_on_page_load(get_stack_optional<bool>(j, "autofillOnPageLoad"));
        x.set_password(j.at("password").get<std::string>());
        x.set_password_revision_date(get_stack_optional<std::string>(j, "passwordRevisionDate"));
        x.set_totp(get_stack_optional<std::string>(j, "totp"));
        x.set_uris(j.at("uris").get<std::vector<LoginUri>>());
        x.set_username(j.at("username").get<std::string>());
    }

    inline void to_json(json & j, const Login & x) {
        j = json::object();
        j["autofillOnPageLoad"] = x.get_autofill_on_page_load();
        j["password"] = x.get_password();
        j["passwordRevisionDate"] = x.get_password_revision_date();
        j["totp"] = x.get_totp();
        j["uris"] = x.get_uris();
        j["username"] = x.get_username();
    }

    inline void from_json(const json & j, PasswordHistory& x) {
        x.set_last_used_date(j.at("lastUsedDate").get<std::string>());
        x.set_password(j.at("password").get<std::string>());
    }

    inline void to_json(json & j, const PasswordHistory & x) {
        j = json::object();
        j["lastUsedDate"] = x.get_last_used_date();
        j["password"] = x.get_password();
    }

    inline void from_json(const json & j, SecureNote& x) {
        x.set_type(j.at("type").get<SecureNoteType>());
    }

    inline void to_json(json & j, const SecureNote & x) {
        j = json::object();
        j["type"] = x.get_type();
    }

    inline void from_json(const json & j, Cipher& x) {
        x.set_attachments(j.at("attachments").get<std::vector<Attachment>>());
        x.set_card(get_stack_optional<Card>(j, "card"));
        x.set_collection_ids(j.at("collectionIds").get<std::vector<std::string>>());
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_deleted_date(get_stack_optional<std::string>(j, "deletedDate"));
        x.set_edit(j.at("edit").get<bool>());
        x.set_favorite(j.at("favorite").get<bool>());
        x.set_fields(j.at("fields").get<std::vector<Field>>());
        x.set_folder_id(get_stack_optional<std::string>(j, "folderId"));
        x.set_id(get_stack_optional<std::string>(j, "id"));
        x.set_identity(get_stack_optional<Identity>(j, "identity"));
        x.set_local_data(get_stack_optional<LocalData>(j, "localData"));
        x.set_login(get_stack_optional<Login>(j, "login"));
        x.set_name(j.at("name").get<std::string>());
        x.set_notes(j.at("notes").get<std::string>());
        x.set_organization_id(get_stack_optional<std::string>(j, "organizationId"));
        x.set_organization_use_totp(j.at("organizationUseTotp").get<bool>());
        x.set_password_history(j.at("passwordHistory").get<std::vector<PasswordHistory>>());
        x.set_reprompt(j.at("reprompt").get<CipherRepromptType>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
        x.set_secure_note(get_stack_optional<SecureNote>(j, "secureNote"));
        x.set_type(j.at("type").get<CipherType>());
        x.set_view_password(j.at("viewPassword").get<bool>());
    }

    inline void to_json(json & j, const Cipher & x) {
        j = json::object();
        j["attachments"] = x.get_attachments();
        j["card"] = x.get_card();
        j["collectionIds"] = x.get_collection_ids();
        j["creationDate"] = x.get_creation_date();
        j["deletedDate"] = x.get_deleted_date();
        j["edit"] = x.get_edit();
        j["favorite"] = x.get_favorite();
        j["fields"] = x.get_fields();
        j["folderId"] = x.get_folder_id();
        j["id"] = x.get_id();
        j["identity"] = x.get_identity();
        j["localData"] = x.get_local_data();
        j["login"] = x.get_login();
        j["name"] = x.get_name();
        j["notes"] = x.get_notes();
        j["organizationId"] = x.get_organization_id();
        j["organizationUseTotp"] = x.get_organization_use_totp();
        j["passwordHistory"] = x.get_password_history();
        j["reprompt"] = x.get_reprompt();
        j["revisionDate"] = x.get_revision_date();
        j["secureNote"] = x.get_secure_note();
        j["type"] = x.get_type();
        j["viewPassword"] = x.get_view_password();
    }

    inline void from_json(const json & j, AttachmentView& x) {
        x.set_file_name(get_stack_optional<std::string>(j, "fileName"));
        x.set_id(get_stack_optional<std::string>(j, "id"));
        x.set_key(get_stack_optional<std::string>(j, "key"));
        x.set_size(get_stack_optional<std::string>(j, "size"));
        x.set_size_name(get_stack_optional<std::string>(j, "sizeName"));
        x.set_url(get_stack_optional<std::string>(j, "url"));
    }

    inline void to_json(json & j, const AttachmentView & x) {
        j = json::object();
        j["fileName"] = x.get_file_name();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["size"] = x.get_size();
        j["sizeName"] = x.get_size_name();
        j["url"] = x.get_url();
    }

    inline void from_json(const json & j, CardView& x) {
        x.set_brand(get_stack_optional<std::string>(j, "brand"));
        x.set_cardholder_name(get_stack_optional<std::string>(j, "cardholderName"));
        x.set_code(get_stack_optional<std::string>(j, "code"));
        x.set_exp_month(get_stack_optional<std::string>(j, "expMonth"));
        x.set_exp_year(get_stack_optional<std::string>(j, "expYear"));
        x.set_number(get_stack_optional<std::string>(j, "number"));
    }

    inline void to_json(json & j, const CardView & x) {
        j = json::object();
        j["brand"] = x.get_brand();
        j["cardholderName"] = x.get_cardholder_name();
        j["code"] = x.get_code();
        j["expMonth"] = x.get_exp_month();
        j["expYear"] = x.get_exp_year();
        j["number"] = x.get_number();
    }

    inline void from_json(const json & j, FieldView& x) {
        x.set_linked_id(get_stack_optional<LinkedIdType>(j, "linkedId"));
        x.set_name(j.at("name").get<std::string>());
        x.set_type(j.at("type").get<FieldType>());
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const FieldView & x) {
        j = json::object();
        j["linkedId"] = x.get_linked_id();
        j["name"] = x.get_name();
        j["type"] = x.get_type();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, IdentityView& x) {
        x.set_address1(get_stack_optional<std::string>(j, "address1"));
        x.set_address2(get_stack_optional<std::string>(j, "address2"));
        x.set_address3(get_stack_optional<std::string>(j, "address3"));
        x.set_city(get_stack_optional<std::string>(j, "city"));
        x.set_company(get_stack_optional<std::string>(j, "company"));
        x.set_country(get_stack_optional<std::string>(j, "country"));
        x.set_email(get_stack_optional<std::string>(j, "email"));
        x.set_first_name(get_stack_optional<std::string>(j, "firstName"));
        x.set_last_name(get_stack_optional<std::string>(j, "lastName"));
        x.set_license_number(get_stack_optional<std::string>(j, "licenseNumber"));
        x.set_middle_name(get_stack_optional<std::string>(j, "middleName"));
        x.set_passport_number(get_stack_optional<std::string>(j, "passportNumber"));
        x.set_phone(get_stack_optional<std::string>(j, "phone"));
        x.set_postal_code(get_stack_optional<std::string>(j, "postalCode"));
        x.set_ssn(get_stack_optional<std::string>(j, "ssn"));
        x.set_state(get_stack_optional<std::string>(j, "state"));
        x.set_title(get_stack_optional<std::string>(j, "title"));
        x.set_username(get_stack_optional<std::string>(j, "username"));
    }

    inline void to_json(json & j, const IdentityView & x) {
        j = json::object();
        j["address1"] = x.get_address1();
        j["address2"] = x.get_address2();
        j["address3"] = x.get_address3();
        j["city"] = x.get_city();
        j["company"] = x.get_company();
        j["country"] = x.get_country();
        j["email"] = x.get_email();
        j["firstName"] = x.get_first_name();
        j["lastName"] = x.get_last_name();
        j["licenseNumber"] = x.get_license_number();
        j["middleName"] = x.get_middle_name();
        j["passportNumber"] = x.get_passport_number();
        j["phone"] = x.get_phone();
        j["postalCode"] = x.get_postal_code();
        j["ssn"] = x.get_ssn();
        j["state"] = x.get_state();
        j["title"] = x.get_title();
        j["username"] = x.get_username();
    }

    inline void from_json(const json & j, LocalDataView& x) {
        x.set_last_launched(get_stack_optional<int64_t>(j, "lastLaunched"));
        x.set_last_used_date(get_stack_optional<int64_t>(j, "lastUsedDate"));
    }

    inline void to_json(json & j, const LocalDataView & x) {
        j = json::object();
        j["lastLaunched"] = x.get_last_launched();
        j["lastUsedDate"] = x.get_last_used_date();
    }

    inline void from_json(const json & j, LoginUriView& x) {
        x.set_match(get_stack_optional<UriMatchType>(j, "match"));
        x.set_uri(j.at("uri").get<std::string>());
    }

    inline void to_json(json & j, const LoginUriView & x) {
        j = json::object();
        j["match"] = x.get_match();
        j["uri"] = x.get_uri();
    }

    inline void from_json(const json & j, LoginView& x) {
        x.set_autofill_on_page_load(get_stack_optional<bool>(j, "autofillOnPageLoad"));
        x.set_password(j.at("password").get<std::string>());
        x.set_password_revision_date(get_stack_optional<std::string>(j, "passwordRevisionDate"));
        x.set_totp(get_stack_optional<std::string>(j, "totp"));
        x.set_uris(j.at("uris").get<std::vector<LoginUriView>>());
        x.set_username(j.at("username").get<std::string>());
    }

    inline void to_json(json & j, const LoginView & x) {
        j = json::object();
        j["autofillOnPageLoad"] = x.get_autofill_on_page_load();
        j["password"] = x.get_password();
        j["passwordRevisionDate"] = x.get_password_revision_date();
        j["totp"] = x.get_totp();
        j["uris"] = x.get_uris();
        j["username"] = x.get_username();
    }

    inline void from_json(const json & j, PasswordHistoryView& x) {
        x.set_last_used_date(j.at("lastUsedDate").get<std::string>());
        x.set_password(j.at("password").get<std::string>());
    }

    inline void to_json(json & j, const PasswordHistoryView & x) {
        j = json::object();
        j["lastUsedDate"] = x.get_last_used_date();
        j["password"] = x.get_password();
    }

    inline void from_json(const json & j, SecureNoteView& x) {
        x.set_type(j.at("type").get<SecureNoteType>());
    }

    inline void to_json(json & j, const SecureNoteView & x) {
        j = json::object();
        j["type"] = x.get_type();
    }

    inline void from_json(const json & j, CipherView& x) {
        x.set_attachments(j.at("attachments").get<std::vector<AttachmentView>>());
        x.set_card(get_stack_optional<CardView>(j, "card"));
        x.set_collection_ids(j.at("collectionIds").get<std::vector<std::string>>());
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_deleted_date(get_stack_optional<std::string>(j, "deletedDate"));
        x.set_edit(j.at("edit").get<bool>());
        x.set_favorite(j.at("favorite").get<bool>());
        x.set_fields(j.at("fields").get<std::vector<FieldView>>());
        x.set_folder_id(get_stack_optional<std::string>(j, "folderId"));
        x.set_id(get_stack_optional<std::string>(j, "id"));
        x.set_identity(get_stack_optional<IdentityView>(j, "identity"));
        x.set_local_data(get_stack_optional<LocalDataView>(j, "localData"));
        x.set_login(get_stack_optional<LoginView>(j, "login"));
        x.set_name(j.at("name").get<std::string>());
        x.set_notes(j.at("notes").get<std::string>());
        x.set_organization_id(get_stack_optional<std::string>(j, "organizationId"));
        x.set_organization_use_totp(j.at("organizationUseTotp").get<bool>());
        x.set_password_history(j.at("passwordHistory").get<std::vector<PasswordHistoryView>>());
        x.set_reprompt(j.at("reprompt").get<CipherRepromptType>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
        x.set_secure_note(get_stack_optional<SecureNoteView>(j, "secureNote"));
        x.set_type(j.at("type").get<CipherType>());
        x.set_view_password(j.at("viewPassword").get<bool>());
    }

    inline void to_json(json & j, const CipherView & x) {
        j = json::object();
        j["attachments"] = x.get_attachments();
        j["card"] = x.get_card();
        j["collectionIds"] = x.get_collection_ids();
        j["creationDate"] = x.get_creation_date();
        j["deletedDate"] = x.get_deleted_date();
        j["edit"] = x.get_edit();
        j["favorite"] = x.get_favorite();
        j["fields"] = x.get_fields();
        j["folderId"] = x.get_folder_id();
        j["id"] = x.get_id();
        j["identity"] = x.get_identity();
        j["localData"] = x.get_local_data();
        j["login"] = x.get_login();
        j["name"] = x.get_name();
        j["notes"] = x.get_notes();
        j["organizationId"] = x.get_organization_id();
        j["organizationUseTotp"] = x.get_organization_use_totp();
        j["passwordHistory"] = x.get_password_history();
        j["reprompt"] = x.get_reprompt();
        j["revisionDate"] = x.get_revision_date();
        j["secureNote"] = x.get_secure_note();
        j["type"] = x.get_type();
        j["viewPassword"] = x.get_view_password();
    }

    inline void from_json(const json & j, Collection& x) {
        x.set_external_id(get_stack_optional<std::string>(j, "externalId"));
        x.set_hide_passwords(j.at("hidePasswords").get<bool>());
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_read_only(j.at("readOnly").get<bool>());
    }

    inline void to_json(json & j, const Collection & x) {
        j = json::object();
        j["externalId"] = x.get_external_id();
        j["hidePasswords"] = x.get_hide_passwords();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["organizationId"] = x.get_organization_id();
        j["readOnly"] = x.get_read_only();
    }

    inline void from_json(const json & j, EncryptedJson& x) {
        x.set_password(j.at("password").get<std::string>());
    }

    inline void to_json(json & j, const EncryptedJson & x) {
        j = json::object();
        j["password"] = x.get_password();
    }

    inline void from_json(const json & j, ExportFormatClass& x) {
        x.set_encrypted_json(j.at("EncryptedJson").get<EncryptedJson>());
    }

    inline void to_json(json & j, const ExportFormatClass & x) {
        j = json::object();
        j["EncryptedJson"] = x.get_encrypted_json();
    }

    inline void from_json(const json & j, Folder& x) {
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
    }

    inline void to_json(json & j, const Folder & x) {
        j = json::object();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["revisionDate"] = x.get_revision_date();
    }

    inline void from_json(const json & j, FolderView& x) {
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
    }

    inline void to_json(json & j, const FolderView & x) {
        j = json::object();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["revisionDate"] = x.get_revision_date();
    }

    inline void from_json(const json & j, Argon2Id& x) {
        x.set_iterations(j.at("iterations").get<int64_t>());
        x.set_memory(j.at("memory").get<int64_t>());
        x.set_parallelism(j.at("parallelism").get<int64_t>());
    }

    inline void to_json(json & j, const Argon2Id & x) {
        j = json::object();
        j["iterations"] = x.get_iterations();
        j["memory"] = x.get_memory();
        j["parallelism"] = x.get_parallelism();
    }

    inline void from_json(const json & j, PBkdf2& x) {
        x.set_iterations(j.at("iterations").get<int64_t>());
    }

    inline void to_json(json & j, const PBkdf2 & x) {
        j = json::object();
        j["iterations"] = x.get_iterations();
    }

    inline void from_json(const json & j, Kdf& x) {
        x.set_p_bkdf2(get_stack_optional<PBkdf2>(j, "pBKDF2"));
        x.set_argon2_id(get_stack_optional<Argon2Id>(j, "argon2id"));
    }

    inline void to_json(json & j, const Kdf & x) {
        j = json::object();
        j["pBKDF2"] = x.get_p_bkdf2();
        j["argon2id"] = x.get_argon2_id();
    }

    inline void from_json(const json & j, InitCryptoRequest& x) {
        x.set_email(j.at("email").get<std::string>());
        x.set_kdf_params(j.at("kdfParams").get<Kdf>());
        x.set_organization_keys(j.at("organizationKeys").get<std::map<std::string, std::string>>());
        x.set_password(j.at("password").get<std::string>());
        x.set_private_key(j.at("privateKey").get<std::string>());
        x.set_user_key(j.at("userKey").get<std::string>());
    }

    inline void to_json(json & j, const InitCryptoRequest & x) {
        j = json::object();
        j["email"] = x.get_email();
        j["kdfParams"] = x.get_kdf_params();
        j["organizationKeys"] = x.get_organization_keys();
        j["password"] = x.get_password();
        j["privateKey"] = x.get_private_key();
        j["userKey"] = x.get_user_key();
    }

    inline void from_json(const json & j, MasterPasswordPolicyOptions& x) {
        x.set_enforce_on_login(j.at("enforce_on_login").get<bool>());
        x.set_min_complexity(j.at("min_complexity").get<int64_t>());
        x.set_min_length(j.at("min_length").get<int64_t>());
        x.set_require_lower(j.at("require_lower").get<bool>());
        x.set_require_numbers(j.at("require_numbers").get<bool>());
        x.set_require_special(j.at("require_special").get<bool>());
        x.set_require_upper(j.at("require_upper").get<bool>());
    }

    inline void to_json(json & j, const MasterPasswordPolicyOptions & x) {
        j = json::object();
        j["enforce_on_login"] = x.get_enforce_on_login();
        j["min_complexity"] = x.get_min_complexity();
        j["min_length"] = x.get_min_length();
        j["require_lower"] = x.get_require_lower();
        j["require_numbers"] = x.get_require_numbers();
        j["require_special"] = x.get_require_special();
        j["require_upper"] = x.get_require_upper();
    }

    inline void from_json(const json & j, PassphraseGeneratorRequest& x) {
        x.set_capitalize(get_stack_optional<bool>(j, "capitalize"));
        x.set_include_number(get_stack_optional<bool>(j, "includeNumber"));
        x.set_num_words(get_stack_optional<int64_t>(j, "numWords"));
        x.set_word_separator(get_stack_optional<std::string>(j, "wordSeparator"));
    }

    inline void to_json(json & j, const PassphraseGeneratorRequest & x) {
        j = json::object();
        j["capitalize"] = x.get_capitalize();
        j["includeNumber"] = x.get_include_number();
        j["numWords"] = x.get_num_words();
        j["wordSeparator"] = x.get_word_separator();
    }

    inline void from_json(const json & j, PasswordGeneratorRequest& x) {
        x.set_avoid_ambiguous(get_stack_optional<bool>(j, "avoidAmbiguous"));
        x.set_length(get_stack_optional<int64_t>(j, "length"));
        x.set_lowercase(j.at("lowercase").get<bool>());
        x.set_min_lowercase(get_stack_optional<bool>(j, "minLowercase"));
        x.set_min_number(get_stack_optional<bool>(j, "minNumber"));
        x.set_min_special(get_stack_optional<bool>(j, "minSpecial"));
        x.set_min_uppercase(get_stack_optional<bool>(j, "minUppercase"));
        x.set_numbers(j.at("numbers").get<bool>());
        x.set_special(j.at("special").get<bool>());
        x.set_uppercase(j.at("uppercase").get<bool>());
    }

    inline void to_json(json & j, const PasswordGeneratorRequest & x) {
        j = json::object();
        j["avoidAmbiguous"] = x.get_avoid_ambiguous();
        j["length"] = x.get_length();
        j["lowercase"] = x.get_lowercase();
        j["minLowercase"] = x.get_min_lowercase();
        j["minNumber"] = x.get_min_number();
        j["minSpecial"] = x.get_min_special();
        j["minUppercase"] = x.get_min_uppercase();
        j["numbers"] = x.get_numbers();
        j["special"] = x.get_special();
        j["uppercase"] = x.get_uppercase();
    }

    inline void from_json(const json & j, DocRef& x) {
        x.set_cipher(get_stack_optional<Cipher>(j, "Cipher"));
        x.set_cipher_view(get_stack_optional<CipherView>(j, "CipherView"));
        x.set_collection(get_stack_optional<Collection>(j, "Collection"));
        x.set_folder(get_stack_optional<Folder>(j, "Folder"));
        x.set_folder_view(get_stack_optional<FolderView>(j, "FolderView"));
        x.set_init_crypto_request(get_stack_optional<InitCryptoRequest>(j, "InitCryptoRequest"));
        x.set_password_generator_request(get_stack_optional<PasswordGeneratorRequest>(j, "PasswordGeneratorRequest"));
        x.set_passphrase_generator_request(get_stack_optional<PassphraseGeneratorRequest>(j, "PassphraseGeneratorRequest"));
        x.set_export_format(get_stack_optional<boost::variant<ExportFormatClass, ExportFormatEnum>>(j, "ExportFormat"));
        x.set_master_password_policy_options(get_stack_optional<MasterPasswordPolicyOptions>(j, "MasterPasswordPolicyOptions"));
        x.set_kdf(get_stack_optional<Kdf>(j, "Kdf"));
    }

    inline void to_json(json & j, const DocRef & x) {
        j = json::object();
        j["Cipher"] = x.get_cipher();
        j["CipherView"] = x.get_cipher_view();
        j["Collection"] = x.get_collection();
        j["Folder"] = x.get_folder();
        j["FolderView"] = x.get_folder_view();
        j["InitCryptoRequest"] = x.get_init_crypto_request();
        j["PasswordGeneratorRequest"] = x.get_password_generator_request();
        j["PassphraseGeneratorRequest"] = x.get_passphrase_generator_request();
        j["ExportFormat"] = x.get_export_format();
        j["MasterPasswordPolicyOptions"] = x.get_master_password_policy_options();
        j["Kdf"] = x.get_kdf();
    }

    inline void from_json(const json & j, PurpleAuthenticator& x) {
    }

    inline void to_json(json & j, const PurpleAuthenticator & x) {
        j = json::object();
    }

    inline void from_json(const json & j, PurpleDuo& x) {
        x.set_host(j.at("host").get<std::string>());
        x.set_signature(j.at("signature").get<std::string>());
    }

    inline void to_json(json & j, const PurpleDuo & x) {
        j = json::object();
        j["host"] = x.get_host();
        j["signature"] = x.get_signature();
    }

    inline void from_json(const json & j, PurpleEmail& x) {
        x.set_email(j.at("email").get<std::string>());
    }

    inline void to_json(json & j, const PurpleEmail & x) {
        j = json::object();
        j["email"] = x.get_email();
    }

    inline void from_json(const json & j, PurpleRemember& x) {
    }

    inline void to_json(json & j, const PurpleRemember & x) {
        j = json::object();
    }

    inline void from_json(const json & j, PurpleWebAuthn& x) {
    }

    inline void to_json(json & j, const PurpleWebAuthn & x) {
        j = json::object();
    }

    inline void from_json(const json & j, PurpleYubiKey& x) {
        x.set_nfc(j.at("nfc").get<bool>());
    }

    inline void to_json(json & j, const PurpleYubiKey & x) {
        j = json::object();
        j["nfc"] = x.get_nfc();
    }

    inline void from_json(const json & j, ApiKeyLoginResponseTwoFactorProviders& x) {
        x.set_authenticator(get_stack_optional<PurpleAuthenticator>(j, "authenticator"));
        x.set_duo(get_stack_optional<PurpleDuo>(j, "duo"));
        x.set_email(get_stack_optional<PurpleEmail>(j, "email"));
        x.set_organization_duo(get_stack_optional<PurpleDuo>(j, "organizationDuo"));
        x.set_remember(get_stack_optional<PurpleRemember>(j, "remember"));
        x.set_web_authn(get_stack_optional<PurpleWebAuthn>(j, "webAuthn"));
        x.set_yubi_key(get_stack_optional<PurpleYubiKey>(j, "yubiKey"));
    }

    inline void to_json(json & j, const ApiKeyLoginResponseTwoFactorProviders & x) {
        j = json::object();
        j["authenticator"] = x.get_authenticator();
        j["duo"] = x.get_duo();
        j["email"] = x.get_email();
        j["organizationDuo"] = x.get_organization_duo();
        j["remember"] = x.get_remember();
        j["webAuthn"] = x.get_web_authn();
        j["yubiKey"] = x.get_yubi_key();
    }

    inline void from_json(const json & j, ApiKeyLoginResponse& x) {
        x.set_authenticated(j.at("authenticated").get<bool>());
        x.set_force_password_reset(j.at("forcePasswordReset").get<bool>());
        x.set_reset_master_password(j.at("resetMasterPassword").get<bool>());
        x.set_two_factor(get_stack_optional<ApiKeyLoginResponseTwoFactorProviders>(j, "twoFactor"));
    }

    inline void to_json(json & j, const ApiKeyLoginResponse & x) {
        j = json::object();
        j["authenticated"] = x.get_authenticated();
        j["forcePasswordReset"] = x.get_force_password_reset();
        j["resetMasterPassword"] = x.get_reset_master_password();
        j["twoFactor"] = x.get_two_factor();
    }

    inline void from_json(const json & j, ResponseForApiKeyLoginResponse& x) {
        x.set_data(get_stack_optional<ApiKeyLoginResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForApiKeyLoginResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, FingerprintResponse& x) {
        x.set_fingerprint(j.at("fingerprint").get<std::string>());
    }

    inline void to_json(json & j, const FingerprintResponse & x) {
        j = json::object();
        j["fingerprint"] = x.get_fingerprint();
    }

    inline void from_json(const json & j, ResponseForFingerprintResponse& x) {
        x.set_data(get_stack_optional<FingerprintResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForFingerprintResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, CaptchaResponse& x) {
        x.set_site_key(j.at("siteKey").get<std::string>());
    }

    inline void to_json(json & j, const CaptchaResponse & x) {
        j = json::object();
        j["siteKey"] = x.get_site_key();
    }

    inline void from_json(const json & j, FluffyAuthenticator& x) {
    }

    inline void to_json(json & j, const FluffyAuthenticator & x) {
        j = json::object();
    }

    inline void from_json(const json & j, FluffyDuo& x) {
        x.set_host(j.at("host").get<std::string>());
        x.set_signature(j.at("signature").get<std::string>());
    }

    inline void to_json(json & j, const FluffyDuo & x) {
        j = json::object();
        j["host"] = x.get_host();
        j["signature"] = x.get_signature();
    }

    inline void from_json(const json & j, FluffyEmail& x) {
        x.set_email(j.at("email").get<std::string>());
    }

    inline void to_json(json & j, const FluffyEmail & x) {
        j = json::object();
        j["email"] = x.get_email();
    }

    inline void from_json(const json & j, FluffyRemember& x) {
    }

    inline void to_json(json & j, const FluffyRemember & x) {
        j = json::object();
    }

    inline void from_json(const json & j, FluffyWebAuthn& x) {
    }

    inline void to_json(json & j, const FluffyWebAuthn & x) {
        j = json::object();
    }

    inline void from_json(const json & j, FluffyYubiKey& x) {
        x.set_nfc(j.at("nfc").get<bool>());
    }

    inline void to_json(json & j, const FluffyYubiKey & x) {
        j = json::object();
        j["nfc"] = x.get_nfc();
    }

    inline void from_json(const json & j, PasswordLoginResponseTwoFactorProviders& x) {
        x.set_authenticator(get_stack_optional<FluffyAuthenticator>(j, "authenticator"));
        x.set_duo(get_stack_optional<FluffyDuo>(j, "duo"));
        x.set_email(get_stack_optional<FluffyEmail>(j, "email"));
        x.set_organization_duo(get_stack_optional<FluffyDuo>(j, "organizationDuo"));
        x.set_remember(get_stack_optional<FluffyRemember>(j, "remember"));
        x.set_web_authn(get_stack_optional<FluffyWebAuthn>(j, "webAuthn"));
        x.set_yubi_key(get_stack_optional<FluffyYubiKey>(j, "yubiKey"));
    }

    inline void to_json(json & j, const PasswordLoginResponseTwoFactorProviders & x) {
        j = json::object();
        j["authenticator"] = x.get_authenticator();
        j["duo"] = x.get_duo();
        j["email"] = x.get_email();
        j["organizationDuo"] = x.get_organization_duo();
        j["remember"] = x.get_remember();
        j["webAuthn"] = x.get_web_authn();
        j["yubiKey"] = x.get_yubi_key();
    }

    inline void from_json(const json & j, PasswordLoginResponse& x) {
        x.set_authenticated(j.at("authenticated").get<bool>());
        x.set_captcha(get_stack_optional<CaptchaResponse>(j, "captcha"));
        x.set_force_password_reset(j.at("forcePasswordReset").get<bool>());
        x.set_reset_master_password(j.at("resetMasterPassword").get<bool>());
        x.set_two_factor(get_stack_optional<PasswordLoginResponseTwoFactorProviders>(j, "twoFactor"));
    }

    inline void to_json(json & j, const PasswordLoginResponse & x) {
        j = json::object();
        j["authenticated"] = x.get_authenticated();
        j["captcha"] = x.get_captcha();
        j["forcePasswordReset"] = x.get_force_password_reset();
        j["resetMasterPassword"] = x.get_reset_master_password();
        j["twoFactor"] = x.get_two_factor();
    }

    inline void from_json(const json & j, ResponseForPasswordLoginResponse& x) {
        x.set_data(get_stack_optional<PasswordLoginResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForPasswordLoginResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, ProjectResponse& x) {
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_object(j.at("object").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
    }

    inline void to_json(json & j, const ProjectResponse & x) {
        j = json::object();
        j["creationDate"] = x.get_creation_date();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["object"] = x.get_object();
        j["organizationId"] = x.get_organization_id();
        j["revisionDate"] = x.get_revision_date();
    }

    inline void from_json(const json & j, ResponseForProjectResponse& x) {
        x.set_data(get_stack_optional<ProjectResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForProjectResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, ProjectDeleteResponse& x) {
        x.set_error(get_stack_optional<std::string>(j, "error"));
        x.set_id(j.at("id").get<std::string>());
    }

    inline void to_json(json & j, const ProjectDeleteResponse & x) {
        j = json::object();
        j["error"] = x.get_error();
        j["id"] = x.get_id();
    }

    inline void from_json(const json & j, ProjectsDeleteResponse& x) {
        x.set_data(j.at("data").get<std::vector<ProjectDeleteResponse>>());
    }

    inline void to_json(json & j, const ProjectsDeleteResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
    }

    inline void from_json(const json & j, ResponseForProjectsDeleteResponse& x) {
        x.set_data(get_stack_optional<ProjectsDeleteResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForProjectsDeleteResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, DatumElement& x) {
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_object(j.at("object").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_revision_date(j.at("revisionDate").get<std::string>());
    }

    inline void to_json(json & j, const DatumElement & x) {
        j = json::object();
        j["creationDate"] = x.get_creation_date();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["object"] = x.get_object();
        j["organizationId"] = x.get_organization_id();
        j["revisionDate"] = x.get_revision_date();
    }

    inline void from_json(const json & j, ProjectsResponse& x) {
        x.set_data(j.at("data").get<std::vector<DatumElement>>());
    }

    inline void to_json(json & j, const ProjectsResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
    }

    inline void from_json(const json & j, ResponseForProjectsResponse& x) {
        x.set_data(get_stack_optional<ProjectsResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForProjectsResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, SecretIdentifierResponse& x) {
        x.set_id(j.at("id").get<std::string>());
        x.set_key(j.at("key").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
    }

    inline void to_json(json & j, const SecretIdentifierResponse & x) {
        j = json::object();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["organizationId"] = x.get_organization_id();
    }

    inline void from_json(const json & j, SecretIdentifiersResponse& x) {
        x.set_data(j.at("data").get<std::vector<SecretIdentifierResponse>>());
    }

    inline void to_json(json & j, const SecretIdentifiersResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
    }

    inline void from_json(const json & j, ResponseForSecretIdentifiersResponse& x) {
        x.set_data(get_stack_optional<SecretIdentifiersResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForSecretIdentifiersResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, SecretResponse& x) {
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_id(j.at("id").get<std::string>());
        x.set_key(j.at("key").get<std::string>());
        x.set_note(j.at("note").get<std::string>());
        x.set_object(j.at("object").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_project_id(get_stack_optional<std::string>(j, "projectId"));
        x.set_revision_date(j.at("revisionDate").get<std::string>());
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const SecretResponse & x) {
        j = json::object();
        j["creationDate"] = x.get_creation_date();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["note"] = x.get_note();
        j["object"] = x.get_object();
        j["organizationId"] = x.get_organization_id();
        j["projectId"] = x.get_project_id();
        j["revisionDate"] = x.get_revision_date();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, ResponseForSecretResponse& x) {
        x.set_data(get_stack_optional<SecretResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForSecretResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, SecretDeleteResponse& x) {
        x.set_error(get_stack_optional<std::string>(j, "error"));
        x.set_id(j.at("id").get<std::string>());
    }

    inline void to_json(json & j, const SecretDeleteResponse & x) {
        j = json::object();
        j["error"] = x.get_error();
        j["id"] = x.get_id();
    }

    inline void from_json(const json & j, SecretsDeleteResponse& x) {
        x.set_data(j.at("data").get<std::vector<SecretDeleteResponse>>());
    }

    inline void to_json(json & j, const SecretsDeleteResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
    }

    inline void from_json(const json & j, ResponseForSecretsDeleteResponse& x) {
        x.set_data(get_stack_optional<SecretsDeleteResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForSecretsDeleteResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, DatumClass& x) {
        x.set_creation_date(j.at("creationDate").get<std::string>());
        x.set_id(j.at("id").get<std::string>());
        x.set_key(j.at("key").get<std::string>());
        x.set_note(j.at("note").get<std::string>());
        x.set_object(j.at("object").get<std::string>());
        x.set_organization_id(j.at("organizationId").get<std::string>());
        x.set_project_id(get_stack_optional<std::string>(j, "projectId"));
        x.set_revision_date(j.at("revisionDate").get<std::string>());
        x.set_value(j.at("value").get<std::string>());
    }

    inline void to_json(json & j, const DatumClass & x) {
        j = json::object();
        j["creationDate"] = x.get_creation_date();
        j["id"] = x.get_id();
        j["key"] = x.get_key();
        j["note"] = x.get_note();
        j["object"] = x.get_object();
        j["organizationId"] = x.get_organization_id();
        j["projectId"] = x.get_project_id();
        j["revisionDate"] = x.get_revision_date();
        j["value"] = x.get_value();
    }

    inline void from_json(const json & j, SecretsResponse& x) {
        x.set_data(j.at("data").get<std::vector<DatumClass>>());
    }

    inline void to_json(json & j, const SecretsResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
    }

    inline void from_json(const json & j, ResponseForSecretsResponse& x) {
        x.set_data(get_stack_optional<SecretsResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForSecretsResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, CipherDetailsResponse& x) {
    }

    inline void to_json(json & j, const CipherDetailsResponse & x) {
        j = json::object();
    }

    inline void from_json(const json & j, ProfileOrganizationResponse& x) {
        x.set_id(j.at("id").get<std::string>());
    }

    inline void to_json(json & j, const ProfileOrganizationResponse & x) {
        j = json::object();
        j["id"] = x.get_id();
    }

    inline void from_json(const json & j, ProfileResponse& x) {
        x.set_email(j.at("email").get<std::string>());
        x.set_id(j.at("id").get<std::string>());
        x.set_name(j.at("name").get<std::string>());
        x.set_organizations(j.at("organizations").get<std::vector<ProfileOrganizationResponse>>());
    }

    inline void to_json(json & j, const ProfileResponse & x) {
        j = json::object();
        j["email"] = x.get_email();
        j["id"] = x.get_id();
        j["name"] = x.get_name();
        j["organizations"] = x.get_organizations();
    }

    inline void from_json(const json & j, SyncResponse& x) {
        x.set_ciphers(j.at("ciphers").get<std::vector<CipherDetailsResponse>>());
        x.set_profile(j.at("profile").get<ProfileResponse>());
    }

    inline void to_json(json & j, const SyncResponse & x) {
        j = json::object();
        j["ciphers"] = x.get_ciphers();
        j["profile"] = x.get_profile();
    }

    inline void from_json(const json & j, ResponseForSyncResponse& x) {
        x.set_data(get_stack_optional<SyncResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForSyncResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, UserApiKeyResponse& x) {
        x.set_api_key(j.at("apiKey").get<std::string>());
    }

    inline void to_json(json & j, const UserApiKeyResponse & x) {
        j = json::object();
        j["apiKey"] = x.get_api_key();
    }

    inline void from_json(const json & j, ResponseForUserApiKeyResponse& x) {
        x.set_data(get_stack_optional<UserApiKeyResponse>(j, "data"));
        x.set_error_message(get_stack_optional<std::string>(j, "errorMessage"));
        x.set_success(j.at("success").get<bool>());
    }

    inline void to_json(json & j, const ResponseForUserApiKeyResponse & x) {
        j = json::object();
        j["data"] = x.get_data();
        j["errorMessage"] = x.get_error_message();
        j["success"] = x.get_success();
    }

    inline void from_json(const json & j, DeviceType & x) {
        static std::unordered_map<std::string, DeviceType> enumValues {
            {"Android", DeviceType::ANDROID},
            {"AndroidAmazon", DeviceType::ANDROID_AMAZON},
            {"ChromeBrowser", DeviceType::CHROME_BROWSER},
            {"ChromeExtension", DeviceType::CHROME_EXTENSION},
            {"EdgeBrowser", DeviceType::EDGE_BROWSER},
            {"EdgeExtension", DeviceType::EDGE_EXTENSION},
            {"FirefoxBrowser", DeviceType::FIREFOX_BROWSER},
            {"FirefoxExtension", DeviceType::FIREFOX_EXTENSION},
            {"IEBrowser", DeviceType::IE_BROWSER},
            {"iOS", DeviceType::I_OS},
            {"LinuxDesktop", DeviceType::LINUX_DESKTOP},
            {"MacOsDesktop", DeviceType::MAC_OS_DESKTOP},
            {"OperaBrowser", DeviceType::OPERA_BROWSER},
            {"OperaExtension", DeviceType::OPERA_EXTENSION},
            {"SafariBrowser", DeviceType::SAFARI_BROWSER},
            {"SafariExtension", DeviceType::SAFARI_EXTENSION},
            {"SDK", DeviceType::SDK},
            {"UnknownBrowser", DeviceType::UNKNOWN_BROWSER},
            {"UWP", DeviceType::UWP},
            {"VivaldiBrowser", DeviceType::VIVALDI_BROWSER},
            {"VivaldiExtension", DeviceType::VIVALDI_EXTENSION},
            {"WindowsDesktop", DeviceType::WINDOWS_DESKTOP},
        };
        auto iter = enumValues.find(j.get<std::string>());
        if (iter != enumValues.end()) {
            x = iter->second;
        }
    }

    inline void to_json(json & j, const DeviceType & x) {
        switch (x) {
            case DeviceType::ANDROID: j = "Android"; break;
            case DeviceType::ANDROID_AMAZON: j = "AndroidAmazon"; break;
            case DeviceType::CHROME_BROWSER: j = "ChromeBrowser"; break;
            case DeviceType::CHROME_EXTENSION: j = "ChromeExtension"; break;
            case DeviceType::EDGE_BROWSER: j = "EdgeBrowser"; break;
            case DeviceType::EDGE_EXTENSION: j = "EdgeExtension"; break;
            case DeviceType::FIREFOX_BROWSER: j = "FirefoxBrowser"; break;
            case DeviceType::FIREFOX_EXTENSION: j = "FirefoxExtension"; break;
            case DeviceType::IE_BROWSER: j = "IEBrowser"; break;
            case DeviceType::I_OS: j = "iOS"; break;
            case DeviceType::LINUX_DESKTOP: j = "LinuxDesktop"; break;
            case DeviceType::MAC_OS_DESKTOP: j = "MacOsDesktop"; break;
            case DeviceType::OPERA_BROWSER: j = "OperaBrowser"; break;
            case DeviceType::OPERA_EXTENSION: j = "OperaExtension"; break;
            case DeviceType::SAFARI_BROWSER: j = "SafariBrowser"; break;
            case DeviceType::SAFARI_EXTENSION: j = "SafariExtension"; break;
            case DeviceType::SDK: j = "SDK"; break;
            case DeviceType::UNKNOWN_BROWSER: j = "UnknownBrowser"; break;
            case DeviceType::UWP: j = "UWP"; break;
            case DeviceType::VIVALDI_BROWSER: j = "VivaldiBrowser"; break;
            case DeviceType::VIVALDI_EXTENSION: j = "VivaldiExtension"; break;
            case DeviceType::WINDOWS_DESKTOP: j = "WindowsDesktop"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, TwoFactorProvider & x) {
        if (j == "Authenticator") x = TwoFactorProvider::AUTHENTICATOR;
        else if (j == "Duo") x = TwoFactorProvider::DUO;
        else if (j == "Email") x = TwoFactorProvider::EMAIL;
        else if (j == "OrganizationDuo") x = TwoFactorProvider::ORGANIZATION_DUO;
        else if (j == "Remember") x = TwoFactorProvider::REMEMBER;
        else if (j == "U2f") x = TwoFactorProvider::U2_F;
        else if (j == "WebAuthn") x = TwoFactorProvider::WEB_AUTHN;
        else if (j == "Yubikey") x = TwoFactorProvider::YUBIKEY;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const TwoFactorProvider & x) {
        switch (x) {
            case TwoFactorProvider::AUTHENTICATOR: j = "Authenticator"; break;
            case TwoFactorProvider::DUO: j = "Duo"; break;
            case TwoFactorProvider::EMAIL: j = "Email"; break;
            case TwoFactorProvider::ORGANIZATION_DUO: j = "OrganizationDuo"; break;
            case TwoFactorProvider::REMEMBER: j = "Remember"; break;
            case TwoFactorProvider::U2_F: j = "U2f"; break;
            case TwoFactorProvider::WEB_AUTHN: j = "WebAuthn"; break;
            case TwoFactorProvider::YUBIKEY: j = "Yubikey"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, LinkedIdType & x) {
        static std::unordered_map<std::string, LinkedIdType> enumValues {
            {"Address1", LinkedIdType::ADDRESS1},
            {"Address2", LinkedIdType::ADDRESS2},
            {"Address3", LinkedIdType::ADDRESS3},
            {"Brand", LinkedIdType::BRAND},
            {"CardholderName", LinkedIdType::CARDHOLDER_NAME},
            {"City", LinkedIdType::CITY},
            {"Code", LinkedIdType::CODE},
            {"Company", LinkedIdType::COMPANY},
            {"Country", LinkedIdType::COUNTRY},
            {"Email", LinkedIdType::EMAIL},
            {"ExpMonth", LinkedIdType::EXP_MONTH},
            {"ExpYear", LinkedIdType::EXP_YEAR},
            {"FirstName", LinkedIdType::FIRST_NAME},
            {"FullName", LinkedIdType::FULL_NAME},
            {"LastName", LinkedIdType::LAST_NAME},
            {"LicenseNumber", LinkedIdType::LICENSE_NUMBER},
            {"MiddleName", LinkedIdType::MIDDLE_NAME},
            {"Number", LinkedIdType::NUMBER},
            {"PassportNumber", LinkedIdType::PASSPORT_NUMBER},
            {"Password", LinkedIdType::PASSWORD},
            {"Phone", LinkedIdType::PHONE},
            {"PostalCode", LinkedIdType::POSTAL_CODE},
            {"Ssn", LinkedIdType::SSN},
            {"State", LinkedIdType::STATE},
            {"Title", LinkedIdType::TITLE},
            {"Username", LinkedIdType::USERNAME},
        };
        auto iter = enumValues.find(j.get<std::string>());
        if (iter != enumValues.end()) {
            x = iter->second;
        }
    }

    inline void to_json(json & j, const LinkedIdType & x) {
        switch (x) {
            case LinkedIdType::ADDRESS1: j = "Address1"; break;
            case LinkedIdType::ADDRESS2: j = "Address2"; break;
            case LinkedIdType::ADDRESS3: j = "Address3"; break;
            case LinkedIdType::BRAND: j = "Brand"; break;
            case LinkedIdType::CARDHOLDER_NAME: j = "CardholderName"; break;
            case LinkedIdType::CITY: j = "City"; break;
            case LinkedIdType::CODE: j = "Code"; break;
            case LinkedIdType::COMPANY: j = "Company"; break;
            case LinkedIdType::COUNTRY: j = "Country"; break;
            case LinkedIdType::EMAIL: j = "Email"; break;
            case LinkedIdType::EXP_MONTH: j = "ExpMonth"; break;
            case LinkedIdType::EXP_YEAR: j = "ExpYear"; break;
            case LinkedIdType::FIRST_NAME: j = "FirstName"; break;
            case LinkedIdType::FULL_NAME: j = "FullName"; break;
            case LinkedIdType::LAST_NAME: j = "LastName"; break;
            case LinkedIdType::LICENSE_NUMBER: j = "LicenseNumber"; break;
            case LinkedIdType::MIDDLE_NAME: j = "MiddleName"; break;
            case LinkedIdType::NUMBER: j = "Number"; break;
            case LinkedIdType::PASSPORT_NUMBER: j = "PassportNumber"; break;
            case LinkedIdType::PASSWORD: j = "Password"; break;
            case LinkedIdType::PHONE: j = "Phone"; break;
            case LinkedIdType::POSTAL_CODE: j = "PostalCode"; break;
            case LinkedIdType::SSN: j = "Ssn"; break;
            case LinkedIdType::STATE: j = "State"; break;
            case LinkedIdType::TITLE: j = "Title"; break;
            case LinkedIdType::USERNAME: j = "Username"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, FieldType & x) {
        if (j == "Boolean") x = FieldType::BOOLEAN;
        else if (j == "Hidden") x = FieldType::HIDDEN;
        else if (j == "Linked") x = FieldType::LINKED;
        else if (j == "Text") x = FieldType::TEXT;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const FieldType & x) {
        switch (x) {
            case FieldType::BOOLEAN: j = "Boolean"; break;
            case FieldType::HIDDEN: j = "Hidden"; break;
            case FieldType::LINKED: j = "Linked"; break;
            case FieldType::TEXT: j = "Text"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, UriMatchType & x) {
        if (j == "domain") x = UriMatchType::URI_DOMAIN;
        else if (j == "exact") x = UriMatchType::EXACT;
        else if (j == "host") x = UriMatchType::HOST;
        else if (j == "never") x = UriMatchType::NEVER;
        else if (j == "regularExpression") x = UriMatchType::REGULAR_EXPRESSION;
        else if (j == "startsWith") x = UriMatchType::STARTS_WITH;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const UriMatchType & x) {
        switch (x) {
            case UriMatchType::URI_DOMAIN: j = "domain"; break;
            case UriMatchType::EXACT: j = "exact"; break;
            case UriMatchType::HOST: j = "host"; break;
            case UriMatchType::NEVER: j = "never"; break;
            case UriMatchType::REGULAR_EXPRESSION: j = "regularExpression"; break;
            case UriMatchType::STARTS_WITH: j = "startsWith"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, CipherRepromptType & x) {
        if (j == "None") x = CipherRepromptType::NONE;
        else if (j == "Password") x = CipherRepromptType::PASSWORD;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const CipherRepromptType & x) {
        switch (x) {
            case CipherRepromptType::NONE: j = "None"; break;
            case CipherRepromptType::PASSWORD: j = "Password"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, SecureNoteType & x) {
        if (j == "Generic") x = SecureNoteType::GENERIC;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const SecureNoteType & x) {
        switch (x) {
            case SecureNoteType::GENERIC: j = "Generic"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, CipherType & x) {
        if (j == "Card") x = CipherType::CARD;
        else if (j == "Identity") x = CipherType::IDENTITY;
        else if (j == "Login") x = CipherType::LOGIN;
        else if (j == "SecureNote") x = CipherType::SECURE_NOTE;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const CipherType & x) {
        switch (x) {
            case CipherType::CARD: j = "Card"; break;
            case CipherType::IDENTITY: j = "Identity"; break;
            case CipherType::LOGIN: j = "Login"; break;
            case CipherType::SECURE_NOTE: j = "SecureNote"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }

    inline void from_json(const json & j, ExportFormatEnum & x) {
        if (j == "AccountEncryptedJson") x = ExportFormatEnum::ACCOUNT_ENCRYPTED_JSON;
        else if (j == "Csv") x = ExportFormatEnum::CSV;
        else if (j == "Json") x = ExportFormatEnum::JSON;
        else { throw std::runtime_error("Input JSON does not conform to schema!"); }
    }

    inline void to_json(json & j, const ExportFormatEnum & x) {
        switch (x) {
            case ExportFormatEnum::ACCOUNT_ENCRYPTED_JSON: j = "AccountEncryptedJson"; break;
            case ExportFormatEnum::CSV: j = "Csv"; break;
            case ExportFormatEnum::JSON: j = "Json"; break;
            default: throw std::runtime_error("This should not happen");
        }
    }
}
namespace nlohmann {
    inline void adl_serializer<boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum>>::from_json(const json & j, boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum> & x) {
        if (j.is_object())
            x = j.get<quicktype::ExportFormatClass>();
        else if (j.is_string())
            x = j.get<quicktype::ExportFormatEnum>();
        else throw std::runtime_error("Could not deserialise!");
    }

    inline void adl_serializer<boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum>>::to_json(json & j, const boost::variant<quicktype::ExportFormatClass, quicktype::ExportFormatEnum> & x) {
        switch (x.which()) {
            case 0:
                j = boost::get<quicktype::ExportFormatClass>(x);
                break;
            case 1:
                j = boost::get<quicktype::ExportFormatEnum>(x);
                break;
            default: throw std::runtime_error("Input JSON does not conform to schema!");
        }
    }
}

