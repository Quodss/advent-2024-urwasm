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

fn skip_line(bytes: *[*]u8, end: [*]u8) void
{
    while ((@intFromPtr(bytes.*) < @intFromPtr(end)) and (bytes.*[0] != '\n'))
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
    const first = read_u32(bytes, end);
    next(bytes, end);
    const second = read_u32(bytes, end);
    if ( (first == second)
        or ((first > second) and ((first - second) > 3))
        or ((first < second) and ((second - first) > 3)) )
    {
        skip_line(bytes, end);
        return false;
    }
    const is_ascending = (first < second);
    var prev = second;
    while ((@intFromPtr(bytes.*) < @intFromPtr(end)) and (bytes.*[0] != '\n'))
    {
        next(bytes, end);
        const num = read_u32(bytes, end);
        if ( ((num <= prev) and (is_ascending or ((prev - num) > 3)))
          or ((num >= prev) and (!is_ascending or ((num - prev) > 3))) )
        {
            skip_line(bytes, end);
            return false;
        }
        prev = num;
    }
    return true;
}

extern fn print(u32) void;

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
        print(io_bytes[0]);
        io_bytes +=1;
    }
    return count;
}
