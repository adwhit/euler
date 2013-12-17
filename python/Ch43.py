import helper as hp

def isdivis(l):
    for i,x in zip(range(1,8),div):
        k = int(''.join(map(str,l[i:i+3])))
        if k%x != 0:
            return False
    return True

    
r = []
l = list(range(10))
div = [2,3,5,7,11,13,17]
p = hp.permute(l)

for x in p:
    if isdivis(list(str(x))):
        print x
        r.append(x)
