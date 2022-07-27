package model

type Options struct {
	QuestionId uint `gorm:"type:int;size:1"`
	Options    uint `gorm:"type:int;size:1"`
	Right      uint `gorm:"default:0"`
}
