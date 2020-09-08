-- v = v0 + a*t
-- s = 0.5*(v0 + v)*t

function calculateEndVelocity (v0, a ,t)
    return (v0 + a*t)
end 

function calculateDistance (v0, v, t)
    return (0.5*(v0 + v)*t)
end 

print(calculateDistance(2, 2, 6))

