import math

def gentri(n):
    return n*(n+1)/2


def divs(x):
    nd = 0 
    for y in range(1,int(math.sqrt(x)+1)):
        if x%y == 0:
            if y**2 == x:
                nd += 1
            else:
                nd += 2 
    return nd

def showtri(maxt):
    nt = 1
    while True:
        t = gentri(nt)
        div = divs(t)
        print nt,t, div
        if div >= maxt:
            break
        nt = nt + 1
       
