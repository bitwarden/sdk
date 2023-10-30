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
 * Login to Bitwarden with Api Key
 * Built from #/definitions/ApiKeyLoginRequest
 */
class ApiKeyLoginRequest extends ClassStructure
{
    /** @var string Bitwarden account client_id */
    public $clientId;

    /** @var string Bitwarden account client_secret */
    public $clientSecret;

    /** @var string Bitwarden account master password */
    public $password;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->clientId = Schema::string();
        $properties->clientId->description = "Bitwarden account client_id";
        $properties->clientSecret = Schema::string();
        $properties->clientSecret->description = "Bitwarden account client_secret";
        $properties->password = Schema::string();
        $properties->password->description = "Bitwarden account master password";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "Login to Bitwarden with Api Key";
        $ownerSchema->required = array(
            self::names()->clientId,
            self::names()->clientSecret,
            self::names()->password,
        );
        $ownerSchema->setFromRef('#/definitions/ApiKeyLoginRequest');
    }
}
