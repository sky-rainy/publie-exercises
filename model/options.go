package model

type Options struct {
	BaseModel
	Content    string `gorm:"comment:答案内容"`
	Img        string `gorm:"comment:答案图片内容"`
	AnswerType uint   `gorm:"size:7; default:0; comment:答案类型默认0选择2内容+图片3图片;"`
	QuestionId uint   `gorm:"index: question_id_status; comment:题目ID"`
	Status     uint   `gorm:"index: question_id_status; size:7; default:0; comment: 状态 0正常 1弃用;"`
	Right      uint   `gorm:"default:0"`
}
