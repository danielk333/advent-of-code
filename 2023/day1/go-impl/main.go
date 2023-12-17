package main

import (
    "fmt"
    "os"
    "strings"
)

func main() {
    var first_num rune = '\x00';
    var last_num rune;
    var total int = 0;

    data, _ := os.ReadFile("../input")
    str_data := string(data[:])
    lines := strings.Split(str_data, "\n")
    for i := 0; i < len(lines); i++ {
        line := strings.TrimRight(lines[i], "\n")
        if len(line) == 0 {
            continue
        }
        for _, char := range line {
            if char >= '0' && char <= '9' {
                if first_num == '\x00' {
                    first_num = char
                }
                last_num = char
            }
        }
        total += int(first_num - '0')*10 + int(last_num - '0')
        first_num = '\x00'
    }
    fmt.Println(total)
}