include("primes.jl")

primes = reverse(primesTo(1000000))
seen = Dict{(Int,Int),Int}()

function recurse(N::Int, traverse::Int)
    while primes[traverse] > N
        traverse += 1
    end
    tot = 0
    for i in traverse:length(primes) 
        newN = N-primes[i]
        if newN == 1
            ct = 0
        elseif newN == 0
            ct = 1
        else
            ct = get(seen, (newN, i), -1)
            if ct == -1
                ct = recurse(newN, i)
                seen[(newN, i)] = ct
            end
        end
        tot += ct
    end
    return tot
end

function main()
    N = 2
    tot = 0
    while tot < 50000
        tot = recurse(N, 1)
        N += 1
    end
    println(N, " ", tot)
end
