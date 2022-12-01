use rusqlite::{Connection, Params, params};

// Creates the file with the two default tables :D

fn create_table_defaults() {
    let con = Connection::open("./main.db");
    // both return a print statement
    match con {
        Ok(T) => {
            // it creates file here too :)
            T.execute(
                "CREATE TABLE main (
                title TEXT,
                author TEXT,
                album TEXT,
                features TEXT,
                savelocationmp3 TEXT,
                savelocationjpg TEXT,
                datedownloaded DATE,
                ischild BOOL,
                uniqueid TEXT PRIMARY KEY,
                plays SMALLINT,
                weight SMALLINT
                )", (params![])
                    // need to pass in params![] because that arguement needs a type that supports "param" type
            ).unwrap();
            T.execute(
                "CREATE TABLE metadata (
                title TEXT,
                description TEXT,
                datecreated DATE,
                songcount SMALLINT,
                totaltime TEXT,
                isautogen BOOL
                )", (params![])
            ).unwrap();
            return println!("created!")

        }
        Err(E) => { println!("its saul goodman :) {E}") }
    }
}
