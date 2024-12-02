var arena = [_]u8{0} ** (1 << 16);

export fn get_arena() *u8
{
    return &arena[0];
}

fn next(bytes: *[*]u8, end: [*]u8) void
{
    if (@intFromPtr(bytes.*) < @intFromPtr(end))
    {
        bytes.* += 1;
    }
}

fn read_u32(bytes: *[*]u8, end: [*]u8) u32
{
    var out: u32 = 0;
    while ( (@intFromPtr(bytes.*) < @intFromPtr(end))
            and (bytes.*[0] <= '9')
            and (bytes.*[0] >= '0') )
    {
        out = 10 * out + (bytes.*[0] - '0');
        bytes.* += 1;
    }
    return out;
}

fn safe_report(bytes: *[*]u8, end: [*]u8) bool
{
    var levels: [100]u32 = undefined;
    levels[0] = read_u32(bytes, end);
    var level_number: u32 = 1;
    while ((@intFromPtr(bytes.*) < @intFromPtr(end)) and (bytes.*[0] != '\n'))
    {
        next(bytes, end);
        levels[level_number] = read_u32(bytes, end);
        level_number += 1;
    }
    for (0..level_number) |skip|
    {
        var this_skip_safe: bool = true;
        const first =  if (skip > 0) levels[0] else levels[1];
        const second = if (skip > 1) levels[1] else levels[2];
        if ( (first == second)
            or ((first > second) and ((first - second) > 3))
            or ((first < second) and ((second - first) > 3)) )
        {
            this_skip_safe = false;
        }
        const is_ascending = (first < second);
        var prev = second;
        var idx: u32 = if (skip > 2) 2 else 3;
        while ( this_skip_safe and (idx < level_number) )
        {
            const num = levels[idx];
            if ( ((num <= prev) and (is_ascending or ((prev - num) > 3)))
              or ((num >= prev) and (!is_ascending or ((num - prev) > 3))) )
            {
                this_skip_safe = false;
            }
            prev = num;
            idx = if (skip != (idx + 1)) idx + 1 else idx + 2;
        }
        if (this_skip_safe)
        {
            return true;
        }
    }
    return false;
}


export fn solve(bytes: [*]u8, len: u32) u32
{
    const end: [*]u8 = bytes + len;
    var count: u32 = 0;
    var io_bytes: [*]u8 = bytes;
    while (@intFromPtr(io_bytes) < @intFromPtr(end))
    {
        const safe = safe_report(&io_bytes, end);
        if (safe)
        {
            count += 1;
        }
        io_bytes +=1;
    }
    return count;
}
