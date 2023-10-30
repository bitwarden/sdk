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
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones
 *
 * Returns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
 */
class ProjectsCommand extends ClassStructure
{
    /** @var ProjectsDeleteRequest */
    public $delete;

    /** @var ProjectGetRequest */
    public $get;

    /** @var ProjectsListRequest */
    public $list;

    /** @var ProjectCreateRequest */
    public $create;

    /** @var ProjectPutRequest */
    public $put;


    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->delete = ProjectsDeleteRequest::schema();
        $properties->get = ProjectGetRequest::schema();
        $properties->list = ProjectsListRequest::schema();
        $properties->put = ProjectPutRequest::schema();
        $properties->create = ProjectCreateRequest::schema();
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "> Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones\n\nReturns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)";
        $ownerSchema->required = array(
            self::names()->delete,
        );
    }
}
