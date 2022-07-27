package model

import "gorm.io/gorm"

// Topic 问题表
type Topic struct {
	gorm.Model
	Title   string `gorm:"column:title; size:250; comment:题目名称"`
	Content string `gorm:"column:content; size:500; comment:题目内容"`
	Img     string `gorm:"column:img; size:500; comment:题目图片内容"`
	Type    uint   `gorm:"column:type; type:int; size:7; default:0; comment:题目类型;"`
	Status  uint   `gorm:"column:status; type:int; size:7; default:0; comment:题目状态;"`
}
