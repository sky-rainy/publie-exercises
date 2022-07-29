package api

import (
	"gollow/auth"
	"gollow/model"
	"gollow/serializer"
	"gollow/service"
	"net/http"

	"github.com/gin-gonic/gin"
)

func Login(c *gin.Context) {
	var user service.User
	if err := c.ShouldBind(&user); err == nil {
		res := user.Login()
		c.JSON(http.StatusOK, res)
	} else {
		c.JSON(http.StatusOK, serializer.ParamErr(err))
	}
}

func Register(c *gin.Context) {
	var user service.RegistryUser
	if err := c.ShouldBind(&user); err == nil {
		res := user.Register()
		c.JSON(http.StatusOK, res)
	} else {
		c.JSON(http.StatusOK, serializer.ParamErr(err))
	}
}

func Logout(c *gin.Context) {
	claims, status := c.Get("claims")
	if !status {
		c.JSON(http.StatusOK, serializer.Response{Code: http.StatusInternalServerError, Message: "该用户未登录"})
	} else {
		model.RedisClient.Del(string(claims.(auth.CustomClaims).Id))
		c.JSON(http.StatusOK, serializer.Response{Code: http.StatusOK, Data: "退出成功"})
	}
}
