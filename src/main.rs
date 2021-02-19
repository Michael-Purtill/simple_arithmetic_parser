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
        let tokens = exp.split(" ");

        if (tokens.len() == 0) {
            return -1;
        }

        if (tokens.len() == 1) {
            // return tokens[0].parse::<132>().unwrap();
        }

        // let mut masterNode = match tokens[2] {
        //     "+" => if tokens.len() >= 4 {Node {op: Op::plus} }
        // }

        1


}