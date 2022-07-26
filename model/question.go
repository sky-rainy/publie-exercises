package model

// Question 问题表
type Question struct {
	ID         uint   `gorm:"column:id; primaryKey; comment:题目ID"`
	Title      string `gorm:"column:title; size:250; comment:题目名称"`
	Content    string `gorm:"column:content; size:500; comment:题目内容"`
	Qtype      uint   `gorm:"column:q_type; size:7; default:0; comment:题目类型;"`
	Status     uint   `gorm:"column:status; size:7; default:0; comment:题目状态;"`
	Delete     uint   `gorm:"column:delete; size:7; default:0; comment:题目删除状态;"`
	CreateTime int    `gorm:"column:create_time; comment:创建时间;"`
}
