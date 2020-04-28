def first_start():
    _range = 136818
    # range_ = 136820
    range_ = 685979
    count = 0

    for p in range(_range, range_+1):
        a = p // 100000
        b = p // 10000 % 10
        c = p // 1000 % 10
        d = p // 100 % 10
        e = p // 10 % 10
        f = p % 10

        if a <= b <= c <= d <= e <= f and (a == b < c or a < b == c < d or b < c == d < e or c < d == e < f or d < e == f):
            count += 1
    return count

def second_start():
    pass

print(first_start())
# print(second_start())
