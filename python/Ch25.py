def genfib(l):
    p=[1,1]
    while True:
        p.append(p[-1]+p[-2])
        if len(str(p[-1])) >= l:
            print len(p)
            break
