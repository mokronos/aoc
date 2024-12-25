import z3

def solve():

    opt = z3.Solver()
    s = z3.BitVec('a', 64)

    rega = s
    regb = 0
    regc = 0
    prog = [2,4,1,3,7,5,4,0,1,3,0,3,5,5,3,0]

    for i, x in enumerate(prog):
        regb = rega % 8
        regb ^= 3
        regc = rega >> regb
        regb ^= regc
        regb ^= 3
        rega = rega >> 3
        opt.add((regb % 8)==x)

        if i != len(prog) - 1:
            opt.add(rega != 0)
        else:
            opt.add(rega == 0)

    assert str(opt.check()) == 'sat'
    return opt.model().eval(s)
    

print(solve())
