import time

def exec():
    current = 50
    conto = 0
    MOD = 100
    with open('input.txt', 'r') as f_in:
        lines = f_in.readlines()
        for i, l in enumerate(lines):
            old_current = current
            segno = 1
            if l.startswith('L'):
                segno = -1
            num = int(l[1:])
            added = num // MOD
            conto += added
            savenum = num
            num %= MOD
            current += segno*num
            current %= MOD
            if current < 0:
                current += MOD
            if current == 0:
                conto += 1
            else:
                if old_current != 0:
                    diff = current - old_current
                    if diff != 0:
                        diff_sign = -1 if diff < 0 else 1
                        if diff_sign != segno:
                            conto += 1
            # print(f'OLD: {old_current}, CUR: {current}, num = {savenum}, added: {added}')
            # if i > 10:
            #     break

    # print(conto)

total = 0
OPERATION = 10000
start = time.time()
for i in range(OPERATION):
    exec()
exec_time = time.time() - start
print(exec_time)
print(exec_time/OPERATION)