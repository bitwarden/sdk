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
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones
 *
 * Returns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
 */
class ProjectsCommand extends ClassStructure
{
    /** @var ProjectsDeleteRequest|null */
    public ?ProjectsDeleteRequest $delete;

    /** @var ProjectGetRequest|null */
    public ?ProjectGetRequest $get;

    /** @var ProjectsListRequest|null */
    public ?ProjectsListRequest $list;

    /** @var ProjectCreateRequest|null */
    public ?ProjectCreateRequest $create;

    /** @var ProjectPutRequest|null */
    public ?ProjectPutRequest $put;


    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->delete = ProjectsDeleteRequest::schema();
        $properties->get = ProjectGetRequest::schema();
        $properties->list = ProjectsListRequest::schema();
        $properties->put = ProjectPutRequest::schema();
        $properties->create = ProjectCreateRequest::schema();
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "> Requires Authentication > Requires using an Access Token for login or calling Sync at least once Deletes all the projects whose IDs match the provided ones\n\nReturns: [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)";
        $ownerSchema->required = array(
            self::names()->delete,
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
