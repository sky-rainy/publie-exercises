package model

import "golang.org/x/crypto/bcrypt"

type User struct {
	ID       uint   `gorm:"primaryKey"`
	Username string `gorm:"column:username;type:char;comment:用户名;"`
	Password string `gorm:"column:password;type:char;comment:密码;"`
	Nickname string `gorm:"column:nickname;type:char;comment:昵称;"`
	Avatar   string `gorm:"column:avatar;type:char;comment:头像;"`
}

type LoginUser struct {
	User  User
	Token string
}

// SetPassword 设置密码
func (user *User) SetPassword(password string) error {
	bytes, err := bcrypt.GenerateFromPassword([]byte(password), 12)
	if err != nil {
		return err
	}
	user.Password = string(bytes)
	return nil
}

// CheckPassword 校验密码
func (user *LoginUser) CheckPassword(password string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(user.User.Password), []byte(password))
	return err == nil
}
