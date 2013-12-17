function cf(N::Int, num::Int, den::Int, ct::Int)
    dots = N - den^2
    frac = num*(sqrt(N) - den)
    a = int(floor(frac))
end


function GCD(x::Int,y::Int)
    r = x % y
    if r == 0
        return y
    else
        return GCD(y, r)
    end
end
