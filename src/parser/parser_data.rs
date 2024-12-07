/*
 * MIT License
 *
 * Copyright (c) 2023 Dylan Tuttle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use slotmap::{SlotMap, new_key_type};

use crate::{
    infrastructure::{error::ErrorReporter, log::Logger},
    scanner::scanner_data::Token,
    semantic_checker::semantic_checker_data::{Callback, DataType, Symbol, SymbolTable, Type},
};

pub type NodePointer = usize;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub root_node: Option<RootNodeKey>,

    pub root_nodes: SlotMap<RootNodeKey, RootNode>,

    pub variable_assignment_nodes: SlotMap<VariableAssignmentKey, VariableAssignment>,
    pub variable_declaration_nodes: SlotMap<VariableDeclarationKey, VariableDeclaration>,

    pub binary_nodes: SlotMap<BinaryKey, Binary>,
    pub unary_nodes: SlotMap<UnaryKey, Unary>,
    pub variable_nodes: SlotMap<VariableKey, Variable>,
    pub literal_nodes: SlotMap<LiteralKey, Literal>,

    pub identifier_nodes: SlotMap<IdentifierKey, Identifier>,
}

impl AST {
    pub fn new() -> AST {
        return AST {
            root_node: None,

            root_nodes: SlotMap::with_key(),

            variable_assignment_nodes: SlotMap::with_key(),
            variable_declaration_nodes: SlotMap::with_key(),

            binary_nodes: SlotMap::with_key(),
            unary_nodes: SlotMap::with_key(),
            variable_nodes: SlotMap::with_key(),
            literal_nodes: SlotMap::with_key(),

            identifier_nodes: SlotMap::with_key(),
        };
    }
}

// #[derive(Clone, Copy, PartialEq)]
// enum ASTNodeKey {
//     RootNode(RootNodeKey),
//     StatementOrExpression(StatementOrExpressionKey),
//     Identifier(IdentifierKey),
// }

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    // RootNode(RootNode),
    // ExpressionNode(ExpressionNode),
    // StatementNode(StatementNode),
    // IdentifierNode(IdentifierStatementNode),
    // TypeHintNode(TypeHintNode),
    // NotANode(NotANode),
}

impl ASTNode {
//     pub fn to_string(&self, ast: &AST) -> String {
//         match self {
//             ASTNode::RootNode(node) => return node.to_string(ast),
//             ASTNode::ExpressionNode(node) => return node.to_string(ast),
//             ASTNode::StatementNode(node) => return node.to_string(ast),
//             ASTNode::IdentifierNode(node) => return node.to_string(),
//             ASTNode::TypeHintNode(node) => return node.to_string(),
//             ASTNode::NotANode(node) => return node.to_string(),
//         }
//     }

//     pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
//         match self {
//             ASTNode::RootNode(node) => return node.to_string_with_level(level, ast),
//             ASTNode::ExpressionNode(node) => return node.to_string_with_level(level, ast),
//             ASTNode::StatementNode(node) => return node.to_string_with_level(level, ast),
//             ASTNode::IdentifierNode(node) => return node.to_string_with_level(level),
//             ASTNode::TypeHintNode(node) => return node.to_string_with_level(level),
//             ASTNode::NotANode(node) => return node.to_string_with_level(level),
//         }
//     }

//     pub fn node_to_string(&self) -> String {
//         match self {
//             ASTNode::RootNode(node) => return node.node_to_string(),
//             ASTNode::ExpressionNode(node) => return node.node_to_string(),
//             ASTNode::StatementNode(node) => return node.node_to_string(),
//             ASTNode::IdentifierNode(node) => return node.node_to_string(),
//             ASTNode::TypeHintNode(node) => return node.node_to_string(),
//             ASTNode::NotANode(node) => return node.node_to_string(),
//         }
//     }
// }

// impl Debug for ASTNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ASTNode::RootNode(_) => write!(f, "ASTNode::RootNode"),
//             ASTNode::ExpressionNode(_) => write!(f, "ASTNode::ExpressionNode"),
//             ASTNode::StatementNode(_) => write!(f, "ASTNode::StatementNode"),
//             ASTNode::IdentifierNode(_) => write!(f, "ASTNode::IdentifierNode"),
//             ASTNode::TypeHintNode(_) => write!(f, "ASTNode::TypeHintNode"),
//             ASTNode::NotANode(_) => write!(f, "ASTNode::NotANode"),
//         }
//     }
}


#[derive(Clone, PartialEq)]
pub enum RootNodeKey {
    REPLRootNodeKey(REPLRootNodeKey),
    FileRootNodeKey(FileRootNodeKey),
}


#[derive(Clone, PartialEq)]
pub enum RootNode {
    REPLRootNode(REPLRootNode),
    FileRootNode(FileRootNode),
}


new_key_type! { struct REPLRootNodeKey; }
#[derive(Clone, PartialEq)]
pub struct REPLRootNode {
    pub children: Vec<REPLConstruct>,
    pub children_start_at: usize,
}


impl REPLRootNode {
    pub fn new(children: Vec<REPLConstruct>) -> REPLRootNode {
        REPLRootNode { children, children_start_at: 0 }
    }
}





pub enum Traversal<'traversal> {
    Preorder(&'traversal mut Callback),
    Postorder(&'traversal mut Callback),
    PrePostorder(&'traversal mut Callback, &'traversal mut Callback),
}

pub fn traverse(
    traversal: &mut Traversal,
    node: NodePointer,
    ast: &mut AST,
    symbol_table: &mut SymbolTable,
    logger: &mut Logger,
    error: &mut ErrorReporter,
) {
    // If we're doing pre or prepost,
    // run the callback before we traverse to our children
    match traversal {
        Traversal::Preorder(&mut ref mut callback_pre) => {
            callback_pre.run(node, ast, symbol_table, logger, error)
        }
        Traversal::Postorder(_) => {}
        Traversal::PrePostorder(&mut ref mut callback_pre, _) => {
            callback_pre.run(node, ast, symbol_table, logger, error)
        }
    }

    match ast.get_node(node) {
        ASTNode::RootNode(root_node) => {
            for child in root_node.children.clone() {
                traverse(traversal, child, ast, symbol_table, logger, error);
            }
        }
        ASTNode::ExpressionNode(expr_node) => match expr_node {
            ExpressionNode::Binary(bin_node) => {
                let left = bin_node.left;
                let right = bin_node.right;
                traverse(traversal, left, ast, symbol_table, logger, error);
                traverse(traversal, right, ast, symbol_table, logger, error);
            }
            ExpressionNode::Unary(un_node) => {
                traverse(traversal, un_node.operand, ast, symbol_table, logger, error);
            }
            // Leaf node
            ExpressionNode::Literal(_) => {}
            ExpressionNode::Variable(_) => {}
        },
        ASTNode::StatementNode(stat_node) => match stat_node {
            StatementNode::VariableDeclaration(var_node) => {
                let expression = var_node.expression;
                match var_node.type_hint {
                    Some(type_hint) => {
                        traverse(traversal, type_hint, ast, symbol_table, logger, error)
                    }
                    None => {}
                }
                traverse(traversal, expression, ast, symbol_table, logger, error);
            }
            StatementNode::VariableAssignment(assmt_node) => {
                let expression = assmt_node.expression;
                traverse(traversal, expression, ast, symbol_table, logger, error);
            }
        },
        // Leaf node
        ASTNode::IdentifierNode(_) => {}
        ASTNode::TypeHintNode(_) => {}
        ASTNode::NotANode(_) => {}
    }

    // If we're doing post or prepost,
    // run the callback before we traverse to our children
    match traversal {
        Traversal::Preorder(_) => {}
        Traversal::Postorder(callback_post) => {
            callback_post.run(node, ast, symbol_table, logger, error)
        }
        Traversal::PrePostorder(_, callback_post) => {
            callback_post.run(node, ast, symbol_table, logger, error)
        }
    }
}
