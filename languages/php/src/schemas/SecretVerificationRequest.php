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
 * Built from #/definitions/SecretVerificationRequest
 * @property string|null $masterPassword The user's master password to use for user verification. If supplied, this will be used for verification purposes.
 * @property string|null $otp Alternate user verification method through OTP. This is provided for users who have no master password due to use of Customer Managed Encryption. Must be present and valid if master_password is absent.
 */
class SecretVerificationRequest extends ClassStructure
{
    /**
     * @param Properties|static $properties
     * @param Schema $ownerSchema
     */
    public static function setUpProperties($properties, Schema $ownerSchema)
    {
        $properties->masterPassword = (new Schema())->setType([Schema::STRING, Schema::NULL]);
        $properties->masterPassword->description = "The user's master password to use for user verification. If supplied, this will be used for verification purposes.";
        $properties->otp = (new Schema())->setType([Schema::STRING, Schema::NULL]);
        $properties->otp->description = "Alternate user verification method through OTP. This is provided for users who have no master password due to use of Customer Managed Encryption. Must be present and valid if master_password is absent.";
        $ownerSchema->type = Schema::OBJECT;
        $ownerSchema->additionalProperties = false;
        $ownerSchema->setFromRef('#/definitions/SecretVerificationRequest');
    }
}
