from methods import Methods

def calc_coords(wire):
    coords = {}
    x = y = 0
    steps = 0
    for w in wire:
        d = w[0]
        n = int(w[1:])

        _x = _y = 0

        if   d == 'U': _y = 1
        elif d == 'D': _y = -1
        elif d == 'R': _x = 1
        elif d == 'L': _x = -1

        for _ in range(0, n):
            x += _x
            y += _y
            steps += abs(_x) + abs(_y)
            cord = str(x) + ', ' + str(y)
            coords[cord] = steps


    return coords

def calculations():
    wire_1 = []
    wire_1 = []
    with open('../../input/2019/day3.txt') as f:
        wire_1 = f.readline().strip()
        wire_2 = f.readline().strip()

    wire_1_map = calc_coords(wire_1.split(','))
    wire_2_map = calc_coords(wire_2.split(','))

    all_coords = set(list(wire_1_map.keys())).intersection(list(wire_2_map.keys()))
    min_steps = min([wire_1_map[c] + wire_2_map[c] for c in all_coords])
    print('second star: ' + str(min_steps))

    distances = []

    for c in all_coords:
        c = c.split(',')
        distances.append(Methods.manhattan(c[0], c[1]))

    return min(distances)

print('first star: ' + str(calculations()))
