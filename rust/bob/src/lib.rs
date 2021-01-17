pub fn reply(message: &str) -> &str {
    if message.trim().ends_with("?") {
        if message.to_uppercase() == message {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else if message.trim() == "" {
        return "Fine. Be that way!";
    } else if message.to_uppercase() == message {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
