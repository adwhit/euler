
coins = [200,100,50,20,10,5,2,1]

def joiner(c,l):
    return [c+x for x in l]


def decompose(n,coins):
    for i,c in enumerate(coins):
        if n > c:
            if coins == [1]:
                return joiner([c],decompose(n-c,coins[i:]))
            else:
                tmp = joiner([c],decompose(n-c,coins[i:]))
                for x in decompose(n,coins[i+1:]): tmp.append(x)
                return tmp
        if n == c:
            if n>1:
                tm = decompose(n,coins[i+1:])
                tm.insert(0,[c])
                return tm 
            else:
                return [[c]]

    

