package fts

/*
#cgo LDFLAGS: -L./ftslib/target/release -lfts
#include <stdlib.h>
#include <stdint.h>
#include "./ftslib/fts.h"
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

func Init() C.int {
	return C.init_fts()
}

// 查询结果
type ActionResult struct {
	Err     string        `json:"err"`
	Success bool          `json:"success"`
	Result  []QueryResult `json:"result"`
}

// 查询结果item
type QueryResult struct {
	Id       uint64  `json:"id"`
	Title    string  `json:"title"`
	Contents string  `json:"contents"`
	Score    float32 `json:"score"`
}

// 内容搜索
type Contents struct {
	Id       uint64   `json:"id" binding:"required"`
	Title    string   `json:"title" binding:"required"`
	Contents []string `json:"contents" binding:"required"`
}

func DeleteAll() (ActionResult, error) {
	o := C.delete_all()
	r := C.GoString(o)
	C.free_cstring(o)
	var actionResult ActionResult
	if err := json.Unmarshal([]byte(r), &actionResult); err != nil {
		fmt.Println("json.Unmarshal failed:", err)
		return actionResult, err
	}
	fmt.Printf("result:%+v\n", actionResult)
	return actionResult, nil
}

func BatchAdd(contents []Contents) (ActionResult, error) {
	var actionResult ActionResult
	contents_go_str, err := json.Marshal(contents)
	if err != nil {
		return actionResult, err
	}
	// 1 query to CString
	cupdate := C.CString(string(contents_go_str))
	defer C.free(unsafe.Pointer(cupdate))
	o := C.batch_add(cupdate)
	r := C.GoString(o)
	// 4 rust free CString
	C.free_cstring(o)
	if err := json.Unmarshal([]byte(r), &actionResult); err != nil {
		fmt.Println("json.Unmarshal failed:", err)
		return actionResult, err
	}
	fmt.Printf("result:%+v\n", actionResult)

	return actionResult, nil
}

func Query(query string) (ActionResult, error) {
	// 1 query to CString
	cquery := C.CString(query)
	defer C.free(unsafe.Pointer(cquery))
	// 2 to_string
	o := C.query(cquery)
	// 3 rust_string to go_string
	r := C.GoString(o)
	// 4 rust free CString
	C.free_cstring(o)

	var actionResult ActionResult
	if err := json.Unmarshal([]byte(r), &actionResult); err != nil {
		fmt.Println("json.Unmarshal failed:", err)
		return actionResult, err
	}
	fmt.Printf("result:%+v\n", actionResult)

	return actionResult, nil
}
