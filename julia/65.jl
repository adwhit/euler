function rec(level::Int, maxdepth::Int)
    if level == maxdepth
        n = BigInt(1)
        d = BigInt(getc(level))
        return (n, d)
    end
    con = getc(level)
    n, d = rec(level+1, maxdepth)
    return (d, d*con + n)
end

function getc(level::Int)
    if level == 1
        return 2
    elseif level % 3 == 0
        return 2*int(level/3)
    else
        return 1
    end
end

function solve(depth::Int)
    d, n = rec(1, depth)
    println(sum(convert(Vector{Int},convert(Vector{Char},string(n))) - 48))
end

solve(100)
