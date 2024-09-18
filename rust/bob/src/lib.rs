pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    let is_empty = trimmed_message.is_empty();
    let is_question = trimmed_message.ends_with('?');
    let is_shouting = trimmed_message.chars().any(char::is_alphabetic) && trimmed_message.to_uppercase() == trimmed_message;

    if is_empty {
        "Fine. Be that way!"
    } else if is_question && is_shouting {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_shouting {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
