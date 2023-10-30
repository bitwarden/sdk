<?php

namespace Bitwarden\Sdk;


use Bitwarden\Sdk\Schemas\Command;
use FFI;

class CommandRunner
{
    private FFI\CData $handle;

    private BitwardenLib $bitwardenLib;

    public function __construct(BitwardenLib $bitwardenLib, $handle)
    {
        $this->bitwardenLib = $bitwardenLib;
        $this->handle = $handle;
    }

    public function run(Command $command)
    {
        $this->bitwardenLib->run_command($command, $this->handle);
    }
}
