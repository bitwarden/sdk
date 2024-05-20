package cinterface

import (
	"fmt"
	"unsafe"
)

/*
#cgo LDFLAGS: -lbitwarden_c
#cgo linux,amd64 LDFLAGS: -L ./lib/linux-x64
#cgo linux,arm64 LDFLAGS: -L ./lib/linux-arm64
#cgo darwin,amd64 LDFLAGS: -L ./lib/darwin-x64
#cgo darwin,arm64 LDFLAGS: -L ./lib/darwin-arm64
#cgo windows,amd64 LDFLAGS: -L ./lib/windows-x64
#include <stdlib.h>
typedef void* ClientPtr;
extern char* run_command(const char *command, ClientPtr client);
extern ClientPtr init(const char *clientSettings);
extern void free_mem(ClientPtr client);
*/
import "C"

type ClientPointer struct {
	Pointer C.ClientPtr
}

type BitwardenLibrary interface {
	Init(clientSettings string) (ClientPointer, error)
	FreeMem(client ClientPointer)
	RunCommand(command string, client ClientPointer) (string, error)
}

type BitwardenLibraryImpl struct{}

func NewBitwardenLibrary() BitwardenLibrary {
	return &BitwardenLibraryImpl{}
}

func (b *BitwardenLibraryImpl) Init(clientSettings string) (ClientPointer, error) {
	ptr := C.init(C.CString(clientSettings))
	if ptr == nil {
		return ClientPointer{}, fmt.Errorf("initialization failed")
	}
	return ClientPointer{Pointer: ptr}, nil
}

func (b *BitwardenLibraryImpl) FreeMem(client ClientPointer) {
	C.free_mem(client.Pointer)
}

func (b *BitwardenLibraryImpl) RunCommand(command string, client ClientPointer) (string, error) {
	cstr := C.run_command(C.CString(command), client.Pointer)
	if cstr == nil {
		return "", fmt.Errorf("run command failed")
	}
	defer C.free(unsafe.Pointer(cstr))
	return C.GoString(cstr), nil
}
