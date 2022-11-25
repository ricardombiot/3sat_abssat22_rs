include("./gen_3sat_cnf.jl")
using Dates
# Example:
# julia generator.jl n=10 l=7 c=27
function generator_main(db, args)
    #println(args)
    params = Dict{String, Int64}()
    for argument in args
        parts = split(argument,"=")
        key = "$(parts[1])"
        value = parse(Int64,"$(parts[2])",base=10)
        params[key] = value
    end

    #println(params)
    n = params["n"]
    n_literals = params["l"]
    n_clauses = params["c"]
    today=now()
    today=Int(floor(datetime2unix(today)))
    #today = Dates.today()
    base_set_name = replace("vsminisat_set$(n)n$(n_literals)l$(n_clauses)c_$(today)","-" => "")

    #println(base_set_name)
    generator(db, base_set_name, n, n_literals, n_clauses)
    write_info_file(base_set_name)
end

function write_info_file(base_set_name)
    open("./output/info.txt", "w") do io
        write(io, "$base_set_name\n")
    end;
end

function generator(db, base_set_name, n, n_literals, n_clauses)
    for i in 1:n
        tagname = "i$(i)_v$(n_literals)_c$n_clauses.cnf"
        valid = false
        while !valid
            println("Generating unique... $i")
            instance = generate_3sat_cnf(n_literals, n_clauses)
            valid = test_massive_3sat.DBController.insert_instance(db, tagname, instance)
        end
    end
end


