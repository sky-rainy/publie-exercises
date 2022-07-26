package model

import (
	utils "gollow/utils"
	"log"
	"os"
	"strconv"
	"time"

	"github.com/go-redis/redis"
	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var DB *gorm.DB
var RedisClient *redis.Client

func Database(connectString string) {
	newLogger := logger.New(
		log.New(os.Stdout, "\r\n", log.LstdFlags),
		logger.Config{
			SlowThreshold:             time.Second,
			LogLevel:                  logger.Info,
			IgnoreRecordNotFoundError: true,
			Colorful:                  false,
		},
	)

	db, err := gorm.Open(mysql.Open(connectString), &gorm.Config{
		Logger: newLogger,
	})
	if err != nil {
		utils.Log().Error("mysql lost: %v", err)
		panic(err)
	}

	sqlDB, err := db.DB()
	if err != nil {
		utils.Log().Error("mysql lost: %v", err)
		panic(err)
	}

	// 设置连接池
	sqlDB.SetMaxIdleConns(10)
	sqlDB.SetMaxOpenConns(20)
	DB = db

	// 开始自动迁移模式
	go migration()
}

func Redis() {
	db, err := strconv.ParseUint(os.Getenv("REDIS_DB"), 10, 64)
	if err != nil {
		utils.Log().Error("redis lost: %v", err)
		panic(err)
	}
	client := redis.NewClient(&redis.Options{
		Addr:       os.Getenv("REDIS_ADDR"),
		Password:   os.Getenv("REDIS_PW"),
		DB:         int(db),
		MaxRetries: 1,
	})

	_, pErr := client.Ping().Result()

	if pErr != nil {
		utils.Log().Error("redis lost: %v", err)
		panic(pErr)
	}

	RedisClient = client
}
