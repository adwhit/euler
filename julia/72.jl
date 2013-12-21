include("tools.jl")

function ctfracs(N::Int)
    tot = 0 
    primes = primesTo(N)
    for n=2:N
        ct = n-1
        facts = unique(primefacts(n, primes))
        red = reduce(*, facts)
        rep = div(facts,red)
        for i in facts
            ct -= div(n,i) -1
            ct += div(red,n) - 1
        end
        tot += ct
        println(n, " ", ct, " ",tot)
    end
    return tot
end
