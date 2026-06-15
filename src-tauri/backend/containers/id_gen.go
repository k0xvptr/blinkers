package containers

import (
	"github.com/thanhpk/randstr"
)

func genTaskID() string{
	str := randstr.Hex(8)
	return str
}
