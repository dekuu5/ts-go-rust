package main

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

func getInput() string {
	return `forward 5
down 5
forward 8
up 3
down 8
forward 2`
}

func parseLines(lines string) (int, int, error) {
	a := strings.Split(lines, " ")
	amount, err := strconv.Atoi(a[1])
	if err != nil {
		return 0, 0, errors.New("can't convert to int")
	}
	dir := a[0]
	if dir == "forward" {
		return amount, 0, nil
	} else if dir == "up" {
		return 0, -amount , nil
	}
	return 0, amount, nil
}


func main() {
	lines := strings.Split(getInput(), "\n");
	xAndy :=  [2]int{}
	for _, v := range lines {
		x, y , err := parseLines(v);
		if err != nil {
			fmt.Println(err)
		}
		xAndy[0] += x
		xAndy[1] += y
	} 
	fmt.Printf("x = %v, y = %v, dir = %v\n", xAndy[0], xAndy[1], xAndy[0] * xAndy[1])

}