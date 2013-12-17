include("tools.jl")

function factdict(bound::Int)
    factd = Dict{Int, Vector{Int}}()
    primes = primesTo(bound)
    for i in 1:bound
        println(i)
        if i in primes
            factd[i] = Int[]
        else
            factd[i] = primefacts(i, primes)
        end
    end
    return factd
end


function totient(n::Int, primes::Vector{Int})
    facts = primefacts(n, primes)
    sieve = ones(Bool, n-1)
    for f in unique(facts)
        for i=f:f:n-1
            sieve[i] = false
        end
    end
    return sum(sieve)
end

function totient(n::Int)
    return totient(n, primesTo(n))
end

function isperm(n::Int, m::Int)
    if sort(quicksplit(n)) == sort(quicksplit(m))
        return true
    else
        return false
    end
end

function main()
    minr = 100
    bestn = 1
    bound = 10000000
    primes = primesTo(div(bound, 100))
    nprimes = length(primes)
    for i in 1:length(primes)
        p1 = primes[i]
        for j in 1:i
            p2 = primes[j]
            v = p1*p2
            if v > bound
                break
            end
            #toti = totient(v, primes)
            toti = v - p1 - p2 + 1
            rat = v/toti
            if rat < minr && isperm(toti, v)
                minr = rat
                bestn = v
                println(v, " ", toti," ", rat)
            end
        end
    end
    return bestn, minr
end
