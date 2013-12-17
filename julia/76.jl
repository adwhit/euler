cache = Dict{(Int,Int), Int}()


function ways(N::Int, mx::Int)
    if (N,mx) in keys(cache)
        return cache[(N,mx)]
    end
    if N==1
        return 0
    end
    tot = 0
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

ways(100,100)
