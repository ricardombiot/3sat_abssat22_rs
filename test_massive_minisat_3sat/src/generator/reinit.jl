using Dates

function reinit()
    if isdir("./output")
        save_zip()
        drop_db()
        drop_info()
    end
end

function save_zip()
    zip_output()
end

function read_info_name()
    try
        f = open("./output/info.txt")
        lines = readlines(f)
        close(f)
    
        return lines[1]
    catch
        return "notinfo"
    end

end

function zip_output()
    name = read_info_name()
    n=now()
    u=Int(floor(datetime2unix(n)))
    zip_name = "./output_$name.zip"
    command_main = Cmd(["zip","-r",zip_name,"./output"])
    run(command_main)
end


function drop_db()
    try
        command_main = Cmd(["rm","./output/3sat_test.sqlite"])
        run(command_main)
    catch
    end
end

function drop_info()
    try
        command_main = Cmd(["rm","./output/info.txt"])
        run(command_main)
    catch
    end
end