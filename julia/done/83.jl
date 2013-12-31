
#init first column


function main()
    arr = readcsv("../data/matrix.txt", Int)
    route = zeros(Int,(80,80))
    route[1,1] = arr[1,1]
    for x = 2:80
        route[x,1] = route[x-1,1] + arr[x,1]
    end
    for N = 2:80
        #intial fill gives upper bound
        route[:,N] = route[:,N-1] + arr[:,N]
    end
    updated = true
    while updated
        updated = false
        for N = 1:80
            for i = 1:80
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
                if N > 1
                    leftscore = route[i, N-1]
                    if leftscore + val < score
                        score = leftscore + val
                    end
                end
                if N < 80
                    rightscore = route[i, N+1]
                    if rightscore + val < score
                        score = rightscore + val
                        println(i, " ", N)
                    end
                end
                #update
                if score < route[i,N]
                    updated = true
                    route[i,N] = score
                end
            end
        end
    end
    println(route[1:5,1:5])
    println()
    println(arr[1:5,1:5])
    println()
    println(route[80,80])
end

