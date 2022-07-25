package middleware

import (
	"github.com/gin-gonic/gin"
	"gollow/auth"
	"gollow/model"
	"gollow/serializer"
	"net/http"
)

func RequireAuth () gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("token")
		if token == "" {
			c.JSON(http.StatusUnauthorized, serializer.NeedLogin())
			c.Abort()
			return
		}

		j := auth.NewJWT()
		claims, err := j.ParseToken(token)
		if err != nil {
			if err == auth.TokenExpired {
				c.JSON(http.StatusOK, serializer.Response{Code: http.StatusUnauthorized, Message: "授权已过期"})
				c.Abort()
				return
			}
			c.JSON(http.StatusOK, serializer.Response{Code: http.StatusUnauthorized, Message: err.Error()})
			c.Abort()
			return
		}
		if user := model.RedisClient.Get(string(claims.Id)); user == nil {
			c.JSON(http.StatusOK, serializer.Response{Code: http.StatusUnauthorized, Message: "授权已过期"})
			c.Abort()
			return
		}

		c.Set("claims", claims)
		c.Next()
	}
}
