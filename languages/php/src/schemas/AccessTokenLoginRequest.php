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
 * Login to Bitwarden with access token
 * Built from #/definitions/AccessTokenLoginRequest
 * @property string|null $stateFile
 */
class AccessTokenLoginRequest extends ClassStructure
{
    /** @var string Bitwarden service API access token */
    public $accessToken;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->accessToken = Schema::string();
        $properties->accessToken->description = "Bitwarden service API access token";
        $properties->stateFile = (new Schema())->setType([Schema::STRING, Schema::NULL]);
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "Login to Bitwarden with access token";
        $ownerSchema->required = array(
            self::names()->accessToken,
        );
        $ownerSchema->setFromRef('#/definitions/AccessTokenLoginRequest');
    }
}
