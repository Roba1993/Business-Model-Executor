extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    let mut stream = item.clone().into_iter();
    let mut id = String::from("");
    let mut name = String::from("");
    let mut description = String::from("");
    let mut typ = String::from("Static");
    let mut inputs : Vec<(String, String)> = vec!();
    let mut outputs : Vec<String> = vec!();
    let mut code : proc_macro::TokenStream = "{ }".parse().unwrap();

    loop {
        match stream.next() {
            None => { break; },
            Some(token) => { 
                println!("{:?}", token);
                let attr = get_ident(&token).expect("No ident given").to_string();

                match attr.as_str() {
                    "id" => { id = get_id(&mut stream); },
                    "name" => { name = get_name(&mut stream); },
                    "description" => { description = get_description(&mut stream); },
                    "typ" => { typ = get_typ(&mut stream); },
                    "fn" => { 
                        get_execute(&mut stream);
                        inputs = get_inputs(&mut stream);
                        get_sign(&mut stream);
                        outputs = get_outputs(&mut stream);
                        code = get_code(&mut stream);
                    },
                    _ => { break; panic!("Unknown attribute defined"); }
                }
            }
        }
    }

    let mut inp_str = String::from("");
    for s in &inputs {
        inp_str.push_str(&format!("\"{}\",", s.1));
    }

    let mut out_str = String::from("");
    for o in &outputs {
        out_str.push_str(&format!("\"{}\",", o));
    }

    let mut fn_inp_str = String::from("");
    for (i, s) in inputs.iter().enumerate() {
        fn_inp_str.push_str(&format!("let {} : {} = _private_input.get({}).ok_or(\"Argument not provided\")?{}; \n", s.0, s.1, i, get_value_typ(&s.1)));
    }

    let mut fn_out_str = String::from("");
    for (i, s) in outputs.iter().enumerate() {
        fn_out_str.push_str(&format!("out.push(bme::Register{{ block_id: _private_block_id, node_id: {}, value: {} }});", (i*2)+3, get_bme_value(s, i)));
    }

    println!("id: {}", id);
    println!("name: {}", name);
    println!("description: {}", description);
    println!("inputs: {:?}", inputs);
    println!("outputs: {:?}", outputs);
    println!("code: {:?}", code);

    /*for i in item.into_iter() {
        println!("{:?}", i);
    }*/

    let c = format!(r#"
        #[derive(Debug)]
        pub struct {name} {{}}
        impl bme::ExecutionBlock for {name} {{
            fn get_id(&self) -> u32 {{
                {id}
            }}

            fn get_name(&self) -> &'static str {{
                "{name}"
            }}

            fn get_type(&self) -> bme::ExecutionBlockType {{
                bme::ExecutionBlockType::{typ}
            }}

            fn get_inputs(&self) -> &'static [&'static str] {{
                &[{inp_str}]
            }}

            fn get_outputs(&self) -> &'static [&'static str] {{
                &[{out_str}]
            }}

            fn execute(&self, input: Vec<bme::Register>, block_id: u32) -> Result<Vec<bme::Register>, bme::error::Error> {{
                let _private_input = input.into_iter().map(|r| r.value).collect::<Vec<Value>>();
                let _private_block_id = block_id;

                {fn_inp_str}

                // execute the block
                let result = {{ {code} }};


                let mut out = vec!();

                {fn_out_str}

                Ok(out)
            }}
        }}
    "#, name=name, id=id, typ=typ, inp_str=inp_str, out_str=out_str, fn_inp_str=fn_inp_str, fn_out_str=fn_out_str, code=code.to_string());

    println!("{}", c);

    c.parse().unwrap()
}


fn get_ident(inp: &proc_macro::TokenTree) -> Option<proc_macro::Ident> {
    match inp {
        proc_macro::TokenTree::Ident(i) => Some(i.clone()),
        _ => None,
    }
}

