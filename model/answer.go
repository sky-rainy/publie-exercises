package model

// Answer 答案表
type Answer struct {
	ID      uint   `gorm:"column:id; primaryKey; comment:答案ID"`
	Content string `gorm:"column:content; comment:答案内容"`
	Aimg    string `gorm:"column:a_img; comment:答案图片内容"`
	Atype   uint   `gorm:"column:a_type; size:2; default:0; comment:答案类型默认0选择2内容+图片3图片;"`
	QID     uint   `gorm:"column:q_id; index; comment:题目ID"`
	Sort    uint   `gorm:"column:sort; size:7; comment:排序"`
}
