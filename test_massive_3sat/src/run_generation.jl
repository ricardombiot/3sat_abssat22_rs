import Pkg; 
Pkg.add("SQLite")
#Pkg.add("SHA")
include("./test_massive_3sat.jl")

# julia run_generation.jl n=10 l=7 c=27
test_massive_3sat.generate_stage(ARGS)