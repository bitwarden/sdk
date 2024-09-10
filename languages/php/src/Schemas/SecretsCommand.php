<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


class SecretsCommand extends ClassStructure
{
    /** @var SecretsDeleteRequest|SecretsGetRequest|SecretCreateRequest|SecretPutRequest|SecretIdentifiersRequest|SecretsGetRequest|SecretsSyncRequest */
    public $secrets;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->secrets = new Schema();
        $properties->secrets->delete = SecretsDeleteRequest::schema();
        $properties->secrets->getByIds = SecretsGetRequest::schema();
        $properties->secrets->create = SecretCreateRequest::schema();
        $properties->secrets->update = SecretPutRequest::schema();
        $properties->secrets->list = SecretIdentifiersRequest::schema();
        $properties->secrets->get = SecretsGetRequest::schema();
        $properties->secrets->sync = SecretsSyncRequest::schema();
        $properties->secrets->setFromRef('#/definitions/SecretsCommand');
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->secrets,
        );
    }
}
