
function modulus(x::Int,y::Int)
    return sqrt(x^2 + y^2)
end

function angle(A::Vector{Int}, B::Vector{Int})
    costheta = dot(A,B) / modulus(A...) / modulus(B...)
    return acos(costheta)
end

function main()
    ct = 0
    eps = 0.0000001
    arr = readcsv("../data/triangles.txt", Int)
    for i in 1:size(arr)[1]
        line = arr[i,:]
        A = line[1:2]
        B = line[3:4]
        C = line[5:6]
        tot = angle(A,B) + angle(B,C) + angle(A,C)
        if tot > 2*pi - eps
            ct += 1
        end
    end
    println(ct)
end
