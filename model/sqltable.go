package model

// QuestionBank 问题表
type QuestionBank struct {
	ID           uint   `gorm:"primaryKey;column:id;comment:题目ID"`
	Title        string `gorm:"type:varchar(100);column:title;comment:题目名称"`
	Pcontent     string `gorm:"type:varchar(550);column:pcontent;comment:题目内容"`
	Ptype        uint   `gorm:"type:tinyint(2);column:ptype;comment:题目类型;default:0"`
	Pstatus      uint   `gorm:"type:tinyint(2);column:pstatus;comment:题目状态;default:0"`
	Pdelete      uint   `gorm:"type:tinyint(2);column:pdelete;comment:题目删除状态;default:0"`
	CreationTime int    `gorm:"column:creationtime;comment:创建时间;"`
}

// AnswerList 答案表
type AnswerList struct {
	ID       uint   `gorm:"primaryKey;column:id;comment:答案ID"`
	Acontent string `gorm:"type:varchar(600);column:acontent;comment:答案内容"`
	Atype    uint   `gorm:"type:tinyint(2);column:atype;comment:答案类型默认0选择2内容+图片3图片;default:0"`
	QID      int    `gorm:"column:qid;comment:题目ID"`
	Adelete  uint   `gorm:"site:2;column:adelete;comment:题目删除状态;default:0"`
}
