package conf

import (
	"github.com/joho/godotenv"
	"gollow/model"
	"gollow/utils"
	"os"
)

func Init(){
	// 设置日志级别
	utils.BuildLogger(os.Getenv("LOG_LEVEL"))

	err := godotenv.Load()
	if err != nil {
		utils.Log().Error("env file: %v", err)
		panic(err)
	}

	// 连接数据库
	model.Database(os.Getenv("MYSQL"))
	model.Redis()
}
