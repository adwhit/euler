include("tools.jl")

function quickjoin(arr::Array{Int})
    s = 0
    l = length(arr)
    for i in 1:l
        s += 10^(l-i) * arr[i]
    end
    return s
end

function reversible(arr::Vector{Int})
    l = length(arr)
    carry = 0
    for i in 1:l
        digit = arr[i] + arr[l-i+1] + carry
        carry = div(digit,10)
        base = digit % 10
        if base % 2 == 0
            return false
        end
    end
    return true
end

function chk(n::Vector{Int})
    n = quickjoin(n)
    new = quickjoin(quicksplit(n))
    println(n," ",new," ", n+new)
end

function main()
    ct = 0
    #arr = [0,0,0,0,0,0,0,0]
    for i1=1:9
        sense = i1 % 2
        for i2=0:9
            if (i2 != 0) && i2 % 2 != sense
                if reversible(Int[i1, i2])
                    ct += 1
                    chk(Int[i1, i2])
                end
            end
            for i3=0:9
                if (i3 != 0) && (i3 % 2) != sense
                    if reversible(Int[i1, i2, i3])
                        ct += 1
                        chk(Int[i1, i2, i3])
                    end
                end
                for i4=0:9
                    if (i4 != 0) && (i4 % 2) != sense
                        if reversible(Int[i1, i2, i3, i4])
                            ct += 1
                            chk(Int[i1, i2, i3, i4])
                        end
                    end
                    for i5=0:9
                        if (i5 != 0)# && (i5 % 2) != sense
                            if reversible(Int[i1, i2, i3, i4, i5])
                                ct += 1
                                chk(Int[i1, i2, i3, i4, i5])
                            end
                        end
               #         for i6=0:9
               #             for i7=0:9
               #                 for i8=0:9
                   end
               end
           end
        end
    end
    println(ct)
end

function qcheck()
    for i=10000:99999
        if reversible(quicksplit(i))
            println(i)
        end
    end
end

main()
