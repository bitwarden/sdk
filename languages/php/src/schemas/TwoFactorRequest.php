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
 * Built from #/definitions/TwoFactorRequest
 */
class TwoFactorRequest extends ClassStructure
{
    const AUTHENTICATOR = 'Authenticator';

    const EMAIL = 'Email';

    const DUO = 'Duo';

    const YUBIKEY = 'Yubikey';

    const U2F = 'U2f';

    const REMEMBER = 'Remember';

    const ORGANIZATION_DUO = 'OrganizationDuo';

    const WEB_AUTHN = 'WebAuthn';

    /** @var string Two-factor Token */
    public $token;

    /** @var string Two-factor provider */
    public $provider;

    /** @var bool Two-factor remember */
    public $remember;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->token = Schema::string();
        $properties->token->description = "Two-factor Token";
        $properties->provider = new Schema();
        $propertiesProviderAllOf0 = Schema::string();
        $propertiesProviderAllOf0->enum = array(
            self::AUTHENTICATOR,
            self::EMAIL,
            self::DUO,
            self::YUBIKEY,
            self::U2F,
            self::REMEMBER,
            self::ORGANIZATION_DUO,
            self::WEB_AUTHN,
        );
        $propertiesProviderAllOf0->setFromRef('#/definitions/TwoFactorProvider');
        $properties->provider->allOf[0] = $propertiesProviderAllOf0;
        $properties->provider->description = "Two-factor provider";
        $properties->remember = Schema::boolean();
        $properties->remember->description = "Two-factor remember";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->provider,
            self::names()->remember,
            self::names()->token,
        );
        $ownerSchema->setFromRef('#/definitions/TwoFactorRequest');
    }
}
