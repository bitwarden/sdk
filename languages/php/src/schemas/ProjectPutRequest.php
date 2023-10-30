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
 * Built from #/definitions/ProjectPutRequest
 */
class ProjectPutRequest extends ClassStructure
{
    /** @var string ID of the project to modify */
    public $id;

    /** @var string Organization ID of the project to modify */
    public $organizationId;

    /** @var string */
    public $name;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->id = Schema::string();
        $properties->id->description = "ID of the project to modify";
        $properties->id->format = "uuid";
        $properties->organizationId = Schema::string();
        $properties->organizationId->description = "Organization ID of the project to modify";
        $properties->organizationId->format = "uuid";
        $properties->name = Schema::string();
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->id,
            self::names()->name,
            self::names()->organizationId,
        );
        $ownerSchema->setFromRef('#/definitions/ProjectPutRequest');
    }
}
