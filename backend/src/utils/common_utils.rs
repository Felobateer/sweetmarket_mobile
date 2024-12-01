use std::collections::HashMap;


pub fn get_placeholder_string_for_array(arr: &[impl std::fmt::Display]) -> Result<String, String> {
    if arr.is_empty() {
        return Err("Invalid input: array is empty".to_string());
    }

    // Create a string of placeholders
    let placeholders = vec!["?"; arr.len()].join(", ");
    Ok(placeholders)
}


pub fn multiple_column_set(object: &HashMap<String, String>) -> Result<(String, Vec<String>), String> {
    if object.is_empty() {
        return Err("Invalid input: object is empty".to_string());
    }

    // Map the keys to SQL column assignments
    let column_set = object
        .keys()
        .map(|key| format!("{} = ?", key))
        .collect::<Vec<String>>()
        .join(", ");

    // Collect the values in order
    let values = object.values().cloned().collect::<Vec<String>>();

    Ok((column_set, values))
}
