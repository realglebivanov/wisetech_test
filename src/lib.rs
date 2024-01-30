use anyhow::anyhow;

struct State<'a> {
    depth: usize,
    width: usize,
    line_width: usize,
    is_first_line: bool,
    lookbehind: Vec<&'a str>,
    acc: String,
}

impl<'a> State<'a> {
    fn new(line_width: usize) -> Self {
        Self {
            depth: 0,
            width: 0,
            line_width,
            is_first_line: true,
            lookbehind: Vec::with_capacity(line_width),
            acc: String::with_capacity(line_width * 10),
        }
    }

    fn consume_line(&mut self) {
        let spacing_left = self.line_width - self.width;
        let (even_spacing, odd_spacer_count) = if self.depth == 1 {
            (spacing_left, 0)
        } else {
            (
                spacing_left / (self.depth - 1),
                spacing_left % (self.depth - 1),
            )
        };

        if !self.is_first_line {
            self.acc.push_str("\n");
        }

        self.lookbehind
            .drain(0..self.depth)
            .enumerate()
            .for_each(|(index, word)| {
                self.acc.extend(word.chars());

                for _ in 0..even_spacing {
                    self.acc.push(' ');
                }

                if index < odd_spacer_count {
                    self.acc.push(' ');
                }
            });

        self.is_first_line = false;
        self.depth = 0;
        self.width = 0;
    }
}

pub fn transform(input: &str, line_width: u32) -> anyhow::Result<String> {
    let line_width: usize = line_width
        .try_into()
        .or(Err(anyhow!("Too big of a line_width")))?;

    let mut state = input
        .split(" ")
        .filter(|w| !w.is_empty())
        .try_fold(State::new(line_width), folder)?;

    if state.depth > 0 {
        state.consume_line();
    }

    return Ok(state.acc);
}

fn folder<'a>(mut state: State<'a>, word: &'a str) -> anyhow::Result<State<'a>> {
    let char_count = word.chars().count();
    let width_delta = if state.lookbehind.is_empty() {
        char_count
    } else {
        char_count + 1
    };

    if char_count >= state.line_width {
        return Err(anyhow!("Unexpected word size (longer than line_width)"));
    }

    if state.width + width_delta > state.line_width {
        state.consume_line();
    }

    state.lookbehind.push(word);
    state.depth += 1;
    state.width += width_delta;

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);

            let transformed = transform(input, line_width).unwrap();

            assert_eq!(transformed, expected);
        }
    }
}
