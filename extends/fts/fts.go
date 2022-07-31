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

// go生成的proto文件中的结构体，需要使用unsafe.Pointer来转换
func createdCBuffer(data []byte) C.Buffer {
	var buffer C.Buffer
	buffer.data = (*C.uchar)(unsafe.Pointer(&data[0]))
	buf_len := len(data)
	buffer.len = C.ulonglong(buf_len)
	return buffer
}

// handleActionResult converts the C buffer to a Go ActionResult
func handleActionResult(buffer C.Buffer) (*ActionResult, error) {
	var actionResult ActionResult
	if err := proto.Unmarshal(*(*[]byte)(unsafe.Pointer(&buffer.data)), &actionResult); err != nil {
		println("proto.Unmarshal failed:", err)
		return &actionResult, err
	}
	C.free_bytes(buffer)
	return &actionResult, nil
}

func Init() C.int {
	return C.init_fts()
}

func DeleteAll() (*ActionResult, error) {
	buffer := C.delete_all()
	return handleActionResult(buffer)
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
	buffer := createdCBuffer(contents_bytes)
	buffer_rec := C.batch_add(buffer)
	return handleActionResult(buffer_rec)
}

func Query(query string) (*ActionResult, error) {
	cquery := C.CString(query)
	defer C.free(unsafe.Pointer(cquery))
	buffer := C.query(cquery)
	return handleActionResult(buffer)
}
