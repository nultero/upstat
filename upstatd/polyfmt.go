package main

import "fmt"

func polyfmt(s, colorHex string) {

}

// Intended to just dump errs, nice decent red.
// Not newline termed.
func polyerr(err error) {
	fmt.Printf("%%{F#F54242}%s%%{F-}", err.Error())
}
