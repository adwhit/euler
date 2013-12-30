include("tools.jl")

function stack(A::Int, B::Int, A0::Int)
    newA = int((K - B^2)/A)
    N = int(floor((sqrt(K) + B)/newA))
    newB = N*newA-B
    if N == 2*A0
        return Int[N]
    else
        arr = stack(newA, newB, A0)
        push!(arr,N)
        return arr
    end
end

function convergent(D::Int, arr::Vector{Int})
    a0 = int(floor(sqrt(D)))
    num = BigInt(1)
    den = BigInt(arr[1])
    for i in 2:length(arr)
        num, den = den, den*arr[i] + num
    end
    num, den = den*a0 + num, den
    gcd = GCD(num,den)
    return div(num,gcd), div(den,gcd)
end



function getstack(K::Int)
    strt = int(floor(sqrt(K)))
    stk = stack(K, 1, strt, strt)
    return stk[end:-1:1]
end

function main()
    best = 0
    ind  = 0
    for D = 1:1000
        if int(floor(sqrt(D)))^2 == D
            continue
        end
        arr = getstack(D)
        arr = repmat(arr, 10)
        for i =1:length(arr)
            x, y = convergent(D, arr[1:i])
            if x^2 - D*y^2 == 1
                if x > best
                    best = x
                    ind = D
                    println(ind," ", best)
                end
                break
            end
            if i == length(arr)
                println("Failed", D)
            end
        end
    end
end
