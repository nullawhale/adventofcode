def calc(f):
    return int(f) // 3 - 2

def first_start():
    sum = 0
    with open('../../input/2019/day1.txt') as f:
        for line in f:
            sum = sum + calc(line)

    return sum

def second_start():
    sum = 0
    with open('../../input/2019/day1.txt') as f:
        for line in f:
            fuel = calc(line)
            while fuel > 0:
                sum += fuel;
                fuel = calc(fuel)

    return sum

print(first_start())
print(second_start())
