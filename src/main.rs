
#[derive(Debug)]
enum Token {
    WS,
    DQUOTE,
    DOT,
    DOLLAR,
    OBRACK,
    CBRACK,
    NL,
    WORD(String),
}

fn main() {
    
    //let s = "  \ton confirm log \"Request ${req.id} confirmed.\"\t";
    let s = "on confirm { \nlog \"Request ${re.id} confirmed\"}";
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
                    if !p.is_alphabetic() {
                        break;
                    }
                    word.push(*p);
                    it.next();
                }
                println!( "WORD {:?}", Token::WORD(word.clone()) );
            },
            _ => {
                println!("{:?}", c);
            },
        }
        o = it.next();
    }
    
}

