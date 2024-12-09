
typedef unsigned long long u64;
typedef unsigned long u32;
typedef unsigned char u8;

void solve(u8* fil, u32 len, u64* out)
{
    u64 checksum_fil_idx = 0;

    u64 block_idx = 0;
    u64 checksum = 0;

    while (checksum_fil_idx < len)
    {
        if (!(checksum_fil_idx & 1))  // if pointing at a file
        {
            u8 n_blocks = fil[checksum_fil_idx] - (u8)'0';
            if (n_blocks & ((u8)1 << 7))  // moved file
            {
                block_idx += n_blocks & ~((u8)1 << 7);
            }
            else
            {
                while (n_blocks--)
                {
                    checksum += block_idx++ * (checksum_fil_idx >> 1);
                }
            }

            checksum_fil_idx++;
        }
        else  //  fill free space
        {
            u8 n_blocks_free = fil[checksum_fil_idx] - (u8)'0';

            //  if len is odd, last digit is a file, else it's free space
            //
            u64 pack_fil_idx = (len & 1) ? len - 1 : len - 2;

            while (pack_fil_idx > checksum_fil_idx && n_blocks_free)
            {
                u8 n_blocks_move = fil[pack_fil_idx] - (u8)'0';
                if (n_blocks_move > n_blocks_free || n_blocks_move == 0)
                {
                    pack_fil_idx -= 2;
                }
                else
                {
                    while (n_blocks_move--)
                    {
                        checksum += block_idx++ * (pack_fil_idx >> 1);
                        n_blocks_free--;
                    }
                    fil[pack_fil_idx] |= ((u8)1 << 7);
                }
            }

            block_idx += n_blocks_free;
            checksum_fil_idx++;
        }
    }

    
    *out = checksum;
}