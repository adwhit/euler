include("primes.jl")

plook = primelookup(100000000)

function primecat(x::Int, y::Int)
    p1 = int(string(x,y))
    p2 = int(string(y,x))
    if plook[p1] && plook[p2]
        return true
    else
        return false
    end
end

function main()
    arr = primesTo(10000)
    lg = bool(zeros(length(arr), length(arr)))
    for i in 1:length(arr)
        for j in i+1:length(arr)
            if primecat(arr[i], arr[j])
                lg[i,j] = true
                lg[j,i] = true
            end
        end
    end
    for i in 1:length(arr)
        for j in i+1:length(arr)
            if lg[i,j]
                for k in j+1:length(arr)
                    if lg[j,k] && lg[i,k]
                        for l in k+1:length(arr)
                            if lg[i,l] && lg[j,l] && lg[k,l]
                                for m in l+1:length(arr)
                                    if lg[i,m] && lg[j,m] && lg[k,m] && lg[l,m]
                                        println(arr[i]," ",arr[j]," ",arr[k]," ",arr[l]," ",arr[m])
                                    end
                                end
                            end
                        end
                    end
                end
            end
        end
    end
    
end


