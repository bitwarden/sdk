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
 * Built from #/definitions/SecretsSyncRequest
 * @property string|null $lastSyncedDate Optional date time a sync last occurred
 */
class SecretsSyncRequest extends ClassStructure
{
    /** @var string Organization to sync secrets from */
    public $organizationId;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->organizationId = Schema::string();
        $properties->organizationId->description = "Organization to sync secrets from";
        $properties->organizationId->format = "uuid";
        $properties->lastSyncedDate = (new Schema())->setType([Schema::STRING, Schema::NULL]);
        $properties->lastSyncedDate->description = "Optional date time a sync last occurred";
        $properties->lastSyncedDate->format = "date-time";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->organizationId,
        );
        $ownerSchema->setFromRef('#/definitions/SecretsSyncRequest');
    }
}