fn get_literal(inp: &proc_macro::TokenTree) -> Option<proc_macro::Literal> {
    match inp {
        proc_macro::TokenTree::Literal(i) => Some(i.clone()),
        _ => None,
    }
}

fn get_punct(inp: &proc_macro::TokenTree) -> Option<proc_macro::Punct> {
    match inp {
        proc_macro::TokenTree::Punct(i) => Some(i.clone()),
        _ => None,
    }
}

fn get_group(inp: &proc_macro::TokenTree) -> Option<proc_macro::Group> {
    match inp {
        proc_macro::TokenTree::Group(i) => Some(i.clone()),
        _ => None,
    }
}

fn get_id(stream: &mut proc_macro::token_stream::IntoIter) -> String {
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    let res = get_literal(&stream.next().expect("No id given")).expect("No id given").to_string();
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    res
}

fn get_name(stream: &mut proc_macro::token_stream::IntoIter) -> String {
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    let res = get_ident(&stream.next().expect("No name given")).expect("No name given").to_string();
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    res
}

fn get_description(stream: &mut proc_macro::token_stream::IntoIter) -> String {
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    let res = get_literal(&stream.next().expect("No name given")).expect("No name given").to_string();
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    res
}

fn get_typ(stream: &mut proc_macro::token_stream::IntoIter) -> String {
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    let res = get_ident(&stream.next().expect("No name given")).expect("No name given").to_string();
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    res
}

fn get_execute(stream: &mut proc_macro::token_stream::IntoIter)  {
    // get function name execute
    if get_ident(&stream.next().expect("No function name found")).expect("No function name found").to_string() != "execute" {
        panic!("No function name execution found");
    }
}

fn get_inputs(stream: &mut proc_macro::token_stream::IntoIter) -> Vec<(String, String)> {
    let mut inputs = vec!();

    let mut attr_stream = get_group(&stream.next().expect("Function inputs defined")).expect("Function inputs defined").stream().into_iter();
    loop {
        match attr_stream.next() {
            None => { break; },
            Some(a) => { 
                // get the first attribute
                let mut attr = a.to_string();

                // when the first attribute is a seperator try again
                if attr == "," {
                    attr = attr_stream.next().expect("No Value for attribute").to_string();
                }

                // get the seperator
                get_punct(&attr_stream.next().expect("No seperator found")).expect("No seperator found");

                // get the value
                let value = attr_stream.next().expect("No Value for attribute").to_string();

                // push name and value
                inputs.push((attr, value));
            }
        }
    }

    inputs
}

fn get_outputs(stream: &mut proc_macro::token_stream::IntoIter) -> Vec<String> {
    let mut outputs = vec!();

    // get the inputs of the function
    for i in get_group(&stream.next().expect("Function inputs not defined")).expect("Function inputs not defined").stream().into_iter() {
        outputs.push(i.to_string());
    }

    outputs
}


fn get_sign(stream: &mut proc_macro::token_stream::IntoIter) {
    // get -> sign
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
    get_punct(&stream.next().expect("No seperator found")).expect("No seperator found");
}

fn get_code(stream: &mut proc_macro::token_stream::IntoIter) -> proc_macro::TokenStream {
    // get the code
    get_group(&stream.next().expect("Function code not defined")).expect("Function code not defined").stream()
}

fn get_value_typ(typ: &String) -> String {
    match typ.as_str() {
        "String" => String::from(".get_string().ok_or(\"Value is not a String\")?"),
        "i64" => String::from(".get_integer().ok_or(\"Value is not a Integer\")?"),
        _ => panic!("Only String & i64 parameters are supported")
    }
}

fn get_bme_value(typ: &String, value: usize) -> String {
    match typ.as_str() {
        "String" => format!("bme::Value::String(result.{})", value),
        "i64" => format!("bme::Value::Integer(result.{})", value),
        _ => panic!("Only String & i64 parameters are supported")
    }
}