include("tools.jl")

seen = Dict{(Int,Int,Int,Int),Bool}()

function stack(K::Int, A::Int, B::Int)
    newA = int((K - B^2)/A)
    N = int(floor((sqrt(K) + B)/newA))
    newB = N*newA-B
    if get(seen, (K, N, newA, newB), false)
        return 0
    end
    seen[(K, N, newA, newB)] = true
    return stack(K, newA, newB) + 1
end

function issqr(n::Int)
    int(sqrt(n))^2 == n ? true : false
end
    
function main()
    ct = 0
    for x=2:10000
        strt = int(floor(sqrt(x)))
        if strt^2 != x && stack(x, 1, strt) % 2 == 1
            ct += 1
        end
    end
    println(ct)
end

main()
