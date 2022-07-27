package model

// Question 问题表
type Question struct {
	ID         uint   `gorm:"column:id; primaryKey; comment:题目ID"`
	Title      string `gorm:"column:title; size:250; comment:题目名称"`
	Content    string `gorm:"column:content; size:500; comment:题目内容"`
	Qimg       string `gorm:"column:q_img; size:500; comment:题目内容"`
	Qtype      uint   `gorm:"column:q_type; size:7; default:0; comment:题目类型;"`
	Status     uint   `gorm:"column:status; size:7; default:0; comment:题目状态;"`
	Sort       uint   `gorm:"column:sort; size:7; comment:排序"`
	Delete     uint   `gorm:"column:delete; size:7; default:0; comment:题目删除状态;"`
	CreateTime int    `gorm:"column:create_time; comment:创建时间;"`
	V1         string `gorm:"column:v1; size:500; comment:选择题1"`
	V2         string `gorm:"column:v2; size:500; comment:选择题2"`
	V3         string `gorm:"column:v3; size:500; comment:选择题3"`
	V4         string `gorm:"column:v4; size:500; comment:选择题4"`
	V5         string `gorm:"column:v5; size:500; comment:选择题5"`
}
