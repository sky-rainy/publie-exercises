package auth

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	"gollow/model"
	"log"
	"os"
	"time"
)

var (
	TokenExpired     error  = errors.New("Token is expired")
	TokenNotValidYet error  = errors.New("Token not active yet")
	TokenMalformed   error  = errors.New("That's not even a token")
	TokenInvalid     error  = errors.New("Couldn't handle this token:")
	SignKey          string = "chen"
)

type JWT struct {
	SigningKey []byte
}

func NewJWT() *JWT {
	return &JWT{
		[]byte(SignKey),
	}
}

func (j *JWT) createToken(claims CustomClaims) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	log.Println(j.SigningKey)
	return token.SignedString(j.SigningKey)
}

func (j *JWT) ParseToken(tokenString string) (*CustomClaims, error) {
	token, err := jwt.ParseWithClaims(tokenString, &CustomClaims{}, func(token *jwt.Token) (interface{}, error) {
		return j.SigningKey, nil
	})

	if err != nil {
		if ve, ok := err.(*jwt.ValidationError); ok {
			if ve.Errors&jwt.ValidationErrorMalformed != 0 {
				return nil, TokenMalformed
			} else if ve.Errors&jwt.ValidationErrorExpired != 0 {
				// Token is expired
				return nil, TokenExpired
			} else if ve.Errors&jwt.ValidationErrorNotValidYet != 0 {
				return nil, TokenNotValidYet
			} else {
				return nil, TokenInvalid
			}
		}
	}

	if claims, ok := token.Claims.(*CustomClaims); ok && token.Valid {
		return claims, nil
	}

	return nil, TokenInvalid
}

type CustomClaims struct {
	Id		int `json:"id"`
	Name	string `json:"name"`
	jwt.StandardClaims
}

func GenerateToken(user model.User) model.LoginUser {
	j := &JWT{
		[]byte(os.Getenv("TOKEN_KEY")),
	}

	claims := CustomClaims{
		int(user.ID),
		user.Username,
		jwt.StandardClaims{
			NotBefore: int64(time.Now().Unix() - 1000),
			ExpiresAt: int64(time.Now().Add(time.Hour * 24 * 7).Unix()),
			Issuer: "chen",
		},
	}

	token, _ := j.createToken(claims)

	// token生成成功后，向redis中保存用户信息
	model.RedisClient.Set(string(user.ID), user, time.Hour * 24 * 7)
	return model.LoginUser{User: user, Token: token}
}
