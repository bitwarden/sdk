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
 * Login with Secrets Manager Access Token
 *
 * This command is for initiating an authentication handshake with Bitwarden.
 *
 * Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
 */
class Command extends ClassStructure
{
    /** @var AccessTokenLoginRequest Login to Bitwarden with access token */
    public $loginAccessToken;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->loginAccessToken = AccessTokenLoginRequest::schema();
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "Login with Secrets Manager Access Token\n\nThis command is for initiating an authentication handshake with Bitwarden.\n\nReturns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)";
        $ownerSchema->required = array(
            self::names()->loginAccessToken,
        );
    }
}
