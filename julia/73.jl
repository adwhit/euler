include("tools.jl")

function main()
    uptarget = 1/2
    lowtarget = 1/3
    ct = 0
    for d=1:12000
        for n = 1:(d-1)
            aim = n/d
            if aim < uptarget && lowtarget < aim && GCD(n,d) == 1
                ct += 1
            end
        end
    end
    println(ct)
end
