
c = list(''.join([str(x) for x in list(range(1000000))]))
print len(c)
r = [c[1],c[10],c[100],c[1000],c[10000],c[100000],c[1000000]]
r = [int(x) for x in r]

print reduce(lambda x,y:x*y,r)


