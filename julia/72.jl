include("tools.jl")

function ctfracs(N::Int)
    tot = 0 
    primes = primesTo(N)
    for n=2:N
        ct = n-1
        facts = unique(primefacts(n, primes))
        for i in facts
            ct -= div(n,i) -1
            ct += i -1
        end
        tot += ct
        println(n, " ", tot)
    end
    return tot
end
