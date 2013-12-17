import datetime as dt

d = dt.date(1901,1,1)
den = dt.date(2000,12,31)
td = dt.timedelta(1)
dc = d
ct = 0


while True:
    if dc.day == 1:
        if dc.weekday() == 6:
            ct += 1
    if dc == den:
        break
    dc += td
