
function fractodec(num::Int, den::Int)
    arr = zeros(Int, 100)
    #remove integral part
    num = num % den
    for i=1:100
        num *= 10
        arr[i] = div(num,den)
        num = num % den
    end
    return arr
end
