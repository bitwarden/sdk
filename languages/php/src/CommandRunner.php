<?php

namespace Bitwarden\Sdk;


use Bitwarden\Sdk\Schemas\Command;
use Exception;
use stdClass;

class CommandRunner
{
    private BitwardenLib $bitwardenLib;

    public function __construct(BitwardenLib $bitwardenLib)
    {
        $this->bitwardenLib = $bitwardenLib;
    }

    /**
     * @throws Exception
     */
    public function run(Command $command): stdClass
    {
        $result = $this->bitwardenLib->run_command($command);
        if ($result->success) {
            return $result->data;
        }

        if (isset($result->errorMessage))
        {
            throw new Exception($result->errorMessage);
        }
        throw new Exception("Unknown error occurred");
    }
}
