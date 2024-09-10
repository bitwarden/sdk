<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


class ProjectsCommand extends ClassStructure
{
    /** @var ProjectGetRequest|ProjectsListRequest|ProjectsCommandOneOf2|ProjectPutRequest|ProjectCreateRequest */
    public $projects;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->projects = new Schema();
        $properties->projects->get = ProjectGetRequest::schema();
        $properties->projects->list = ProjectsListRequest::schema();
        $properties->projects->update = ProjectPutRequest::schema();
        $properties->projects->create = ProjectCreateRequest::schema();
        $properties->projects->setFromRef('#/definitions/ProjectsCommand');
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->projects,
        );
    }
}
