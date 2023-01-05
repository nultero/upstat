package main

import (
	"fmt"
	"os"
	"strconv"
)

type sysbat struct {
	engNow, engFul string
	fullChar       rune
	thresholds     []float32
	usedChars      []rune
}

// TODO -- make the full char configurable from the poly config
func sysStruct() sysbat {
	sb := sysbat{}
	sb.engNow = "/sys/class/power_supply/BAT0/energy_now"
	sb.engFul = "/sys/class/power_supply/BAT0/energy_full"
	sb.fullChar = '⚡'
	sb.thresholds = []float32{98.9, 88.0, 70.0, 50.0, 24.0}
	sb.usedChars = []rune{' ', '', '', '', '', ''}
	return sb
}

func readFd(f string, buf []byte) (float32, error) {
	fd, err := os.Open(f)
	if err != nil {
		return 0.0, err
	}
	n, err := fd.Read(buf)
	if err != nil {
		return 0.0, err
	}
	s := string(buf[:n-1]) //slice off newline
	eng, err := strconv.ParseFloat(s, 32)
	if err != nil {
		return 0.0, err
	}
	eval := float32(eng)
	return eval, nil
}

func (sb *sysbat) getCharge() string {
	buf := make([]byte, 20)
	enow, err := readFd(sb.engNow, buf)
	if err != nil {
		polyerr(err)
		return ""
	}
	eful, err := readFd(sb.engFul, buf)
	if err != nil {
		polyerr(err)
		return ""
	}
	used := (enow / eful) * 100.0
	str := "BAT|"
	if used > sb.thresholds[0] {
		return fmt.Sprintf("%s%c", str, sb.fullChar)
	}

	for i := 1; i < 5; i++ {
		if used > sb.thresholds[i] {
			return fmt.Sprintf(
				"%s%.2f %c",
				str, used, sb.usedChars[i],
			)
		}
	}

	return fmt.Sprintf(
		"%s%.2f %c",
		str, used, sb.usedChars[5],
	)
}
