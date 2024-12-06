package main

// import "fmt"

//export solve
func solve(file []uint8) uint32 {
	

	width := 0
	for i := 0; i < len(file); i++ {
		if file[i] == '\n' {
			width = i
			break
		}
	}
	height := len(file) / width
	diffs := [][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0},}
	guard_start_x := 0
	guard_start_y := 0
	found := false
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			idx := (height+1) * y + x
			if file[idx] == '^' {
				guard_start_x = x
				guard_start_y = y
				found=true
				break
			}
		}
		if found {
			break
		}
	}

	guard_start_idx := (height+1) * guard_start_y + guard_start_x

	trace_up    := uint32(1) << 8
	trace_right := uint32(1) << 9
	trace_down  := uint32(1) << 10
	trace_left  := uint32(1) << 11

	var count uint32
	count = 0
	file_32 := make([]uint32, len(file))
	
	for o_y := 0; o_y < height; o_y++ {
		for o_x := 0; o_x < width; o_x++ {
			o_idx := (height+1) * o_y + o_x
			if file[o_idx] == '#' || o_idx == guard_start_idx {
				continue
			}

			for i, v := range file {
				file_32[i] = uint32(v)
			}

			file_32[o_idx] = '#'
			
			//  test for loop

			guard_x := guard_start_x
			guard_y := guard_start_y
			guard_dir := 0 // 0 - up, 1 - right, 2 - down, 3 - left
			
			x_next := guard_x + diffs[guard_dir][0]
			y_next := guard_y + diffs[guard_dir][1]
			
			next_inbound := x_next >= 0 &&
											x_next < width &&
											y_next >= 0 &&
											y_next < height
											
			guard_idx := (height+1) * guard_y + guard_x

			looped := (((file_32[guard_idx] & trace_up) != 0) && (guard_dir == 0)) ||
								(((file_32[guard_idx] & trace_right) != 0) && (guard_dir == 1)) ||
								(((file_32[guard_idx] & trace_down) != 0) && (guard_dir == 2)) ||
								(((file_32[guard_idx] & trace_left) != 0) && (guard_dir == 3))
			
			for next_inbound && !looped {
				idx_next := (height+1) * y_next + x_next
				if file_32[idx_next] == '#' {
					if guard_dir == 3 {
						guard_dir = 0
					} else {
						guard_dir += 1
					}
				} else {
					file_32[guard_idx] |= (uint32(1) << (8 + guard_dir))
					guard_x = x_next
					guard_y = y_next
				}
				x_next = guard_x + diffs[guard_dir][0]
				y_next = guard_y + diffs[guard_dir][1]

				next_inbound = x_next >= 0 &&
											 x_next < width &&
											 y_next >= 0 &&
											 y_next < height
												
				guard_idx = (height+1) * guard_y + guard_x

				looped = (((file_32[guard_idx] & trace_up) != 0) && (guard_dir == 0)) ||
								 (((file_32[guard_idx] & trace_right) != 0) && (guard_dir == 1)) ||
								 (((file_32[guard_idx] & trace_down) != 0) && (guard_dir == 2)) ||
								 (((file_32[guard_idx] & trace_left) != 0) && (guard_dir == 3))
			}
			
			if looped {
				count += 1
			}
		}
	}

	return count
}

func main() {}