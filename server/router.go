package server

import (
	"gollow/api"
	"gollow/middleware"

	"github.com/gin-gonic/gin"
)

func NewRouter() *gin.Engine {
	r := gin.Default()

	// 中间件
	r.Use(middleware.Cors())

	// 路由
	v1 := r.Group("/api/v1")
	{
		v1.POST("/ping", api.Ping)

		user := v1.Group("/user")
		{
			user.POST("/login", api.Login)
			user.POST("/register", api.Register)
		}
		v1.GET("/delete_all", api.Delete_All)
		v1.GET("/query", api.Query)
		v1.POST("/batch_add", api.BatchAdd)
	}
	return r
}
