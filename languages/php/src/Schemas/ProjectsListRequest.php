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
 * Built from #/definitions/ProjectsListRequest
 */
class ProjectsListRequest extends ClassStructure
{
    /** @var string Organization to retrieve all the projects from */
    public $organizationId;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->organizationId = Schema::string();
        $properties->organizationId->description = "Organization to retrieve all the projects from";
        $properties->organizationId->format = "uuid";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->organizationId,
        );
        $ownerSchema->setFromRef('#/definitions/ProjectsListRequest');
    }
}
