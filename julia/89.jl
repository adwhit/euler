
valc = "MDCLXVI"
valn = [1000,500,100,50,10,5,1]


cton = Dict{Char, Int}()
ntoc = Dict{Int, Char}()
for (c,n) in zip(valc, valn)
    cton[c] = n
    ntoc[n] = c
end

function tolat(r::String)
    lat = Int[]
    bd = length(r)
    for (i,c) in enumerate(r)
        if (i < bd) && (cton[r[i+1]] > cton[c])
            push!(lat,-cton[c])
        else
            push!(lat,cton[c])
        end
    end
    return sum(lat)
end

function torom(N::Int)
    rarr = Char[]
    for (j,n) in enumerate(valn)
        num = int(floor(N/n))
        if num == 4 && j != 1
            push!(rarr,ntoc[n])
            push!(rarr,ntoc[valn[j-1]])
        else
            for i=1:num
                push!(rarr,ntoc[n])
            end
        end
        N = N % n
        if string(N)[1] == '9' && string(N/n)[3] == '9'
            #the annoying situation
            push!(rarr,ntoc[valn[j+2]])
            push!(rarr, ntoc[n])
            N = N - n + valn[j+2]
        end
    end
    return join(rarr)
end

function main()
    arr = readcsv("../data/roman.txt")
    nlines = length(arr)
    dff = 0
    for i in 1:nlines
        s = arr[i]
        n = tolat(s)
        fx = torom(n)
       # println(s)
       # println(n)
       # println(fx)
       # println()
        dff += length(s) - length(fx)
    end
    println(dff)
end

