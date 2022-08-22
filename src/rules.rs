pub fn remove_useless_spaces_around_colon(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in lines {
        let new_line;
        if line.contains(':') {
            let chars_to_check: Vec<char> = line.trim_start().chars().collect();
            let mut collected_chars: Vec<char> = Vec::new();

            let mut quote_char: Option<char> = None; // if some, then means that string started, allowed values '`"
            for item in chars_to_check {
                if item == ' ' {
                    if quote_char.is_some() || collected_chars.last() != Some(&' ') {
                        collected_chars.push(' ');
                    }
                } else if item == ':' {
                    if quote_char.is_none() && collected_chars.last() == Some(&' ') {
                        collected_chars.pop();
                    }
                    collected_chars.push(item);
                } else if item == '"' || item == '\'' || item == '`' {
                    if collected_chars.last() != Some(&'\\') {
                        if quote_char.is_none() {
                            quote_char = Some(item);
                        } else if Some(item) == quote_char {
                            quote_char = None;
                        }
                    }
                    collected_chars.push(item);
                } else {
                    collected_chars.push(item)
                }
            }

            new_line = collected_chars.into_iter().collect();
        } else {
            new_line = line;
        }

        new_lines.push(new_line);
    }
    new_lines
}

// Function
pub fn move_elements_inside(lines: Vec<String>) -> Vec<String> {
    let mut current_bracket_number = 0u32;
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines {
        if current_bracket_number == 0 {
            new_lines.push(line.clone());
        } else {
            let mut new_line = "".to_string();
            let spaces_to_add = if line.trim().starts_with("}") {
                current_bracket_number - 1
            } else {
                current_bracket_number
            };
            for _ in 0..spaces_to_add {
                new_line.push_str("    ") // 4 spaces is default for QML
            }
            new_line.push_str(&line);
            new_lines.push(new_line.trim_end().to_string()); // Trimming, because empty line with only spaces could be here
        }

        if line.contains('{') {
            current_bracket_number += 1;
        }
        if line.contains('}') {
            if current_bracket_number == 0 {
                println!("Mismatched number of {{ brackets, probably QML is broken");
            } else {
                current_bracket_number -= 1;
            }
        }
    }
    new_lines
}

pub fn remove_empty_space_on_end_of_line(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().map(|e| e.trim().to_string()).collect()
}

pub fn skip_start_end_empty_lines(mut lines: Vec<String>) -> Vec<String> {
    while !lines.is_empty() && lines[0].is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines[lines.len() - 1].is_empty() {
        lines.pop();
    }
    lines
}

pub fn move_single_open_bracket(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
    for line in lines {
        if line == "{" {
            while let Some(last_line) = collected_lines.last() {
                if last_line.is_empty() {
                    collected_lines.pop();
                } else {
                    break;
                }
            }
            if let Some(last_line) = collected_lines.last_mut() {
                last_line.push_str(" {");
            }
        } else {
            collected_lines.push(line);
        }
    }

    collected_lines
}

pub fn connect_multiple_empty_lines_into_one(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
    let mut was_empty = false;
    for line in lines {
        if line.is_empty() {
            if was_empty {
                continue;
            }
            was_empty = true;
        } else {
            was_empty = false;
        }
        collected_lines.push(line);
    }
    collected_lines
}

pub fn remove_empty_line_before_close_bracket(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
    for line in lines {
        if line == "}" && collected_lines.last() == Some(&"".to_string()) {
            collected_lines.pop();
        }
        collected_lines.push(line);
    }
    collected_lines
}

pub fn if_movement(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut find_oneliner = false;
    for line in lines {
        let mut new_line = "".to_string();
        if find_oneliner {
            new_line.push_str("    ");
            find_oneliner = false
        }

        new_line.push_str(&line);
        new_line = new_line.replace(" if(", " if (");
        new_line = new_line.replace(" else if(", " else if (");
        new_line = new_line.replace(" else {", " else {");
        new_lines.push(new_line.clone());

        let line_trimmed = new_line.trim();
        if (line_trimmed.starts_with("if (") || line_trimmed.starts_with("else if (")) && line_trimmed.ends_with(")") || line_trimmed == "else" {
            find_oneliner = true
        }
    }
    new_lines
}

pub fn space_before_bracket(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut quote_char: Option<char> = None; // if some, then means that string started, allowed values '`"
    for line in lines {
        if line.contains("{") {
            let mut new_line: Vec<char> = Vec::new();
            for charr in line.chars() {
                if charr == '"' || charr == '\'' || charr == '`' {
                    if new_line.last() != Some(&'\\') {
                        if quote_char.is_none() {
                            quote_char = Some(charr);
                        } else if Some(charr) == quote_char {
                            quote_char = None;
                        }
                    }
                    new_line.push(charr);
                } else if charr == '{' {
                    if quote_char.is_none() && new_line.last() != Some(&' ') && new_line.last() != None {
                        new_line.push(' ');
                    }
                    new_line.push(charr);
                } else {
                    new_line.push(charr);
                }
            }
            new_lines.push(new_line.iter().collect())
        } else {
            new_lines.push(line);
        }
    }
    new_lines
}

#[allow(unused)]
pub fn split_text_to_vector(text: &str) -> Vec<String> {
    text.split('\n').map(str::to_string).collect()
}