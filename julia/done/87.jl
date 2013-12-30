function powr(N::Int)
    primes = primesTo(N)
    counter = zeros(Bool,N)
    i = 1
    while true
        fth = primes[i]^4
        if fth > N
            break
        end
        j = 1
        while true
            cub = primes[j]^3
            if fth + cub > N
                break
            end
            k=1
            while true
                sqr = primes[k]^2
                s = sqr + cub + fth
                if s > N
                    break
                end
                counter[s] = true
                k += 1
            end
            j += 1
        end
        i += 1
    end
    return counter
end

function main()
    c = powr(50000000)
    println(sum(c))
end


