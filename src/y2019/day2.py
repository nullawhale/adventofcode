def compute(ints):
    curPos = 0
    for i in range(len(ints)):
        if ints[curPos] == 1:
            ints[ints[curPos+3]] = ints[ints[curPos+1]] + ints[ints[curPos+2]]
            curPos += 4
        elif ints[curPos] == 2:
            ints[ints[curPos+3]] = ints[ints[curPos+1]] * ints[ints[curPos+2]]
            curPos += 4
        elif ints[curPos] == 99:
            break


    return ints[0]


def first_start():
    with open('../../input/2019/day2.txt') as f:
        ints = [int(i) for i in f.read().split(',')]
        ints[1] = 12
        ints[2] = 2

    return compute(ints)

def second_start():
    with open('../../input/2019/day2.txt') as f:
        ints = [int(i) for i in f.read().split(',')]
        res = 0
        for noun in range(0, 100):
            for verb in range(0, 100):
                _ints = [x for x in ints]
                _ints[1] = noun
                _ints[2] = verb
                preres = compute(_ints)
                if preres == 19690720:
                    res = 100 * noun + verb

    return res

print(first_start())
print(second_start())
