<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\JsonSchema;
use Swaggest\JsonSchema\Schema;


class Command extends BitwardenClassStructure
{
    /** @var ProjectsCommand|null */
    public $projects;

    /** @var SecretsCommand|null */
    public $secrets;

    /** @var AccessTokenLoginRequest|null */
    public $accessTokenLogin;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->projects = ProjectsCommand::schema();
        $properties->secrets = SecretsCommand::schema();
        $properties->accessTokenLogin = AccessTokenLoginRequest::schema();

        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;

        $ownerSchema->oneOf = array(
          self::names()->projects,
          self::names()->secrets,
          self::names()->accessTokenLogin,
        );
    }
}
