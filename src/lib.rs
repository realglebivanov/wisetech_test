use anyhow::anyhow;

struct State<'a> {
    width: usize,
    line_width: usize,
    is_first_line: bool,
    lookbehind: Vec<&'a str>,
    acc: String,
}

impl<'a> State<'a> {
    fn new(line_width: usize) -> Self {
        Self {
            width: 0,
            line_width,
            is_first_line: true,
            lookbehind: Vec::with_capacity(line_width),
            acc: String::with_capacity(line_width * 4 * 10),
        }
    }

    #[inline(always)]
    fn add_word(mut self, word: &'a str) -> anyhow::Result<Self> {
        let char_count = word.chars().count();
        let width_delta = char_count + 1;

        if char_count >= self.line_width {
            return Err(anyhow!("Unexpected word size (longer than line_width)"));
        }

        if self.width + width_delta - 1 > self.line_width {
            self.consume_line();
        }

        self.lookbehind.push(word);
        self.width += width_delta;

        Ok(self)
    }

    #[inline(always)]
    fn consume_line(&mut self) {
        let spacing_left = self.line_width - (self.width - 1);
        let init_length = self.lookbehind.len() - 1;

        let (even_spacing, odd_spacer_count) = if self.lookbehind.len() == 1 {
            (spacing_left - 1, 0)
        } else {
            (spacing_left / init_length, spacing_left % init_length)
        };

        if !self.is_first_line {
            self.acc.push_str("\n");
        }

        self.lookbehind
            .drain(0..self.lookbehind.len())
            .enumerate()
            .for_each(|(i, word)| {
                self.acc.push_str(word);

                if i == 0 || i < init_length {
                    self.acc.push(' ');

                    for _ in 0..even_spacing {
                        self.acc.push(' ');
                    }
                }

                if i < odd_spacer_count {
                    self.acc.push(' ');
                }
            });

        self.is_first_line = false;
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
        .try_fold(State::new(line_width), State::add_word)?;

    if state.lookbehind.len() > 0 {
        state.consume_line();
    }

    return Ok(state.acc);
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
            ("象 形 字", 8, "象   形  字")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);

            let transformed = transform(input, line_width).unwrap();

            assert_eq!(transformed, expected);
        }
    }
}
