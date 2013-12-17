function solve(r::Int, c::Int)
    return sum(collect(1:r)*transpose(collect(1:c)))
end

function main()
    r = 1
    c = 1
    best = 2000000
    bestr = 1
    bestc = 1
    while true
        for c = 1:r
            n = solve(r, c)
            if abs(2000000 - n) < best
                best = abs(2000000 -n)
                bestr = r
                bestc = c
            elseif n > (2000000 + best)
                if c == 1
                    println(r)
                    println(bestr)
                    println(bestc)
                    println(best)
                    println(bestr*bestc)
                    return
                end
                break
            end
            c += 1
        end
        r += 1
    end
end
