from time import time
t = time()
n = set(range(1,10))
r = []

for x in range(1964):
    for y in range(x):
        z = x*y
        l = [int(i) for i in str(z)+str(x)+str(y)]
        if len(l) > 9: break
        if set(l) == n:
            print x,y,z
            r.append(z)
print sum(set(r))
print time() -t
