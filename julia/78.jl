function main()
    N = 100000
    d = 1000000
    parr = zeros(Int,N)
    parr[1] = 1
    target = 10
    for n = 2:N
        i = 1
        g = 1
        w = 0
        while g <= n
            if g == n
                w += (-1)^(abs(i-1))
            else
                w += (-1)^(abs(i-1))*parr[n-g]
            end
            i = i > 0 ? -i : -i + 1
            g = div(i*(3*i-1),2)
        end
        parr[n] = w % d
        if parr[n] % target == 0
            println(n, " ",parr[n])
            target *= 10
        end
    end
end
