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
 * Built from #/definitions/SecretCreateRequest
 * @property string[]|array|null $projectIds IDs of the projects that this secret will belong to
 */
class SecretCreateRequest extends ClassStructure
{
    /** @var string Organization where the secret will be created */
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
        $properties->organizationId = Schema::string();
        $properties->organizationId->description = "Organization where the secret will be created";
        $properties->organizationId->format = "uuid";
        $properties->key = Schema::string();
        $properties->key->maxLength = 500;
        $properties->key->minLength = 1;
        $properties->value = Schema::string();
        $properties->value->maxLength = 25000;
        $properties->value->minLength = 1;
        $properties->note = Schema::string();
        $properties->note->maxLength = 7000;
        $properties->projectIds = (new Schema())->setType([Schema::_ARRAY, Schema::NULL]);
        $properties->projectIds->items = Schema::string();
        $properties->projectIds->items->format = "uuid";
        $properties->projectIds->description = "IDs of the projects that this secret will belong to";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->key,
            self::names()->note,
            self::names()->organizationId,
            self::names()->value,
        );
        $ownerSchema->setFromRef('#/definitions/SecretCreateRequest');
    }
}