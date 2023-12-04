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
 * Built from #/definitions/SecretPutRequest
 * @property string[]|array|null $projectIds
 */
class SecretPutRequest extends ClassStructure
{
    /** @var string ID of the secret to modify */
    public $id;

    /** @var string Organization ID of the secret to modify */
    public $organizationId;

    /** @var string */
    public $key;

    /** @var string */
    public $value;

    /** @var string */
    public $note;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->id = Schema::string();
        $properties->id->description = "ID of the secret to modify";
        $properties->id->format = "uuid";
        $properties->organizationId = Schema::string();
        $properties->organizationId->description = "Organization ID of the secret to modify";
        $properties->organizationId->format = "uuid";
        $properties->key = Schema::string();
        $properties->value = Schema::string();
        $properties->note = Schema::string();
        $properties->projectIds = (new Schema())->setType([Schema::_ARRAY, Schema::NULL]);
        $properties->projectIds->items = Schema::string();
        $properties->projectIds->items->format = "uuid";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->id,
            self::names()->key,
            self::names()->note,
            self::names()->organizationId,
            self::names()->value,
        );
        $ownerSchema->setFromRef('#/definitions/SecretPutRequest');
    }
}
