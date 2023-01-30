#![allow(dead_code)]

use std::{str::Chars, vec, process::exit};

fn unexpected(iter: Chars, value: char,context: &str, expected: &str) -> ! {
    iter.for_each(|e|print!("{e}"));
    println!("\nUnexpected {context}: {value}\nExpecting {expected}");
    exit(1);
}
fn unexpected_str(value: &str,context: &str, expected: &str) -> ! {
    println!("Unexpected {context}: {value}\nExpecting {expected}");
    exit(1);
}


#[derive(Debug)]
struct Root {
    tag: String,
    attr: Vec<(String,String)>,
    childs: Vec<Element>
}
#[derive(Debug)]
struct Element {
    tag: String,
    attr: Vec<(String,String)>,
    scope: u16,
    inner: String,
    relation: Vec<Element>
}
impl Root {
    fn new() -> Root {
        Self { tag: "root".to_string(), attr: vec![], childs: vec![] }
    }
}
impl Element {
    fn new() -> Element {
        Self { tag: "".to_string(), attr: vec![], scope: 0, inner: String::new(), relation: vec![] }
    }
}



fn main() {
    let input = 
    "<section className=\"m-6 p-4 bg-white shadow-md rounded-md\"><main>Main article<aside>aside bar</aside></main><h1 className=\"text-4xl font-bold\">Title</h1><button>Count : {count}</button></section>";
    
    parse(input);
}
fn parse(input: &str) {
    let chars = input.chars();
    let mut root_element = Root::new();
    
    root(chars, &mut root_element);
    println!("{:#?}",root_element);
}


// new root
fn root(mut iter: Chars, data: &mut Root){
    let subject = Element::new();
    match iter.next() {
        Some('<') => {
            let mut subject = identifier(iter, vec![subject]);
            
            let parent = {
                let mut state = subject.pop().unwrap();
                while !subject.is_empty() {
                    let mut parent = subject.pop().unwrap();
                    parent.relation.push(state);
                    state = parent;
                }
                state
            };
            
            data.childs.push(parent);
        },
        Some(er) => unexpected(iter, er, "symbol", "<"),
        None => return,
    };
}

fn identifier(mut iter: Chars, mut subject: Vec<Element>) -> Vec<Element> {
    let mut id = String::new();
    loop {
        match iter.next() {
            // found '/', expecting close tag, 
            Some('/') => {
                let mut closing_tag = String::new();
                loop {
                    match iter.next() {
                        
                        
                        // found '>' or "whitespace", then:
                        Some('>') => {
                            // if tag match with current subject, closing subject, then:
                            if closing_tag == subject.last_mut().unwrap().tag {
                                // if its the top most element, done!
                                if subject.last_mut().unwrap().scope == 0 { return subject; }
                                
                                // go upper scope and move to "new_tag" state
                                let child = subject.pop().unwrap();
                                subject.last_mut().unwrap().relation.push(child);
                                return new_tag(iter, subject)
                            }
                            unexpected_str(&closing_tag, "closing identifier tag", &subject.last_mut().unwrap().tag)
                        },
                        
                        
                        // found identifier, keep collecting
                        Some(chr) => closing_tag.push(chr),
                        None => return subject,
                    }
                }
            },
            
            // found identifier, keep collecting
            Some(chr) if chr.is_alphanumeric() || chr == '-' => id.push(chr),
            
            
            // found whitespace, expecting attribute, move to "attribute" state
            Some(chr) if !id.is_empty() => {
                let mut new_element = Element::new();
                new_element.tag = id;
                new_element.scope = subject.last_mut().unwrap().scope + 1;
                
                subject.push(new_element);
                
                if chr.is_whitespace() {
                    return attribute(iter, subject)
                }
                else if chr == '>'{
                    return inner(iter, subject)
                }
                else {
                    unexpected(iter, chr, "identifier", "alphabetical or -")
                }
            },
            
            Some(er) => unexpected(iter, er, "identifier", "alphabetical or -"),
            None => return subject,
        }
    }
}


fn attribute(mut iter: Chars, mut subject: Vec<Element>) -> Vec<Element> {
    let mut key = String::new();
    let mut val = String::new();
    let mut is_key = true;
    
    loop {
        match iter.next() {
            // found `>` while in "key" context, no attribute provide, move to "inner" state
            Some('>') if is_key => {
                return inner(iter, subject)
            },
            
            // found alphabetic while in "key" context, keep collecting
            Some(chr) if is_key && chr.is_alphabetic() => key.push(chr),
            
            // found `=` and `"` after while in "key" context, change to "value" context
            Some('=') if is_key && iter.next() == Some('"') => { is_key = false; },
            
            // found any but `"` while in "value" context, keep collecting
            Some(chr) if !is_key && chr != '"' => val.push(chr),
            
            // found `"` while in "value" context, then:
            Some('"') if !is_key => {
                
                // assign to context
                if !key.is_empty() && !val.is_empty() {
                    subject.last_mut().unwrap().attr.push((key.clone(),val.clone()));
                    key.clear();
                    val.clear();
                }
                
                let next = match iter.next().clone() {
                    Some(r) => r,
                    None => return subject,
                };
                
                // found "whitespace", change to "key" context
                if next.is_whitespace() { is_key = true; }
                
                // found `<`, move to "inner" state
                else if next == '>' {
                    return inner(iter, subject)
                } else {
                    unexpected(iter, next, "post attribute", "whitespace or >")
                }
            },
            
            Some(er) => unexpected(iter, er, "attribute key", "alphabetical, key identifier"),
            None => return subject,
        }
    };
}



fn inner(mut iter: Chars, mut subject: Vec<Element>) -> Vec<Element> {
    let mut inner = String::new();
    
    loop {
        match iter.next() {
            
            // found `<`, add the inner, move to "identifier" state
            Some('<') => {
                subject.last_mut().unwrap().inner = inner;
                
                return identifier(iter, subject)
            },
            
            // found anything, keep collecting
            Some(chr) => inner.push(chr),
            None => return subject,
        }
    }
}

fn new_tag(mut iter: Chars, subject: Vec<Element>) -> Vec<Element> {
    // let mut new_element = Element::new();
    match iter.next() {
        Some('<') => {
            return identifier(iter, subject)
        },
        Some(er) => unexpected(iter, er, "symbol", "<"),
        None => return subject,
    };
}
