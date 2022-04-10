pub fn reply(message: &str) -> &str {
    fn is_nothing(x: &str) -> bool {
        x.trim().is_empty()
    }
    fn is_question(x: &str) -> bool {
        x.trim().ends_with('?')
    }
    fn is_shout(x: &str) -> bool {
        x.trim().to_uppercase() == x && x.trim().to_uppercase() != x.trim().to_lowercase()
    }

    match message {
        m if is_nothing(m) => "Fine. Be that way!",
        m if is_question(m) && !is_shout(m) => "Sure.",
        m if is_question(m) && is_shout(m) => "Calm down, I know what I'm doing!",
        m if !is_question(m) && is_shout(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
