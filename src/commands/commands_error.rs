use std::path::Path;


#[derive(Debug, thiserror::Error)]
pub enum CommandsError {
	#[error("Improper characters were used")]
	ImproperCharactersUsed,
	#[error("The user needs to run this command in a python project")]
	NotInProjectDirectory,

} impl CommandsError {
    pub fn in_project_directory() -> Result<(), CommandsError> {
    	if !Path::new("./main.py").exists() || !Path::new("./venv/").exists() {
    		Err(CommandsError::NotInProjectDirectory)
    	} else {
    		Ok(())
    	}
    }

    pub fn proper_characters_are_used(project_name: &str) -> Result<(), CommandsError> {
    	for character in project_name.chars() { match character {
    		'#' => return Err(CommandsError::ImproperCharactersUsed),
    		'%' => return Err(CommandsError::ImproperCharactersUsed),
    		'&' => return Err(CommandsError::ImproperCharactersUsed),
    		'{' => return Err(CommandsError::ImproperCharactersUsed),
    		'}' => return Err(CommandsError::ImproperCharactersUsed),
    		'<' => return Err(CommandsError::ImproperCharactersUsed),
    		'>' => return Err(CommandsError::ImproperCharactersUsed),
    		'*' => return Err(CommandsError::ImproperCharactersUsed),
    		'?' => return Err(CommandsError::ImproperCharactersUsed),
    		'/' => return Err(CommandsError::ImproperCharactersUsed),
    		'$' => return Err(CommandsError::ImproperCharactersUsed),
    		'!' => return Err(CommandsError::ImproperCharactersUsed),
    		'"' => return Err(CommandsError::ImproperCharactersUsed),
    		':' => return Err(CommandsError::ImproperCharactersUsed),
    		'@' => return Err(CommandsError::ImproperCharactersUsed),
    		'+' => return Err(CommandsError::ImproperCharactersUsed),
    		'`' => return Err(CommandsError::ImproperCharactersUsed),
    		'|' => return Err(CommandsError::ImproperCharactersUsed),
    		'=' => return Err(CommandsError::ImproperCharactersUsed),
    		 _  => return Ok(())
    	}};
    	Ok(())
    }
}
