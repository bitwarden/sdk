<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\JsonSchema;
use Swaggest\JsonSchema\Schema;


/**
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones
 *
 * Returns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
 */
class ProjectsCommand extends BitwardenClassStructure
{
    public ?\stdClass $delete;

    public ?\stdClass $get;

    public ?\stdClass $list;

    public ?\stdClass $create;

    public ?\stdClass $update;


    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->delete = ProjectsDeleteRequest::schema() ? ProjectsDeleteRequest::schema() : null;
        $properties->get = ProjectGetRequest::schema() ? ProjectGetRequest::schema() : null;
        $properties->list = ProjectsListRequest::schema() ? ProjectsListRequest::schema() : null;
        $properties->update = ProjectPutRequest::schema() ? ProjectPutRequest::schema() : null;
        $properties->create = ProjectCreateRequest::schema() ? ProjectCreateRequest::schema() : null;
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "> Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones\n\nReturns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)";

        $ownerSchema->oneOf = array(
            self::names()->create,
            self::names()->delete,
            self::names()->get,
            self::names()->list,
            self::names()->update,
        );
    }
}
