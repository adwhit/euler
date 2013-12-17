function genarr(fn::Function, row::Int, lg::Array{Bool,2})
    x = 1
    while true
        v = fn(x)
        if v > 10000
            return
        end
        lg[row, v]=true
        x=x+1
    end
end

function main()
    lg = bool(zeros(6,10000))
    genarr(x->int(x*(x+1)/2), 1, lg)
    genarr(x->int(x*(3x-1)/2), 3, lg)
    genarr(x->int(x*(5x-3)/2), 5, lg)
    genarr(x->x*x, 2, lg)
    genarr(x->x*(2x-1), 4, lg)
    genarr(x->x*(3x-2), 6, lg)
    for i=10:99
        iarr = Int[i]
        cl = bool(zeros(6))
        recurse(iarr, cl, lg)
    end
end


function recurse(iarr::Vector{Int}, cl::Vector{Bool}, lg::Array{Bool,2})
    for j=10:99
        x = int(string(iarr[end],j))
        for z = 1:6
            if lg[z,x] && !cl[z] #if it is a special number and hasn't already been done
                if length(iarr) == 5
                    #final stretch
                    y = int(string(j, iarr[1]))
                    for z2 = 1:6
                        if lg[z2,y] && !cl[z2] && (z2 != z)
                            push!(iarr, j)
                            cl[z] = true
                            println(iarr)
                            return
                        end
                    end
                    continue
                end
                clcp = copy(cl)
                iarrcp = copy(iarr)

                clcp[z] = true
                push!(iarrcp, j)

                recurse(iarrcp, clcp, lg)

            end
        end
    end
end
