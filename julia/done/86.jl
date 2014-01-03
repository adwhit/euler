include("../tools.jl")

function main()
    mx = 1810
    while true
        ct = 0
        rtmx = int(sqrt(mx))+1
        for n = 1:mx
            for m = (n+1):2:mx
                if GCD(m, n) != 1
                    continue
                end
                k = 1
                a = m^2 - n^2
                b = 2*m*n
                c = m^2 + n^2
                if minimum([a,b]) > mx
                    break
                end
                if a > b
                    #a is smallest
                    b, a = a, b
                end
                while k*a <= mx
                    s1 = k*a
                    s2 = k*b
                    s3 = k*c
                    if k*b <= mx #kb is max side length
                        ct += div(s1,2)
                    end
                    for x = 1:div(s2,2)
                        if s2 -x <= mx && 2*s2 <= 2*(s1+x) #s2-x is max side length
                            ct += 1
                        end
                    end
                    k += 1
                end
            end
        end
        println(mx, " ",ct) #off by 12?
        if  ct > 1000000
            break
        end
        mx += 1 
    end
end

                    




                    
