include("tools.jl")

function main()
    target = 3/7
    diff = 100.0
    nearn = 0
    neard = 0
    for d=990000:1000000
        for n = int(0.42*d) : int(0.5 * d)
            aim = n/d
            if aim < target && (target - aim) < diff && GCD(n, d) == 1
                diff = target - aim
                nearn = n
                neard = d
            end
        end
    end
    println(nearn)
end
