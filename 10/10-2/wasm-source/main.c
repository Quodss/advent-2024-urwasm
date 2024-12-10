typedef unsigned long long u64;
typedef unsigned long u32;
typedef unsigned char u8;


u32 count_rating(u32 x,
    u32 y,
    u32 elev,
    u8* fil,
    u32 len,
    u32 width,
    u32 height)
{
    if (elev == 9)
    {
        return 1;
    }

    u32 score = 0;

    if (x > 0 && fil[(width + 1) * y + (x - 1)] == '1' + elev)
    {
        score += count_rating(x-1, y, elev+1, fil, len, width, height);
    }

    if (x < width - 1 && fil[(width + 1) * y + (x + 1)] == '1' + elev)
    {
        score += count_rating(x+1, y, elev+1, fil, len, width, height);
    }

    if (y > 0 && fil[(width + 1) * (y - 1) + x] == '1' + elev)
    {
        score += count_rating(x, y-1, elev+1, fil, len, width, height);
    }

    if (y < height - 1 && fil[(width + 1) * (y + 1) + x] == '1' + elev)
    {
        score += count_rating(x, y+1, elev+1, fil, len, width, height);
    }

    return score;
}

u32 solve(u8* fil, u32 len)
{
    
    u32 width=0, height;

    for (;; width++)
    {
        if (fil[width] == '\n')
        {
            height = len / (width+1);
            break;
        }
    }

    u32 sum = 0;

    for (u32 x = 0; x < width; x++)
    {
        for (u32 y = 0; y < height; y++)
        {
            u32 idx = (width + 1) * y + x;
            if (fil[idx] == '0')
            {
                u32 score = count_rating(x, y, 0, fil, len, width, height);
                sum += score;
            }
        }
    }

    return sum;
}