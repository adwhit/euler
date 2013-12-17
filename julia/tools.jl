function primefacts(n::Int)
    primes = primesTo(div(n,2))
    return primefacts(n, primes)
end
        
function primefacts(n::Int, primes::Vector{Int})
    arr = Int[]
    if n in primes
        return arr
    end
    nprimes = length(primes)
    half = div(n,2)
    i = 1
    while true
        p = primes[i]
        if p > half
            return arr
        end
        if n % p == 0
            push!(arr, p)
            n = div(n,p)
            if n == 1
                return arr
            end
        else
            i += 1
            if i > nprimes
                return arr
            end
        end
    end
end

function GCD(x::Int,y::Int)
    r = x % y
    if r == 0
        return y
    else
        return GCD(y, r)
    end
end

function ntoarr(n::Int) 
   return convert(Vector{Int},convert(Vector{Char},string(n))) - 48
end

function isprime(x::Int)
    ps = primesTo(int(sqrt(x)))
    for i in ps
        if x%i == 0
            return false
        end
    end
    return true
end

function quicksplit(n::Int)
    arr = Int[]
    while n > 0
        push!(arr, n % 10)
        n = div(n,10)
    end
    return arr
end

function primesTo(x::Int)
    primes = Int[]
    parr = bool(ones(x))
    parr[1] = false
    for i=4:2:x
        parr[i] = false
    end
    push!(primes, 2)

    for k=3:2:x
        if parr[k]
            push!(primes, k)
            for i=k*2:k:x
                parr[i] = false
            end
        end
    end
    return primes
end

function primelookup(x::Int)
    primes = Int[]
    parr = bool(ones(x))
    parr[1] = false
    for i=4:2:x
        parr[i] = false
    end

    for k=3:2:x
        if parr[k]
            for i=k*2:k:x
                parr[i] = false
            end
        end
    end
    return parr
end
