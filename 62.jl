function main()
    N=10000
    d = Dict{String,Vector{Int}}()
    for i=1:N
        s = join(sort(convert(Vector{Char}, string(i^3))))
        v = get(d, s, Int[])
        push!(v, i^3)
        d[s] = v
    end
    for v in collect(values(d))
        if length(v) > 4
            println(v)
            println(length(v))
        end
    end
end
