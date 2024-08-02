<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
//use Swaggest\JsonSchema\Structure\ClassStructure;


/**
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the secrets whose IDs match the provided ones
 *
 * Returns: [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
 */
class SecretsCommand extends BitwardenClassStructure
{
    public ?\stdClass $delete;

    public ?\stdClass $get;

    public ?\stdClass $getByIds;

    public ?\stdClass $list;

    public ?\stdClass $create;

    public ?\stdClass $put;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->delete = SecretsDeleteRequest::schema() ? SecretsDeleteRequest::schema() : null;
        $properties->getByIds = SecretsGetRequest::schema() ? SecretGetRequest::schema() : null;
        $properties->create = SecretCreateRequest::schema() ? SecretCreateRequest::schema() : null;
        $properties->put = SecretPutRequest::schema() ? SecretPutRequest::schema() : null;
        $properties->list = SecretIdentifiersRequest::schema() ? SecretIdentifiersRequest::schema() : null;
        $properties->get = SecretsGetRequest::schema() ? SecretGetRequest::schema() : null;
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "> Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the secrets whose IDs match the provided ones\n\nReturns: [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)";
        $ownerSchema->oneOf = array(
            self::names()->create,
            self::names()->put,
            self::names()->list,
            self::names()->getByIds,
            self::names()->delete,
        );
    }
}
