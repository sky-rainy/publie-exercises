package server

import (
	"github.com/gin-gonic/gin"
	"gollow/api"
	"gollow/middleware"
)

func NewRouter() *gin.Engine {
	r := gin.Default()

	// 中间件
	r.Use(middleware.Cors())

	// 路由
	v1 :=r.Group("/api/v1")
	{
		v1.POST("/ping", api.Ping)

		user := v1.Group("/user")
		{
			user.POST("/login", api.Login)
			user.POST("/register", api.Register)
		}
	}
	return r
}
