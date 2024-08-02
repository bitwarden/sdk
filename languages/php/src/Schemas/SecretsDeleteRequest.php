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
 * Built from #/definitions/SecretsDeleteRequest
 */
class SecretsDeleteRequest extends ClassStructure
{
    /** @var string[]|array IDs of the secrets to delete */
    public $ids;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->ids = Schema::arr();
        $properties->ids->items = Schema::string();
        $properties->ids->items->format = "uuid";
        $properties->ids->description = "IDs of the secrets to delete";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->ids,
        );
        $ownerSchema->setFromRef('#/definitions/SecretsDeleteRequest');
    }
}
