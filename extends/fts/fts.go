package fts

/*
#cgo LDFLAGS: -L./ftslib/target/release -lfts
#include <stdlib.h>
#include <stdint.h>
#include "./ftslib/fts.h"
*/
import "C"
import (
	"unsafe"

	"google.golang.org/protobuf/proto"
)

func Init() C.int {
	return C.init_fts()
}

func DeleteAll() (*ActionResult, error) {
	buffer := C.delete_all()
	var actionResult ActionResult
	if err := proto.Unmarshal(*(*[]byte)(unsafe.Pointer(&buffer.data)), &actionResult); err != nil {
		println("proto.Unmarshal failed:", err)
		return &actionResult, err
	}
	defer C.free_bytes(buffer)
	return &actionResult, nil
}

func BatchAdd(contents []*Contents) (*ActionResult, error) {

	var actionResult ActionResult
	batch_add_contents := &BatchContents{
		Contents: contents,
	}
	contents_bytes, err := proto.Marshal(batch_add_contents)
	if err != nil {
		return &actionResult, err
	}
	var buffer C.Buffer
	buffer.data = (*C.uchar)(unsafe.Pointer(&contents_bytes[0]))
	buf_len := len(contents_bytes)
	buffer.len = C.ulonglong(buf_len)
	buffer_rec := C.batch_add(buffer)
	if err := proto.Unmarshal(*(*[]byte)(unsafe.Pointer(&buffer_rec.data)), &actionResult); err != nil {
		println("proto.Unmarshal failed:", err)
		return &actionResult, err
	}
	return &actionResult, nil
}

func Query(query string) (*ActionResult, error) {
	cquery := C.CString(query)
	defer C.free(unsafe.Pointer(cquery))
	buffer := C.query(cquery)
	var actionResult ActionResult
	if err := proto.Unmarshal(*(*[]byte)(unsafe.Pointer(&buffer.data)), &actionResult); err != nil {
		println("proto.Unmarshal failed:", err)
		return &actionResult, err
	}
	defer C.free_bytes(buffer)
	return &actionResult, nil
}
