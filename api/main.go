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
	if data, err := fts.DeleteAll(); err == nil {
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
	query := c.DefaultQuery("query", "")
	if data, err := fts.Query(query); err == nil {
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
	Contents []*fts.Contents `json:"contents" binding:"required"`
}

func BatchAdd(c *gin.Context) {
	var contents TContents
	if err := c.ShouldBindJSON(&contents); err == nil {
		data, err := fts.BatchAdd(contents.Contents)
		if err == nil {
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
