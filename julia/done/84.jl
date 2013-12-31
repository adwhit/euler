ch = randperm(16)
cc = randperm(16)

function chance(pos::Int)
    v = shift!(ch)
    push!(ch,v)
    if v == 1
        #go
        pos = 1
    elseif v == 2
        #jail
        pos = 11
    elseif v == 3
        #C1
        pos = 12
    elseif v == 4
        #E3
        pos = 25 
    elseif v == 5
        #H2
        pos = 40
    elseif v == 6
        #R1
        pos = 6
    elseif v == 7 || v == 8
        #next RR
        if pos == 8
            pos = 16
        elseif pos == 23
            pos = 26
        elseif pos == 37
            pos = 6
        end
    elseif v == 9
        #next utility
        if pos == 8 || pos == 37
            pos = 13
        elseif pos == 23
            pos = 29
        end
    elseif v == 10
        #back 3
        pos -= 3
    end
    return pos
end

function cchest(pos::Int)
    v = shift!(cc)
    push!(cc,v)
    if v == 1
        #Go!
        pos = 1
    elseif v == 2
        #jail!
        pos = 11
    end
    return pos
end

function main()
    niter = 10000000
    dice = 4
    ct = zeros(Int,40)
    doubles = 0 

    pos = 1
    for i = 1:niter
        d1 = rand(1:dice)
        d2 = rand(1:dice)

        if d1 == d2
            #double!
            doubles += 1
        else
            doubles = 0
        end

        if doubles == 3
            #jail
            pos = 11
        else
            #move
            pos += d1 + d2

            if pos in [8, 23, 37]
                #chance!
                pos = chance(pos)
            elseif pos in [3,18,34]
                #community chest!
                pos = cchest(pos)
            elseif pos == 31
                #go to jail!
                pos = 11
            end
        end
        #save
        #println("iter: ", i, ", pos: ",pos)
        if pos > 40
            pos = pos % 40
        end
        ct[pos] += 1
    end
    freq = ct / niter
    for v in sortperm(freq, rev=true)
        println(v-1,": ", freq[v]*100)
    end
end
