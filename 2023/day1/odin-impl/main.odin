package main

import "core:fmt"
import "core:os"
import "core:strings"
import "core:unicode/utf8"
import "core:math"

string_to_int :: proc(str: string) -> int {
	n: int = 0
	mult: int = 1
	for i := 0; i < len(str) - 1; i += 1 {
		mult *= 10
	}
	// fmt.println("STRINT: ", str)
	for r in str {
		// fmt.println("r ", r, ", int ", i32(r) - 48, ", mult ", mult)
		n += (int(r) - 48)*mult
		mult /= 10
	}
	// fmt.println("INT: ", n)
	return n
}

process_data :: proc(path: string) {
	data, ok := os.read_entire_file(path, context.allocator)
	defer delete(data, context.allocator)
	total: int = 0
	it := string(data)
	for line in strings.split_lines_iterator(&it) {
		num := find_number(line)
		total += num
		// fmt.println(line, num)
	}
	fmt.println(total)
}

find_number :: proc(line: string) -> int {
	str_nums: [dynamic]string
	// fmt.println("LINE: ", line)
	for ch in line {
		if int(ch) >= 48 && int(ch) <= 58 {
			append(&str_nums, utf8.runes_to_string({ch}))
		}
	}
	// fmt.println("NUMS: ", str_nums)
	
	joind_nums := strings.join({str_nums[0], str_nums[len(str_nums)-1]}, "")
	return string_to_int(joind_nums)
}


main :: proc() {
	process_data("../input")
}
