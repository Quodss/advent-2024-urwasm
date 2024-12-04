//
export function part1(input: Array<u8>): u32 {
  
  let newline: u8 = '\n'.charCodeAt(0) as u8;
  let X: u8 = 'X'.charCodeAt(0) as u8;
  let M: u8 = 'M'.charCodeAt(0) as u8;
  let A: u8 = 'A'.charCodeAt(0) as u8;
  let S: u8 = 'S'.charCodeAt(0) as u8;
  
  let size = 0;
  for (; size < input.length; size++) {
    if (input[size] == newline) break;
  }

  let count: u32 = 0;

  let i_directions: Array<i32> = [-1, 0, 1];
  let j_directions: Array<i32> = [-1, 0, 1];

  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      let idx = i * (size + 1) + j;
      if (input[idx] == X) {
        // for (const i_dir of i_directions) {
        for (let ii = 0; ii < 3; ii++) {
          for (let jj = 0; jj < 3; jj++) {
            let i_dir = i_directions[ii];
            let j_dir = j_directions[jj];
            if (i_dir != 0 || j_dir != 0) {
              let i_enough: bool = (i_dir < 0 && i >= 3)
                || (i_dir > 0 && i < size - 3)
                || (i_dir == 0);
              let j_enough: bool = (j_dir < 0 && j >= 3)
                || (j_dir > 0 && j < size - 3)
                || (j_dir == 0);
              if (i_enough && j_enough) {
                let idx_m = (i + 1*i_dir) * (size + 1) + j + 1*j_dir;
                let idx_a = (i + 2*i_dir) * (size + 1) + j + 2*j_dir;
                let idx_s = (i + 3*i_dir) * (size + 1) + j + 3*j_dir;
                
                let isM: bool = input[idx_m] == M;
                let isA: bool = input[idx_a] == A;
                let isS: bool = input[idx_s] == S;

                if (isM && isA && isS) {
                  count += 1;
                }
              }
            }
          }
        }
      }
    }
  }
  return count;
}


export function part2(input: Array<u8>): u32 {

  let newline: u8 = '\n'.charCodeAt(0) as u8;
  let X: u8 = 'X'.charCodeAt(0) as u8;
  let M: u8 = 'M'.charCodeAt(0) as u8;
  let A: u8 = 'A'.charCodeAt(0) as u8;
  let S: u8 = 'S'.charCodeAt(0) as u8;

  let size = 0;
  for (; size < input.length; size++) {
    if (input[size] == newline) break;
  }

  let count: u32 = 0;

  for (let i = 1; i < size - 1; i++) {
    for (let j = 1; j < size - 1; j++) {
      let idx = i * (size + 1) + j;
      if (input[idx] == A) {
        let idx_lu = (i - 1) * (size + 1) + (j - 1);
        let idx_ld = (i + 1) * (size + 1) + (j - 1);
        let idx_ru = (i - 1) * (size + 1) + (j + 1);
        let idx_rd = (i + 1) * (size + 1) + (j + 1);

        let diag1: bool = ( (input[idx_lu] == M) && (input[idx_rd] == S) 
                         || (input[idx_lu] == S) && (input[idx_rd] == M) );

        let diag2: bool = ( (input[idx_ru] == M) && (input[idx_ld] == S) 
                         || (input[idx_ru] == S) && (input[idx_ld] == M) );
        
        if (diag1 && diag2) {
          count += 1;
        }
      }
    }
  }

  return count;
}