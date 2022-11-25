import Pkg; 
Pkg.add("SQLite")
#Pkg.add("SHA")
include("./test_massive_3sat.jl")

test_massive_3sat.calc_stage()