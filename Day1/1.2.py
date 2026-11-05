import time

def exec():
    start = time.time()
    input = open('input.txt', 'r')
    res = 0
    d = 50
    for l in input.readlines():
        s = (1 if l.startswith('R') else -1)
        c = int(l[1:-1])
        newd = (d + c*s)
        q = abs(newd // 100)
        newd = newd % 100
        if (s == -1 and d == 0) or (s == 1 and newd == 0):
            q -= 1
        #print(f"instr: {l.strip()}\tprevd: {d}\tnewestd: {newd}\tq: {q}")
        res += q
        if newd == 0:
            res += 1
        d = newd

    # print(res)
    return time.time() - start

total = 0
OPERATION = 10000
for i in range(OPERATION):
    total += exec()

print(total/OPERATION)
