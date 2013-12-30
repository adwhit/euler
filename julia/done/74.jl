seen = [1=>1,2=>1,145=>1,169=>3,363601=>3,1454=>3,871=>2,45361=>2,872=>2,45362=>2,40585=>1]

include("tools.jl")

function chain(n::Int)
    return sum([factorial(x) for x in ntoarr(n)])
end

function main()
    for n=1:1000000
        makechain(n)
        if n%100000 == 0
            println(n)
        end
    end
    ct = 0
    for v in values(seen)
        if v == 60
            ct += 1
        end
    end
    println(ct)
end

function makechain(n::Int)
    ct = 1
    currchain = Int[n]
    while true
        n1 = chain(n)
        if n1 == n
            seen[n] = 1
            break
        end
        n = n1
        if n in keys(seen)
            ct += seen[n]
            for (i,num) in enumerate(currchain)
                seen[num] = ct - i + 1
            end
            push!(currchain, n)
            break
        else
            push!(currchain, n)
        end
        ct += 1
    end
    return currchain, ct
end

