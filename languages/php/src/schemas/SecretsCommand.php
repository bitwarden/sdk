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
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the secrets whose IDs match the provided ones
 *
 * Returns: [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
 */
class SecretsCommand extends ClassStructure
{
    /** @var SecretsDeleteRequest */
    public $delete;

    /** @var SecretGetRequest */
    public $get;

    /** @var SecretsGetRequest */
    public $get_by_ids;

    /** @var SecretIdentifiersRequest */
    public $list;

    /** @var SecretCreateRequest */
    public $create;

    /** @var SecretPutRequest */
    public $put;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->delete = SecretsDeleteRequest::schema();
        $properties->get_by_ids = SecretsGetRequest::schema();
        $properties->create = SecretCreateRequest::schema();
        $properties->put = SecretPutRequest::schema();
        $properties->list = SecretIdentifiersRequest::schema();
        $properties->get = SecretsGetRequest::schema();
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "> Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the secrets whose IDs match the provided ones\n\nReturns: [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)";
        $ownerSchema->required = array(
            self::names()->delete,
        );
    }
}
