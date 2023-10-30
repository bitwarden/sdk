<?php
/**
 * @file ATTENTION!!! The code below was carefully crafted by a mean machine.
 * Please consider to NOT put any emotional human-generated modifications as the splendid AI will throw them away with no mercy.
 */

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Constraint\Properties;
use Swaggest\JsonSchema\Schema;
use Swaggest\JsonSchema\Structure\ClassStructure;


class KdfArgon2id extends ClassStructure
{
    /** @var int */
    public $iterations;

    /** @var int */
    public $memory;

    /** @var int */
    public $parallelism;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->iterations = Schema::integer();
        $properties->iterations->minimum = 1.0;
        $properties->iterations->format = "uint32";
        $properties->memory = Schema::integer();
        $properties->memory->minimum = 1.0;
        $properties->memory->format = "uint32";
        $properties->parallelism = Schema::integer();
        $properties->parallelism->minimum = 1.0;
        $properties->parallelism->format = "uint32";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->required = array(
            self::names()->iterations,
            self::names()->memory,
            self::names()->parallelism,
        );
    }
}
