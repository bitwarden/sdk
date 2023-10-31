<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\Command;
use FFI;
use Swaggest\JsonDiff\Exception;
use Swaggest\JsonSchema\JsonSchema;


class BitwardenLib
{
    public FFI $ffi;
    public FFI\CData $handle;

    /**
     * @throws \Exception
     */
    public function __construct()
    {
        $lib_file = null;

        if (PHP_OS === 'WINNT') {
            $lib_file = '/lib/windows-x64/bitwarden_c.dll';
            if (file_exists($lib_file) == false) {
                $lib_file = __DIR__.'../../../../target/debug/bitwarden_c.dll';
            }
        } elseif (PHP_OS === 'Linux') {
            $lib_file = '/lib/ubuntu-x64/libbitwarden_c.so';
            if (file_exists($lib_file) == false) {
                $lib_file = __DIR__.'../../../../target/debug/libbitwarden_c.so';
            }
        } elseif (PHP_OS === 'Darwin') {
            $architecture = trim(exec('uname -m'));
            if ($architecture === 'x86_64' || $architecture === 'amd64') {
                $lib_file = __DIR__.'/lib/macos-x64/libbitwarden_c.dylib';
            } elseif ($architecture === 'arm64') {
                $lib_file = __DIR__.'/lib/macos-arm64/libbitwarden_c.dylib';
            }
            if (file_exists($lib_file) == false) {
                $lib_file = __DIR__.'../../../../target/debug/libbitwarden_c.dylib';
            }
        }

        if ($lib_file == null || is_file($lib_file) == false) {
            throw new \Exception("Lib file not found");
        }

        $this->ffi = FFI::cdef('
            void* init(const char* param);
            char* run_command(void* c_str_ptr, void* client_ptr);
            void free_mem(void* client_ptr);',
            $lib_file
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
