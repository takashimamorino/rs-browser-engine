struct Parser {
    position: usize,
    input: String,
}

impl Parser {
    // input が引数で受け取った文字列ではじまっているか判定
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    // 現在の文字を返して、position を 1 進める
    fn consume_char(&mut self) -> char {
        let mut iter = self.input(self.position..).char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_pos;
        cur_char
    }
}