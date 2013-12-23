include("tools.jl")

function main(N::Int)
    global primes = primesTo(N)
    l = length(primes)
    tot = recurse(1, N, BigInt(1), 1,l)
    return tot
end

function recurse(x::Int, N::Int, ct::BigInt, pri::Int, l::Int)
    # take first p
    p = primes[pri]
    #cut short if x getting too large
    if x > N/p
        return 0
    end
    tot = 0
    if l > pri
        # skip this prime
        tot += recurse(x, N, ct, pri+1,l)
    end
    x *= p
    ct *= (p-1)
    tot += ct
    if l > pri
        tot += recurse(x, N, ct, pri+1,l)
    end
    while x <= N/p
        x *= p
        ct *= p
        tot += ct
        if l>pri
            tot += recurse(x, N, ct, pri+1,l)
        end
    end
    return tot
end
