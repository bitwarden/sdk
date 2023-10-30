<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


class Command extends ClassStructure
{
    /** @var ProjectsCommand */
    public ProjectsCommand $projects;

    /** @var SecretsCommand */
    public SecretsCommand $secrets;

    /** @var AccessTokenLoginRequest */
    public AccessTokenLoginRequest $access_token_request;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->projects = ProjectsCommand::schema();
        $properties->secrets = SecretsCommand::schema();
        $properties->access_token_request = AccessTokenLoginRequest::schema();
        $properties->projects->setFromRef('#/definitions/ProjectsCommand');
        $properties->secrets->setFromRef('#/definitions/SecretsCommand');
        $properties->access_token_request->setFromRef('#/definitions/AccessTokenLoginRequest');
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
//        $ownerSchema->required = array(
//            self::names()->projects,
//        );
    }
}
