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
 */

class SecretsSyncRequest extends ClassStructure
{

    /** @var string */
    public $organizationId;
    /** @var string */
    public $lastSyncedDate;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->organizationId = Schema::string();
        $properties->organizationId->format = "uuid";
        $properties->organizationId->description = "Organization ID";
        $properties->lastSyncedDate = Schema::string();
        $properties->lastSyncedDate->format = "date-time";
        $properties->lastSyncedDate->description = "Last synced date";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->organizationId,
        );
        $ownerSchema->setFromRef('#/definitions/SecretsSyncRequest');
    }
}
