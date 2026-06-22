use crate::core::tasks::Status;

pub fn parse_status(input: &str) -> Result<Status, String> {
    let normalized = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    match normalized.as_str() {
        "todo" => Ok(Status::Todo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(format!("Invalid status: '{}'. Valid options are: todo, in-progress, done", input)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_todo_lowercase() {
        assert_eq!(parse_status("todo").unwrap(), Status::Todo);
    }

    #[test]
    fn parse_todo_uppercase() {
        assert_eq!(parse_status("TODO").unwrap(), Status::Todo);
    }

    #[test]
    fn parse_todo_mixed_case() {
        assert_eq!(parse_status("To Do").unwrap(), Status::Todo);
        assert_eq!(parse_status("tO-dO").unwrap(), Status::Todo);
    }

    #[test]
    fn parse_todo_with_hyphen() {
        assert_eq!(parse_status("to-do").unwrap(), Status::Todo);
    }

    #[test]
    fn parse_inprogress_lowercase() {
        assert_eq!(parse_status("inprogress").unwrap(), Status::InProgress);
    }

    #[test]
    fn parse_inprogress_with_space() {
        assert_eq!(parse_status("in progress").unwrap(), Status::InProgress);
    }

    #[test]
    fn parse_inprogress_with_hyphen() {
        assert_eq!(parse_status("in-progress").unwrap(), Status::InProgress);
    }

    #[test]
    fn parse_inprogress_uppercase() {
        assert_eq!(parse_status("IN-PROGRESS").unwrap(), Status::InProgress);
    }

    #[test]
    fn parse_done_lowercase() {
        assert_eq!(parse_status("done").unwrap(), Status::Done);
    }

    #[test]
    fn parse_done_uppercase() {
        assert_eq!(parse_status("DONE").unwrap(), Status::Done);
    }

    #[test]
    fn parse_invalid_returns_error() {
        let result = parse_status("tod");
        assert!(result.is_err());
    }

    #[test]
    fn parse_random_string_returns_error() {
        let result = parse_status("invalid");
        assert!(result.is_err());
    }
}
