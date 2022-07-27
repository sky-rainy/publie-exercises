package model

import "gorm.io/gorm"

// Answer 答案表
type Answer struct {
	gorm.Model
	Content string `gorm:"column:content; comment:答案内容"`
	Img     string `gorm:"column:img; comment:答案图片内容"`
	Type    uint   `gorm:"column:type; type:int; size:2; default:0; comment:答案类型默认0选择2内容+图片3图片;"`
}
