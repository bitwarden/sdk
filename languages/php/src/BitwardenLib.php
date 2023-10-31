<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\Command;
use FFI;
use Swaggest\JsonSchema\JsonSchema;


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
            __DIR__ . '/libbitwarden_c.dylib'
        );
    }

    public function init(ClientSettings $client_settings): FFI\CData
    {
        $this->handle = $this->ffi->init(json_encode($client_settings->jsonSerialize()));
        return $this->handle;
    }

    public function run_command(Command $command): \stdClass
    {
        $encoded_json = json_encode($command->jsonSerialize());
        try {
            $result = $this->ffi->run_command($encoded_json, $this->handle);
            return json_decode(FFI::string($result));
        } catch (\FFI\Exception $e) {
            throw new \RuntimeException('Error occurred during FFI operation: ' . $e->getMessage());
        }
    }

    public function free_mem(): void
    {
        $this->ffi->free_mem($this->handle);
    }
}
