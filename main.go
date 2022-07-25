package main

import (
	"gollow/conf"
	"gollow/server"
)

func main(){
	// 初始化缓存连接及log设置
	conf.Init()

	// 装载路由
	r := server.NewRouter()
	err := r.Run(":12900")
	if err != nil {
		panic(err)
	}
}
