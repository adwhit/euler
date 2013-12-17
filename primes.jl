function isprime(x::Int)
    ps = primesTo(int(sqrt(x)))
    for i in ps
        if x%i == 0
            return false
        end
    end
    return true
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
