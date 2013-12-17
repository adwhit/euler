function solve(D::Int)
    x = 1
    y = 1
    iter = 0
    while true
        rem = x^2 - 1 - D*y^2
        if rem == 0
            return x, y
        elseif rem > 0
            y += 1
        elseif rem < 0
            x += max(-int(rem/(2*x + 1)),1)
        end
        iter += 1
        if iter % 100000000 == 0
            println(iter," ", x, " ", y, " ", rem)
            if iter > 400000000
                println("Skipping")
                return 0, 0
            end
        end
    end
end
        
function main()
    N = 1000
    sqrs = [i^2 for i=1:int(sqrt(N))]
    for D = 1:1000
        if D in sqrs
            continue
        end
        x, y = solve(D)
        println(D," ",x," ", y)
    end
end
