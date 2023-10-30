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
 * Login to Bitwarden with Username and Password
 * Built from #/definitions/PasswordLoginRequest
 * @property TwoFactorRequest|null $twoFactor
 */
class PasswordLoginRequest extends ClassStructure
{
    /** @var string Bitwarden account email address */
    public $email;

    /** @var string Bitwarden account master password */
    public $password;

    /** @var Kdf Kdf from prelogin */
    public $kdf;

    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->email = Schema::string();
        $properties->email->description = "Bitwarden account email address";
        $properties->password = Schema::string();
        $properties->password->description = "Bitwarden account master password";
        $properties->twoFactor = new Schema();
        $properties->twoFactor->anyOf[0] = TwoFactorRequest::schema();
        $properties->twoFactor->anyOf[1] = Schema::null();
        $properties->kdf = new Schema();
        $propertiesKdfAllOf0 = new Schema();
        $propertiesKdfAllOf0->oneOf[0] = Kdf::schema();
        $propertiesKdfAllOf0->oneOf[1] = Kdf::schema();
        $propertiesKdfAllOf0->setFromRef('#/definitions/Kdf');
        $properties->kdf->allOf[0] = $propertiesKdfAllOf0;
        $properties->kdf->description = "Kdf from prelogin";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->description = "Login to Bitwarden with Username and Password";
        $ownerSchema->required = array(
            self::names()->email,
            self::names()->kdf,
            self::names()->password,
        );
        $ownerSchema->setFromRef('#/definitions/PasswordLoginRequest');
    }
}
