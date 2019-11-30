package setting

import (
	"github.com/go-ini/ini"
	"time"
)

var (
	Cfg *ini.File

	RunMode string

	HTTPPort     int
	ReadTimeout  time.Duration
	WriteTimeout time.Duration

	PageSize  int
	JwtSecret string
)
