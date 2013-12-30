include("tools.jl")

function solve(N::Int)
    mx = int(sqrt(N/2))+1
    counter = zeros(N)
    for n in 1:mx
        for m in (n+1):2:mx
            if GCD(m,n) == 1
                L = 2*m*(m + n)
                if L > N
                    #shortcut
                    break
                end
                for k = 1:div(N,L)
                    counter[k*L] += 1
                end
            end
        end
    end
    return counter
end

function main()
    counter = solve(1500000)
    println(sum(counter .== 1))
end

