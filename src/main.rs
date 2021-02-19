fn main() {

}

enum Op {
    plus,
    minus,
    mult,
    div,
}

enum NodeVal {
    Int(String),
    N(Box<Node>),
    Undefined,
}

struct Node {
    operator: Op,
    left: NodeVal,
    right: NodeVal,
}

fn math(exp: String) -> i32 {
        let tokens = exp.split(" ").collect::<Vec<&str>>();

        if tokens.len() == 0 {
            return -1;
        }

        if tokens.len() == 1 {
            return tokens[0].parse::<i32>().unwrap();
        }

        let mut masterNode = Node {operator: Op::plus, left: NodeVal:: Undefined, right: NodeVal:: Undefined};

        1


}