package main

import (
	"fmt"
	"github.com/hsmtkk/automatic-giggle/gigasecond"
	"time"
)

func main() {
	t := gigasecond.AfterGigaSecond(time.Now())
	fmt.Println(t)
}
