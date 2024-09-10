<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


/**
 * Basic client behavior settings. These settings specify the various targets and behavior of the Bitwarden Client. They are optional and uneditable once the client is initialized.
 *
 * Defaults to
 *
 * ``` # use bitwarden_core::{ClientSettings, DeviceType}; let settings = ClientSettings { identity_url: "https://identity.bitwarden.com".to_string(), api_url: "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(), device_type: DeviceType::SDK, }; let default = ClientSettings::default(); ```
 * Built from #/definitions/ClientSettings
 */
class ClientSettings extends ClassStructure
{
    const ANDROID = 'Android';

    const I_OS = 'iOS';

    const CHROME_EXTENSION = 'ChromeExtension';

    const FIREFOX_EXTENSION = 'FirefoxExtension';

    const OPERA_EXTENSION = 'OperaExtension';

    const EDGE_EXTENSION = 'EdgeExtension';

    const WINDOWS_DESKTOP = 'WindowsDesktop';

    const MAC_OS_DESKTOP = 'MacOsDesktop';

    const LINUX_DESKTOP = 'LinuxDesktop';

    const CHROME_BROWSER = 'ChromeBrowser';

    const FIREFOX_BROWSER = 'FirefoxBrowser';

    const OPERA_BROWSER = 'OperaBrowser';

    const EDGE_BROWSER = 'EdgeBrowser';

    const IE_BROWSER = 'IEBrowser';

    const UNKNOWN_BROWSER = 'UnknownBrowser';

    const ANDROID_AMAZON = 'AndroidAmazon';

    const UWP = 'UWP';

    const SAFARI_BROWSER = 'SafariBrowser';

    const VIVALDI_BROWSER = 'VivaldiBrowser';

    const VIVALDI_EXTENSION = 'VivaldiExtension';

    const SAFARI_EXTENSION = 'SafariExtension';

    const SDK = 'SDK';

    /** @var string The identity url of the targeted Bitwarden instance. Defaults to `https://identity.bitwarden.com` */
    public $identityUrl;

    /** @var string The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com` */
    public $apiUrl;

    /** @var string The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK` */
    public $userAgent;

    /** @var string Device type to send to Bitwarden. Defaults to SDK */
    public $deviceType;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->identityUrl = Schema::string();
        $properties->identityUrl->description = "The identity url of the targeted Bitwarden instance. Defaults to `https://identity.bitwarden.com`";
        $properties->identityUrl->default = "https://identity.bitwarden.com";
        $properties->apiUrl = Schema::string();
        $properties->apiUrl->description = "The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`";
        $properties->apiUrl->default = "https://api.bitwarden.com";
        $properties->userAgent = Schema::string();
        $properties->userAgent->description = "The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`";
        $properties->userAgent->default = "Bitwarden Rust-SDK";
        $properties->deviceType = new Schema();
        $propertiesDeviceTypeAllOf0 = Schema::string();
        $propertiesDeviceTypeAllOf0->enum = array(
            self::ANDROID,
            self::I_OS,
            self::CHROME_EXTENSION,
            self::FIREFOX_EXTENSION,
            self::OPERA_EXTENSION,
            self::EDGE_EXTENSION,
            self::WINDOWS_DESKTOP,
            self::MAC_OS_DESKTOP,
            self::LINUX_DESKTOP,
            self::CHROME_BROWSER,
            self::FIREFOX_BROWSER,
            self::OPERA_BROWSER,
            self::EDGE_BROWSER,
            self::IE_BROWSER,
            self::UNKNOWN_BROWSER,
            self::ANDROID_AMAZON,
            self::UWP,
            self::SAFARI_BROWSER,
            self::VIVALDI_BROWSER,
            self::VIVALDI_EXTENSION,
            self::SAFARI_EXTENSION,
            self::SDK,
        );
        $propertiesDeviceTypeAllOf0->setFromRef('#/definitions/DeviceType');
        $properties->deviceType->allOf[0] = $propertiesDeviceTypeAllOf0;
        $properties->deviceType->description = "Device type to send to Bitwarden. Defaults to SDK";
        $properties->deviceType->default = "SDK";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "Basic client behavior settings. These settings specify the various targets and behavior of the Bitwarden Client. They are optional and uneditable once the client is initialized.\n\nDefaults to\n\n``` # use bitwarden_core::{ClientSettings, DeviceType}; let settings = ClientSettings { identity_url: \"https://identity.bitwarden.com\".to_string(), api_url: \"https://api.bitwarden.com\".to_string(), user_agent: \"Bitwarden Rust-SDK\".to_string(), device_type: DeviceType::SDK, }; let default = ClientSettings::default(); ```";
        $ownerSchema->setFromRef('#/definitions/ClientSettings');
    }
}