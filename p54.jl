module ty

type Hand
    vals::Vector{Int}
    suits::Vector{Char}
end

export Hand
end 

using ty

function isstraight(h::Hand)
    vs = sort(unique(v))
    if length(vs) < 5
        return false
    end
    if sum(diff(vs)) == 4
        return true
    end
end

function matches(v::Vector{Int})
    d, drev = count(v)
    m = maximum(keys(drev)) # hi
    if m == 1
        dff = sum(diff(v))
        if dff == 4
            return 5, maximum(d) #straight
        else
            return 1, maximum(d) #no matches
        end
    end
    if m == 2
        if length(d) == 3
            return 3, drev[2] #two pair
        else
            return 2, drev[2] #pair
        end
    elseif m == 3
        if length(d) == 2
            return 7, drev[3] # full house
        else
            return 4, drev[3] # three of kind
        end
    elseif m == 4
        return 8, drev[3] # four of a kind
    end
end

function score(h::Hand)
    ss  = 0
    ms, mc = matches(h.vals)
    if length(unique(h.suits)) == 1
        ss = 6
    end
    if ms == 5 && ss == 6
        if minimum(h.vals) == 10
            return 10, mc
        else 
            return 9, mc
        end
    end
    return maximum((ss,ms)), mc
end
        
function count(v::Vector{Int})
    d = Dict{Int, Int}()
    drev = Dict{Int, Int}()
    for i in v
        d[i] = get(d,i,0) + 1
    end
    for (k,v) in sort(collect(d))
        drev[v] = k
    end
    return d, drev
end

function stov(c::Char)
    if c == 'T'
        return 10
    elseif c == 'J'
        return 11
    elseif c == 'Q'
        return 12
    elseif c == 'K'
        return 13
    elseif c == 'A'
        return 14
    else
        return int(c) - 48
    end
end

function tohands(h::Vector{String})
    str1 = h[1:5]
    str2 = h[6:10]
    h1 = tohand(str1)
    h2 = tohand(str2)
    return (h1, h2)
end

function tohand(h::Vector{String})
    vals = Array(Int,5)
    suits = Array(Char,5)
    for i in 1:length(h)
        vals[i] = stov(h[i][1])
        suits[i] = h[i][2]
    end
    ind = sortperm(vals)
    return Hand(vals[ind], suits[ind])
end

function main()
    arr = readcsv(open("data/poker.txt"), String)
    win = Bool[]
    for i=1:1000
        hands = tohands(collect(arr[i, :]))
        s1, h1 = score(hands[1])
        s2, h2 = score(hands[2])
        if s1 > s2
            push!(win, true)
        elseif s2 > s1
            push!(win, false)
        elseif s1 == s2
            if h1 > h2
                push!(win, true)
            elseif h2 > h1
                push!(win, false)
            elseif h1 == h2
                println(hands), s1, h1
            end
        end
    end
    return win
end
