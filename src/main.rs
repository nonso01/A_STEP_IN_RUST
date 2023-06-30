#![allow(dead_code)]

// enums vs structs, learning about enums and special Option
#[derive(Debug)]

enum JavascriptTool {
    Angular(String),
    React(String),
    Vue(String),
    Svelte(String),
    TypeScript(String),
    // i see typescript as a tool and not a language, lol typescript,
    // is written in js for js
    Grunt(String),
    Gulp(String),
    AnyOther(String),
}

fn main() {
    let tool_info = String::from(
        "React is extremly user friendly when compared to vue, but vue and svelte are more powerful based on some concepts"
        );

    let my_best_tool = JavascriptTool::React(tool_info);
    // since we aren't referencing ownership will be taken
    alert_tool(my_best_tool);
}

// i just want to understand how match works
fn alert_tool(tool: JavascriptTool) -> u8 {
    match tool {
        // just like arrow fn
        // const foo = (param) => baa in js
        JavascriptTool::React(_) => {
            println!("yeah React won");
                0
        }
        JavascriptTool::Vue(_) => {
            println!("Yeah vue is a great choice");
                1
        }
        _ => todo!(),
    }

    // we also have if let syntax :)
    /**
     * if let JavascriptTool::React(_) = tool {
     * println!("....");
     * } else {
     * }
     */
}


