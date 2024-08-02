<?php

namespace Bitwarden\Sdk\Schemas;

use Swaggest\JsonSchema\Structure\ClassStructureContract;
use Swaggest\JsonSchema\Structure\WithResolvedValue;

abstract class BitwardenClassStructure implements ClassStructureContract, WithResolvedValue
{
    use BitwardenClassStructureTrait;
}
