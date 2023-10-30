<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\SDK;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


/**
 * Built from #/definitions/FingerprintRequest
 */
class FingerprintRequest extends ClassStructure
{
    /** @var string The input material, used in the fingerprint generation process. */
    public $fingerprintMaterial;

    /** @var string The user's public key encoded with base64. */
    public $publicKey;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->fingerprintMaterial = Schema::string();
        $properties->fingerprintMaterial->description = "The input material, used in the fingerprint generation process.";
        $properties->publicKey = Schema::string();
        $properties->publicKey->description = "The user's public key encoded with base64.";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->fingerprintMaterial,
            self::names()->publicKey,
        );
        $ownerSchema->setFromRef('#/definitions/FingerprintRequest');
    }
}