use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // solução para o problema sem precisar me preocupar com caracteres unicode, no caso os testes com caractes japoneses e arabe
    // return input.chars().rev().collect();
    // versão para resolver o problema com caracteres unicode como um todo
    return input.graphemes(true).rev().collect::<String>();
}

