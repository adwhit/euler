for num in range(10,100):
    ns = [int(i) for i in str(num)]
    for den in range(num,100):
        ds = [int(i) for i in str(den)]
        try:
            tar = float(num)/float(den)
            b = float(ns[1])/float(ds[0])
            c = float(ns[0])/float(ds[1])
            if  b == tar:
                if ns[0] == ds[1]:
                    if not ns[0] == ns[1]:
                        print num,den
            if c == tar:
                if ns[1] == ds[0]:
                    if not ns[0] == ns[1]:
                        print num,den 
        except ZeroDivisionError:
            pass


