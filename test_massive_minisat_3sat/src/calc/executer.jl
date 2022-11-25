

function calc_main(db)
    list_instances = test_massive_3sat.DBController.select_peding_instances(db)

    path_instance = "./output/instances/instance_work.cnf"
    instance_name = "instance_work.cnf"
    path_output_solver_minisat = "./output/solver_minisat/instance_work.txt"
    path_output_solver_rs = "./output/solver_sat_machine_rs/instance_work.txt"

    for item in list_instances 
        id = item.id
        instance = item.instance
        #println(instance)
        write_instance_to_file(instance, path_instance)
        
        solver_minisat(instance_name)
        write_solution_on_database(db, id, "minisat", path_output_solver_minisat)

       # solver_machine_rs(instance_name)
        solver_machine_rs(instance_name)
        write_solution_on_database(db, id, "3sat_rs", path_output_solver_rs)

        check_solution(db, id, path_output_solver_minisat,path_output_solver_rs)
    end

    calc_result(db)
end

function calc_result(db)
    list_instances = test_massive_3sat.DBController.select_all_instances(db)
    total_instances = 0
    total_instances_sat = 0
    total_instances_unsat = 0
    flag_valid = "ok"
    for item in list_instances 
        is_valid = item.is_valid
        is_sat = item.is_sat

        total_instances += 1
        if is_sat == 1
            total_instances_sat += 1
        else
            total_instances_unsat += 1
        end

        if is_valid != 1
            flag_valid = "err"
        end
    end

    info_file = "./output/info.txt"
    lines = readlines(info_file)
    name = lines[1]
    name = name*"_$flag_valid"

    rm(info_file)

    open(info_file, "w") do io
        write(io, "$name\n")
        if flag_valid == "ok" 
            write(io, "ALL_VALID: true\n")
        else
            write(io, "ALL_VALID: false\n")
        end
        
        write(io, "TOTAL INSTANCES: $total_instances\n")
        write(io, " - SAT INSTANCES: $total_instances_sat\n")
        write(io, " - UNSAT INSTANCES: $total_instances_unsat\n")
    end;
end

function check_solution(db, id, path_output_solver_minisat,path_output_solver_rs)
    solver_minisat_file_content = Set(readlines(path_output_solver_minisat))
    solver_machine_rs_file_content = Set(readlines(path_output_solver_rs))

    minisat_answer_is_sat = "SAT" in solver_minisat_file_content
    machine_rs_answer_is_sat = "SAT" in solver_machine_rs_file_content 

    is_valid = machine_rs_answer_is_sat == minisat_answer_is_sat
    is_sat = minisat_answer_is_sat
    test_massive_3sat.DBController.update_flags(db, id, is_valid, is_sat)
end

function write_instance_to_file(instance, path_instance)
    open(path_instance, "w") do io
        write(io,instance)
    end;
end

function write_solution_on_database(db, id, mode, file_txt)
    test_massive_3sat.DBController.update(db, id, mode, file_txt)
end

function solver_minisat(instance_file :: String) 
    instance_file = replace(instance_file, ".cnf" => "")
    command_main = Cmd(["sh","./run_minisat.sh",instance_file])
    execution_time = @elapsed run(ignorestatus(command_main))

    return execution_time
end


function solver_machine_rs(instance_file :: String) 
    println(instance_file)
    command_main = Cmd(["./abs3sat","mode=solver",instance_file])
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