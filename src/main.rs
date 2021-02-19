fn main() {

}

enum NodeVal {
    Int(String),
    N(Box<Node>),
    Undefined,
}

struct Node {
    operator: String,
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

        let mut masterNode = Node {operator:"".to_string(), left: NodeVal:: Undefined, right: NodeVal:: Undefined};

        if tokens.len() == 3 {
            masterNode.operator = tokens[1].to_string();
            masterNode.left = NodeVal::Int(tokens[0].to_string());
            masterNode.right = NodeVal::Int(tokens[2].to_string());

            return evalNode(masterNode);
        }
        else {
            for i in 0..tokens.len() {

            }
        }

        1


}

fn evalNode(node: Node) -> i32 {
    return 1;
}