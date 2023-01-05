package main

import (
	"fmt"
	"time"
)

func main() {
	battery := sysStruct()
	for {
		bat := battery.getCharge()
		fmt.Print("\n")
		time.Sleep(1 * time.Second)
	}
}
