import helper as hp

p = set(hp.primes(10000))
p = [x for x in p if x > 999]

for pr in sorted(p):
    flag = False
    r = []
    gp = hp.permute([int(x) for x in str(pr)])
    for g in gp:
        if g in p:
            r.append(g)
    if len(r) >= 2:
        dif = [n - pr for n in r]
        for num in dif:
            for num2 in dif:
                if num * 2 == num2:
                    flag = True
    if flag:
        print pr,r 





