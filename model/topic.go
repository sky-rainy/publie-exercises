package model

import "gorm.io/gorm"

// Topic 问题表
type Topic struct {
	gorm.Model
	Title     string `gorm:"size:250; comment:题目名称"`
	Content   string `gorm:"size:500; comment:题目内容"`
	Img       string `gorm:"size:500; comment:题目图片内容"`
	TopicType uint   `gorm:"size:7; default:0; comment:题目类型;"`
	Status    uint   `gorm:"index; size:7; default:0; comment:题目状态;"`
}
