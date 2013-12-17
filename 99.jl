function main()
    arr = readcsv("data/base_exp.txt", Int)
    best = 0
    biggest = 0
    for i in 1:size(arr)[1]
        n = arr[i,2] * log(arr[i,1])
        if n > biggest
            biggest = n
            best = i
        end
    end
    println(best)
end

main()
    

