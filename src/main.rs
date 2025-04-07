
#[derive(Debug)]
enum Token {
    WS,
    DQUOTE,
    DOT,
    DOLLAR,
    OBRACK,
    CBRACK,
    NL,
    CHAR(char),
    WORD(String),
}

fn lex<I>(it:I)
    where I: Iterator<Item = char>,
{



}

fn main() {
    
    let s = "on confirm { 
log \"Request ${re.id1} confirmed!\"}";
    let mut word = String::new();
    
    let mut it = s.chars().peekable();
    
    let mut o = it.next();
    
    loop {
        if o.is_none() {
            break;
        }
        
        let c = o.unwrap();
            
        match c {
            '\n' => {
                println!("{:?}", Token::NL);
            },
            ' '|'\t' => {
                println!("{:?}",Token::WS);
            },
            '$' => {
                println!("{:?}",Token::DOLLAR);
            },
            '{' => {
                println!("{:?}",Token::OBRACK);
            },
            '}' => {
                println!("{:?}",Token::CBRACK);
            },
            _ if c == '.' => {
                println!("{:?}",Token::DOT);
            },
            _ if c == '"' => {
                println!("{:?}",Token::DQUOTE);
            },
            _ if c.is_alphabetic() => {
                word.clear();
                word.push(c);
                while let Some(p) = it.peek() {
                    if !p.is_alphanumeric() {
                        break;
                    }
                    word.push(*p);
                    it.next();
                }
                println!( "{:?}", Token::WORD(word.clone()) );
            },
            _ => {
                println!("{:?}", Token::CHAR(c));
            },
        }
        o = it.next();
    }
    
}

