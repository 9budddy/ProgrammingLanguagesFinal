use crate::token::Token;
use crate::lexer_mockup::Lexer;
use crate::parse_tree::ParseTree;

pub fn brad_pratt() {

    // create a sequence of tokens that is assumed to
    //   be output of the lexer

    // a = !1 + 2 * b == 3 + 4 / 5 & 6 * 7
    // We get
    //  = a
    //      (&
    //          (==
    //              (+ (!1)
    //                  (* 2 b)
    //              )
    //              (+ 3
    //                  (/ 4 5)
    //              )
    //          )
    //          (* 6 7)
    //      )

    let tokens = vec![
        Token::ID(String::from("a")),
        Token::OP_ASSIGN,
        Token::OP_NOT,
        Token::LIT_I32(1),
        Token::OP_ADD,
        Token::LIT_I32(2),
        Token::OP_MUL,
        Token::ID(String::from("b")),
        Token::OP_EQUAL,
        Token::LIT_I32(3),
        Token::OP_ADD,
        Token::LIT_I32(4),
        Token::OP_DIV,
        Token::LIT_I32(5),
        Token::OP_AND_BIT,
        Token::LIT_I32(6),
        Token::OP_MUL,
        Token::LIT_I32(7),

    ];

    // create Pratt parser
    let lexer = Lexer::new(tokens);
    let mut parser = PrattParser::new(lexer);

    // start Pratt top-down operator precedence parsing
    let tree = parser.analyze();

    // print parse tree
    tree.print();

}

struct PrattParser {
    lexer: Lexer,
}

impl PrattParser {
    fn new(lexer : Lexer) -> PrattParser {
        PrattParser { lexer }
    }

    fn analyze(&mut self) -> ParseTree {
        self.pratt_driver(Token::EOI.right_bp() )
    }

    fn pratt_driver(&mut self, requested_bp: i32) -> ParseTree {
        let mut current_token = self.current(); // ID =
        self.advance();
        let mut left_denotation = self.func_prefix(current_token);
        loop {
            current_token = self.current();
            // compare binding powers
            if requested_bp >= current_token.left_bp() {
                // finish subexpression (requested rbp >= curr lbp)
                return left_denotation;
            }
            // go on with subexpression (requested rbp < curr lbp)
            self.advance();
            left_denotation = self.func_infix(current_token, left_denotation);
        }
    }

    pub fn func_prefix(&mut self, token: Token) -> ParseTree {
        match token {
            Token::ID(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_I32(_) => {
                ParseTree::new(token.clone())
            }
            Token::OP_NOT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(right_denotation);
                node
            }
            Token::EOI => {
                ParseTree::new(token.clone())
            }
            _ => {
                panic!("Missing prefix function for token {:?}", token);
            }
        }
    }

    fn func_infix(&mut self, token: Token, left_denotation : ParseTree) -> ParseTree {
        match token {
            Token::OP_ADD => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_SUB => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_DIV => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_MUL => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_EQUAL => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_ASSIGN => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_AND_BIT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_OR_BIT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_GT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }
            Token::OP_LT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                node
            }

            Token::EOI => { todo!() }
            _ => {
                panic!("Missing infix function for token {:?}", token);
            }
        }
    }
}


impl PrattParser { // utility functions for lexer

    fn current(&mut self) -> Token {
        self.lexer.current()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

}


impl Token {

    fn binding_powers(token : &Token) -> (i32, i32) {
        match token {
            Token::OP_NOT => (13,14),
            Token::ID(_) => (12,12),
            Token::LIT_I32(_) => (12,12),
            Token::OP_DIV => (10,11),
            Token::OP_MUL => (10,11),
            Token::OP_SUB => (8,9),
            Token::OP_ADD => (8,9),
            Token::OP_LT => (6,7),
            Token::OP_GT => (6,7),
            Token::OP_EQUAL => (4,5),
            Token::OP_AND_BIT => (2,3),
            Token::OP_OR_BIT => (2,3),
            Token::OP_ASSIGN => (2,1),
            Token::EOI => (0,0),
            _ => {
                panic!("Missing binding powers for token {:?}", token);
            }
        }
    }

    fn left_bp(&self) -> i32 { Token::binding_powers(self).0 } //left bp
    fn right_bp(&self) -> i32 { Token::binding_powers(self).1 } //right bp

}


