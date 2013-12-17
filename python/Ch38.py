fullgp = set([1,2,3,4,5,6,7,8,9])

def ispan(n):
    s = ''
    for x in range(1,9):
        s += str(x*n)
        if len(s) == 9:
            gp = set([int(i) for i in s])
            if gp == fullgp:
                print n, s
                return True
            else: 
                return False
        elif len(s) > 9:
            return False


for x in range(100000):
    ispan(x)



