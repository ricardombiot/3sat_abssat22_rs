module DBController
    using SQLite
    using SHA

    function connect()
        db = SQLite.DB("./output/3sat_test.sqlite")
        return db
    end

    function create_table_instances(db)
        try
            SQLite.drop!(db, "instances")
        catch
            println("We couldnt drop 'instances' table because doesnt exist.")
        end
        
        sql_table = "CREATE TABLE IF NOT EXISTS instances (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tagname TEXT NOT NULL,
            instance TEXT NOT NULL,
            sha256_key TEXT NOT NULL UNIQUE,
            result_minisat TEXT,
            result_exaustive TEXT,
            result_3sat_rs TEXT,
            is_valid BOOLEAN,
            is_sat BOOLEAN
        );"

        DBInterface.execute(db, sql_table)
    end

    function insert_instance(db,tagname, instance)
        try
            sha_key = bytes2hex(sha256(instance))
            sql_insert = "INSERT INTO instances (tagname, instance, sha256_key, result_minisat, result_exaustive, result_3sat_rs, is_valid, is_sat)
                          VALUES('$tagname','$instance','$sha_key', NULL, NULL, NULL, NULL, NULL);"
            DBInterface.execute(db, sql_insert)
            return true
        catch
            return false
        end              
    end

    function select_peding_instances(db)
        sql_select = "select id, instance from instances where is_valid is NULL;"
        return DBInterface.execute(db, sql_select);
    end

    function select_all_instances(db)
        sql_select = "select id, is_valid, is_sat from instances;"
        return DBInterface.execute(db, sql_select);
    end


    function update(db, id, mode, file_txt)
        f = open(file_txt)
        lines = readlines(f)
        close(f)
        result_txt = ""
        for line in lines
            result_txt *= line * "\n" 
        end

        sql_update = "UPDATE instances SET result_$mode = '$result_txt' WHERE id = $id;"
        return DBInterface.execute(db, sql_update);
    end

    function update_flags(db, id, is_valid, is_sat)
        sql_update = "UPDATE instances SET is_valid = $is_valid , is_sat = $is_sat WHERE id = $id;"
        return DBInterface.execute(db, sql_update);
    end

end