cache = Dict{(Int,Int), BigInt}()


function ways(N::Int, mx::Int)
    if (N,mx) in keys(cache)
        return cache[(N,mx)]
    end
    if N==1
        return 0
    end
    tot = BigInt(0)
    for left=1:N-1
        right = N-left
        if left > mx  # invalid
            continue
        end
        if left >= right
            #score 
            tot += 1
        end
        newmx = minimum((left, right, mx))
        tot += ways(right, newmx)
    end
    cache[(N,mx)] = tot
    return tot
end

function main()
    n = 2
    while true
        w = ways(n,n)
        println(n," ", w -ways(n-1,n-1))
        if w % 1000000 == 0
            break
        end
        n+=1
    end
end
