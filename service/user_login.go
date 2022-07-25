package service

import (
	"errors"
	"gollow/auth"
	"gollow/model"
	"gollow/serializer"
	"net/http"
)

type User struct {
	Username string `form:"username" json:"username" binding:"required,min=2,max=10"`
	Password string `form:"password" json:"password" binding:"required,min=8,max=20"`
}

func (user *User) Login () serializer.Response {
	var loginUser model.LoginUser

	if err := model.DB.Where("username = ?", user.Username).First(&loginUser.User).Error; err != nil {
		return serializer.ParamErr(err)
	}

	if loginUser.CheckPassword(user.Password) == false {
		return serializer.ParamErr(errors.New("账号或密码错误"))
	}

	return serializer.Response{
		Code: http.StatusOK,
		Data: auth.GenerateToken(loginUser.User),
	}
}
