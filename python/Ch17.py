d = {
        0:'',
        1:'one',
        2:'two',
        3:'three',
        4:'four',
        5:'five',
        6:'six',
        7:'seven',
        8:'eight',
        9:'nine',
        10:'ten',
        11:'eleven',
        12:'twelve',
        13:'thirteen',
        14:'fourteen',
        15:'fifteen',
        16:'sixteen',
        17:'seventeen',
        18:'eighteen',
        19:'nineteen',
        20:'twenty',
        30:'thirty',
        40:'forty',
        50:'fifty',
        60:'sixty',
        70:'seventy',
        80:'eighty',
        90:'ninety'
    }

def givelen(n):
    numstr = ''
    track = 0
    s = str(n)
    s = [int(x) for x in list(s.zfill(3))]
    for pos,num in enumerate(s):
        if pos == 0 and num > 0:
            numstr = d[num] + 'hundred'
            if s[1] != 0 or s[2] != 0:
                numstr= numstr+'and'
        if pos == 1:
            if num != 1:
                numstr = numstr + d[num*10] 
            else:
                numstr = numstr + d[num*10 + s[2]]
                s[2] = 0
        if pos == 2:
            numstr = numstr + d[num]
    print numstr
    return len(numstr)

def sumlen(top):
    big = 0
    for x in range(1,top+1):
        big += givelen(x)
        print x
    return big
        


