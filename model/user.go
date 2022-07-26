package model

import "golang.org/x/crypto/bcrypt"

type User struct {
	ID       uint `gorm:"primaryKey"`
	Username string
	Password string
	Nickname string
	Avatar   string `gorm:"size:1000"`
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
