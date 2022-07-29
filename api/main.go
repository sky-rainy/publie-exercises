package api

import (
	"gollow/extends/fts"
	"gollow/serializer"
	"net/http"

	"github.com/gin-gonic/gin"
)

func Ping(c *gin.Context) {
	c.JSON(200, serializer.Response{Code: 0, Message: "Pong"})
}
func Delete_All(c *gin.Context) {
	data, err := fts.DeleteAll()
	if err == nil {
		// println(data)
		c.JSON(http.StatusOK, serializer.Response{
			Code: http.StatusOK,
			Data: data,
		})
	} else {
		c.JSON(http.StatusOK, serializer.Response{
			Code: http.StatusBadRequest,
			Data: err,
		})
	}
}

func Query(c *gin.Context) {
	data, err := fts.Query("123")
	if err == nil {
		// println(data)
		c.JSON(http.StatusOK, serializer.Response{
			Code: http.StatusOK,
			Data: data,
		})
	} else {
		c.JSON(http.StatusOK, serializer.Response{
			Code: http.StatusBadRequest,
			Data: err,
		})
	}
}

type TContents struct {
	Contents []fts.Contents `json:"contents"`
}

func BatchAdd(c *gin.Context) {
	var contents TContents
	if err := c.ShouldBind(&contents); err == nil {
		data, err := fts.BatchAdd(contents.Contents)
		if err == nil {
			// println(data)
			c.JSON(http.StatusOK, serializer.Response{
				Code: http.StatusOK,
				Data: data,
			})
		} else {
			c.JSON(http.StatusOK, serializer.Response{
				Code: http.StatusBadRequest,
				Data: err,
			})
		}
	} else {
		c.JSON(http.StatusOK, serializer.ParamErr(err))
	}
}
