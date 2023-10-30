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
    /** @var ProjectsCommand|null */
    public ?ProjectsCommand $projects;

    /** @var SecretsCommand|null */
    public ?SecretsCommand $secrets;

    /** @var AccessTokenLoginRequest|null */
    public ?AccessTokenLoginRequest $access_token_request;

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
        $ownerSchema->required = array(
            self::names()->projects,
        );
    }

    public function jsonSerialize()
    {
        $result = new \stdClass();
        $schema = static::schema();
        $properties = $schema->getProperties();
        $processed = array();
        if (null !== $properties) {
            foreach ($properties->getDataKeyMap() as $propertyName => $dataName) {
                // Get uninitialized properties as null; direct access will throw error on typed properties
//                $value = isset($this->$propertyName) ? $this->$propertyName : null;
                $value = $this->$propertyName ?? null;
//                $value = $this->$propertyName;

                // Value is exported if exists.
                if (null !== $value || array_key_exists($propertyName, $this->__arrayOfData)) {
                    $result->$dataName = $value;
                    $processed[$propertyName] = true;
                    continue;
                }

                // Non-existent value is only exported if belongs to nullable property (having 'null' in type array).
                $property = $schema->getProperty($propertyName);
                if ($property instanceof Schema) {
                    $types = $property->type;
                    if ($types === Schema::NULL || (is_array($types) && in_array(Schema::NULL, $types))) {
                        $result->$dataName = $value;
                    }
                }
            }
        }
        foreach ($schema->getNestedPropertyNames() as $name) {
            /** @var ObjectItem $nested */
            $nested = $this->$name;
            if (null !== $nested) {
                foreach ((array)$nested->jsonSerialize() as $key => $value) {
                    $result->$key = $value;
                }
            }
        }

        if (!empty($this->__arrayOfData)) {
            foreach ($this->__arrayOfData as $name => $value) {
                if (!isset($processed[$name])) {
                    $result->$name = $this->{$name};
                }
            }
        }

        return $result;
    }
}
