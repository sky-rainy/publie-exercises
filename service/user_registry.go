package service

import (
	"gollow/auth"
	"gollow/model"
	"gollow/serializer"
	"net/http"
)

type RegistryUser struct {
	Nickname string `form:"nickname" json:"nickname" binding:"required,min=2,max=10"`
	Username string `form:"username" json:"username" binding:"required,min=2,max=10"`
	Password string `form:"password" json:"password" binding:"required,min=8,max=20"`
	PasswordConfirm string `form:"passwordConfirm" json:"passwordConfirm" binding:"required,min=8,max=20"`
}

func (registryUser *RegistryUser) valid() *serializer.Response {
	if registryUser.PasswordConfirm != registryUser.Password {
		return &serializer.Response{
			Code: http.StatusBadRequest,
			Message: "两次输入的密码不一致",
		}
	}

	count := int64(0)
	model.DB.Model(&model.User{}).Where("nickname = ?", registryUser.Nickname).Count(&count)
	if count > 0 {
		return &serializer.Response{
			Code: http.StatusBadRequest,
			Message: "昵称被占用",
		}
	}

	count = 0
	model.DB.Model(&model.User{}).Where("username = ?", registryUser.Username).Count(&count)
	if count > 0 {
		return &serializer.Response{
			Code: http.StatusBadRequest,
			Message: "用户名已经注册",
		}
	}
	return nil
}

func (registryUser *RegistryUser) Register () serializer.Response {
	user := model.User{
		Username: registryUser.Username,
		Nickname: registryUser.Nickname,
	}

	// 表单验证
	if err := registryUser.valid(); err != nil {
		return *err
	}

	// 加密密码
	if err := user.SetPassword(registryUser.Password); err != nil {
		return serializer.Err(http.StatusBadGateway, err.Error())
	}

	// 创建用户
	if err := model.DB.Create(&user).Error; err != nil {
		return serializer.ParamErr(err)
	}

	return serializer.Response{
		Code: http.StatusOK,
		Data: auth.GenerateToken(user),
	}
}
