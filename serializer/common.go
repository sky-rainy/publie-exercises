package serializer

import "net/http"

type Response struct {
	Code    int         `json:"code"`
	Data    interface{} `json:"data,omitempty"`
	Message string      `json:"Message,omitempty"`
}

// Err 通用错误处理
func Err(code int, message string) Response {
	return Response{
		Code: code,
		Message: message,
	}
}

// DBErr 数据库操作错误
func DBErr() Response {
	return Err(http.StatusInternalServerError, "数据库操作失败")
}

// ParamErr 参数错误
func ParamErr(err error) Response {
	return Err(http.StatusBadRequest, err.Error())
}

// NeedLogin 需要登录
func NeedLogin() Response {
	return Err(http.StatusUnauthorized, "用户需要登录")
}
