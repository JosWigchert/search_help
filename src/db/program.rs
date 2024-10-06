use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Program {
    pub id: Option<i32>,
    pub name: String,
    pub commands: Vec<String>,
    pub help_text: String,
}

impl Program {
    // Constructor for creating a new Program instance
    pub fn new(name: &str, help_command: Vec<String>, help_text: &str) -> Self {
        Program {
            id: None,
            name: name.to_string(),
            commands: help_command.clone(),
            help_text: help_text.to_string(),
        }
    }

    // Create the program table
    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS program (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                name          TEXT NOT NULL,
                help_command  TEXT NOT NULL,
                help_text     TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    // Insert a new program into the database
    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO program (name, help_command, help_text) VALUES (?1, ?2, ?3)",
            params![self.name, self.commands.join(";"), self.help_text],
        )?;
        Ok(())
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Program>> {
        let mut stmt = conn.prepare("SELECT * FROM program")?;
        let program_iter = stmt.query_map([], |row| {
            Ok(Program {
                id: row.get(0)?,
                name: row.get(1)?,
                commands: row
                    .get::<_, String>(2)?
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
                help_text: row.get(3)?,
            })
        })?;

        let mut programs = Vec::new();
        for program in program_iter {
            programs.push(program?);
        }

        Ok(programs)
    }

    // Get a program by ID
    pub fn get_by_id(conn: &Connection, id: i32) -> Result<Option<Program>> {
        let mut stmt =
            conn.prepare("SELECT id, name, help_command, help_text FROM program WHERE id = ?1")?;
        let program_iter = stmt.query_map([id], |row| {
            Ok(Program {
                id: row.get(0)?,
                name: row.get(1)?,
                commands: row
                    .get::<_, String>(2)?
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
                help_text: row.get(3)?,
            })
        })?;

        for program in program_iter {
            return Ok(Some(program?));
        }

        Ok(None)
    }

    // Get a program by name
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Program>> {
        let mut stmt =
            conn.prepare("SELECT id, name, help_command, help_text FROM program WHERE name = ?1")?;
        let program_iter = stmt.query_map([name], |row| {
            Ok(Program {
                id: row.get(0)?,
                name: row.get(1)?,
                commands: row
                    .get::<_, String>(2)?
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
                help_text: row.get(3)?,
            })
        })?;

        for program in program_iter {
            return Ok(Some(program?));
        }

        Ok(None)
    }

    // Update a program's details in the database
    pub fn update(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "UPDATE program SET name = ?1, help_command = ?2, help_text = ?3 WHERE id = ?4",
            params![
                self.name,
                self.commands.join(";"),
                self.help_text,
                self.id.unwrap()
            ],
        )?;
        Ok(())
    }

    // Delete a program by ID
    pub fn delete(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("DELETE FROM program WHERE id = ?1", params![id])?;
        Ok(())
    }
}
