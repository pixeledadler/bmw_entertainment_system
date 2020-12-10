#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


// This class implements a methid to communicate woth a SQLite DB for music 
// track specifics. 
// Checks, if database exist
// If it doesn't makes a new database with two tables
// 1. for music track information
// - track ID
// - title
// - artist
// - file location
// 2. playlists
// - playlist id
// - track id

mod database {
    struct Tables{
        name: String,
    }
    struct MusicTrack {
        id: i32,
        title: String,
        artist: String,
        file_path: String
    }
    
    pub fn database_connection () -> bool {
        let connection = rusqlite::Connection::open("music_db.sql");
        let table_table = connection.execute("SELECT name FROM sqlite_master;");
        for table in table_table {
            if table.name == "tracks"{
                return true;

            }
        }
        connection.execute("
        CREATE TABLE tracks(
            id INTEGER PRIMARY KEY,
            title TEXT,
            artist TEXT,
            file_path TEXT)", rusqlite::NO_PARAMS);
        return false;
    }
}