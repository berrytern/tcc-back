pub fn format_date(mut value: String) -> String {
    let mut found = false;
    if value.len() == 32 {
        let mut a = String::from("");
        a.push_str(&value[0..11]);
        a.push_str("0");
        a.push_str(&value[11..23]);
        value = a;
    }
    value[0..24].chars()
    .map(|c| {
        if c == ' ' && !found {
            found = true;
            'T'
        } else if c == ' ' && found {
            'Z'
        } else {
            c
        }
    })
    .collect::<String>()
}