arr = readcsv("../data/matrix.txt", Int)
route = zeros(Int,(80,80))

#init first column
route[:,1] = arr[:,1]

function fillcol(N::Int)
    for i in 1:80
        val = arr[i,N]
        score = route[i,N]
        #try to improve
        if i > 1
            upscore = route[i-1,N]
            if upscore + val < score
                score = upscore + val
            end
        end
        if i < 80
            downscore = route[i+1,N]
            if downscore + val < score
                score = downscore + val
            end
        end
        #update
        route[i,N] = score
    end
end

function main()
    for N = 2:80
        #intial fill
        route[:,N] = route[:,N-1] + arr[:,N]
        for r = 1:80
            fillcol(N)
        end
    end
    println(minimum(route[:,80]))
end

