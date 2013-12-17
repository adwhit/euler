import numpy as np

def conv(s):
    a = np.array([map(int,row.split(' ')) for row in s.split('\n')])
    return a 

def getnext(i,j,dx):
    if dx == 0:
        return i+1,j,a[i+1][j]
    elif dx == 1: 
        return i+1,j+1,a[i+1][j+1]

def route(ar):
    tot = 75
    i=0
    j=0
    for dr in ar:
        i,j,add = getnext(i,j,dr)
        tot += add
    return tot

def genroute(ar):
    while True:
        changed = False
        for idx,b in enumerate(ar):
            if b == 0:
                ar[idx] = 1
                ar[:idx] = 0
                changed = True
                break
        if not changed:
            break
        yield ar


s = '''75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23'''

a = conv(s)
big = 0 
rt = np.zeros(14).astype(int)

for x in genroute(rt):
    rout = route(x)
    if rout > big:
        big = rout
        print big, x
        win = x.copy()
