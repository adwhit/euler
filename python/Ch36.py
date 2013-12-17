
def pal(n):
    r = []
    for x in range(1,n,2):
        if int(str(x)[::-1]) == x:
            b = bin(x)
            if b[:1:-1] == b[2:]:
                r.append(x)
                print x
    return r
