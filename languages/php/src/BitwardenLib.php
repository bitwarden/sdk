<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\Command;
use FFI;


class BitwardenLib
{
    public FFI $ffi;
    public FFI\CData $handle;

    public function __construct()
    {
        $this->ffi = FFI::cdef('
            void* init(const char* param);
            char* run_command(void* c_str_ptr, void* client_ptr);
            void free_mem(void* client_ptr);',
            __DIR__ . '/bitwarden_c.dll'
        );
    }

    public function init(ClientSettings $client_settings)
    {
        $this->handle = $this->ffi->init(json_encode($client_settings->jsonSerialize()));
        return $this->handle;
    }

    public function run_command(Command $command)
    {
        return $this->ffi->run_command($command->jsonSerialize(), $this->handle);
    }

    public function free_mem()
    {
        $this->ffi->free_mem($this->handle);
    }
}
