include("./base/base.jl")

function main()
    base_path_instances = "./output/instances"
    base_path_solver_ex = "./output/solver_exhaustive"
    base_path_solver_machine_rs = "./output/solver_sat_machine_rs"

    list_instances = readdir(base_path_instances)

    total_time = 0
    total_instances = length(list_instances)
    counter = 0
    for instance_file in list_instances
        if contains(instance_file, ".cnf")
            counter += 1
            path_instance = "$base_path_instances/$instance_file"
            solver_instance_file = replace("$instance_file", ".cnf" => ".txt")
            path_output_solver_ex = "$base_path_solver_ex/$solver_instance_file"
            path_output_solver_machine_rs = "$base_path_solver_machine_rs/$solver_instance_file"

            #only execution time...
            total_time_ex = solver_ex(path_instance, path_output_solver_ex)
            #println("... $total_time_ex segs")
            total_time_ex = trunc(total_time_ex, digits=2)

            total_time_rs = solver_machine_rs(instance_file)
            total_time_rs = trunc(total_time_rs, digits=2)
            
            #=total_time_machine = solver_machine(path_instance, path_output_solver_machine)
            total_time_machine = trunc(total_time_machine, digits=2)

            total_time_machine_rs = solver_machine_rs(path_instance, path_output_solver_machine_rs)
            total_time_machine_rs = trunc(total_time_machine_rs, digits=2)


            if total_time_machine_rs < total_time_machine
                spaik_improvement = trunc( (1 - (total_time_machine_rs/total_time_machine)) * 100, digits=2)
            else
                spaik_improvement = trunc( (1 - (total_time_machine/total_time_machine_rs)) * -100 , digits=2)
            end
            =#

            total_time_instance = trunc(total_time_ex+total_time_rs, digits=3)
            total_time += trunc(total_time_instance, digits=3)
            rest_instances = total_instances-counter
            if rest_instances == 0
                estimation_time_min = 0
            else
                avg_time = total_time/counter
                estimation_time_min = trunc( (rest_instances * avg_time) / 60, digits=2)
            end
            total_time_min = trunc(total_time / 60, digits=2)

            println("- - - - - - - - - - - - - -")
            println("Instance: $counter/$total_instances ")
            println("  Exponential:$total_time_ex segs")
            println("  MachineRs: $total_time_rs segs")
            println("  ## Total: $total_time_instance seg")
            println("[Total Time: $total_time_min min]")
            println("[Estimation to finishied: $estimation_time_min min]")
            println("- - - - - - - - - - - - - -")

        end
    end

end


function solver_ex(path_instance, path_output_solver_ex)
    println("ExhaustiveSolver: [WORKING ON $path_instance]")
    solver = ExhaustiveSolver.new(path_instance)
    execution_time = @elapsed ExhaustiveSolver.run!(solver)

    open(path_output_solver_ex, "w") do io
        if isempty(solver.list_solutions)
            write(io, "UNSAT\n")
        else
            write(io, "SAT\n")
            for solution in solver.list_solutions
                txt_solution = solution_to_string(solution)
                write(io, "$txt_solution\n")
            end
        end
    end;

    return execution_time
end

function solver_machine_rs(instance_file :: String) 
    command_main = Cmd(["./abs3sat","mode=exhaustive",instance_file])
    execution_time = @elapsed run(command_main)

    return execution_time
end

function solution_to_string(solution :: BitArray{1}) :: String
    txt_solution = ""
    for bit in solution
        if bit
            txt_solution *= "1"
        else
            txt_solution *= "0"
        end
    end

    return txt_solution
end

main()
