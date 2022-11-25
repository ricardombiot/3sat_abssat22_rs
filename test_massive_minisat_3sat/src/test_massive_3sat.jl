module test_massive_3sat


    include("dbcontroller.jl")
    include("./generator/generator.jl")
    include("./generator/reinit.jl")

    include("./calc/exaustive_solver.jl")
    include("./calc/executer.jl")
   

    function generate_stage(args)
        reinit()
        db = DBController.connect()
        DBController.create_table_instances(db)

        generator_main(db, args)
    end

    function calc_stage()
        db = DBController.connect()
        calc_main(db)
    end

    function save_stage()
        save_zip()
    end

end # module
