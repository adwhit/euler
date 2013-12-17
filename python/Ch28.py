res = [1]
narm = 0
print "Circle 0 :",res
while narm<=500:
    narm = (len(res)-1)/4 + 1
    ngap = narm*2
    res.append(ngap+res[-1])
    print "Circle %s: %s" %(narm,res[-4:])
print sum(res[:-1])


