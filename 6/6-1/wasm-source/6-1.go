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
  guard_dir := 0 // 0 - up, 1 - right, 2 - down, 3 - left
  diffs := [][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0},}
  guard_x := 0
  guard_y := 0
  found := false
  for y := 0; y < height; y++ {
    for x := 0; x < width; x++ {
      idx := (height+1) * y + x
      if file[idx] == '^' {
        guard_x = x
        guard_y = y
        found=true
        break
      }
    }
    if found {
      break
    }
  }

  // fmt.Println("found")

  x_next := guard_x + diffs[guard_dir][0]
  y_next := guard_y + diffs[guard_dir][1]

  for x_next >= 0 && x_next < width && y_next >= 0 && y_next < height {
    idx_next := (height+1) * y_next + x_next
    if file[idx_next] == '#' {
      if guard_dir == 3 {
        guard_dir = 0
      } else {
        guard_dir += 1
      }
    } else {
      guard_idx := (height+1) * guard_y + guard_x
      // fmt.Println("marked %d, %d\n", guard_x, guard_y)
      file[guard_idx] = '1'
      guard_x = x_next
      guard_y = y_next
    }
    x_next = guard_x + diffs[guard_dir][0]
    y_next = guard_y + diffs[guard_dir][1]
  }

  // fmt.Println("done tracing")

  var count uint32
  count = 1  //  guard doesn't mark the spot when she leaves

  for y := 0; y < height; y++ {
    for x := 0; x < width; x++ {
      idx := (height+1) * y + x
      // fmt.Println("%d, %d, %d\n", x, y, idx)
      if file[idx] == '1' {
        count += 1
      }
    }
  }

  return count
}

func main() {}