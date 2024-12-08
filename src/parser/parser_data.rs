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

use slotmap::{new_key_type, SlotMap};

use crate::{
    infrastructure::{error::ErrorReporter, log::Logger},
    scanner::scanner_data::Token,
    semantic_checker::semantic_checker_data::{Callback, DataType, Symbol, SymbolTable, Type},
};

// #[derive(PartialEq)]
pub struct AST {
    pub root: RootKey,

    pub file_nodes: SlotMap<FileKey, File>,

    pub repl_root_nodes: SlotMap<REPLRootKey, REPLRoot>,
    pub file_root_nodes: SlotMap<FileRootKey, FileRoot>,

    pub variable_declaration_nodes: SlotMap<VariableDeclarationKey, VariableDeclaration>,
    pub variable_assignment_nodes: SlotMap<VariableAssignmentKey, VariableAssignment>,

    pub binary_expression_nodes: SlotMap<BinaryExpressionKey, BinaryExpression>,
    pub unary_expression_nodes: SlotMap<UnaryExpressionKey, UnaryExpression>,
    pub variable_expression_nodes: SlotMap<VariableExpressionKey, VariableExpression>,
    pub literal_expression_nodes: SlotMap<LiteralExpressionKey, LiteralExpression>,

    pub identifier_nodes: SlotMap<IdentifierKey, Identifier>,

    pub type_hint_nodes: SlotMap<TypeHintKey, TypeHint>,
}

impl AST {
    pub fn new(repl_mode: bool) -> AST {
        let root;

        let mut repl_root_nodes = SlotMap::with_key();
        let mut file_root_nodes = SlotMap::with_key();

        if repl_mode {
            root = RootKey::REPL(repl_root_nodes.insert(REPLRoot::new()));
        } else {
            root = RootKey::File(file_root_nodes.insert(FileRoot::new()));
        }

        return AST {
            root,

            file_nodes: SlotMap::with_key(),

            repl_root_nodes,
            file_root_nodes,

            variable_declaration_nodes: SlotMap::with_key(),
            variable_assignment_nodes: SlotMap::with_key(),

            binary_expression_nodes: SlotMap::with_key(),
            unary_expression_nodes: SlotMap::with_key(),
            variable_expression_nodes: SlotMap::with_key(),
            literal_expression_nodes: SlotMap::with_key(),

            identifier_nodes: SlotMap::with_key(),

            type_hint_nodes: SlotMap::with_key(),
        };
    }

    pub fn has_been_changed(&self) -> bool {
        match self.root {
            RootKey::File(_) => false,
            RootKey::REPL(repl) => {
                let root = self.get_repl_root_mut(repl);
                let answer = root.ast_has_been_changed;
                root.ast_has_been_changed = false;
                answer
            }
        }
    }

    pub fn get_repl_root(&self, key: REPLRootKey) -> &REPLRoot {
        return &self.repl_root_nodes[key];
    }

    pub fn get_repl_root_mut(&self, key: REPLRootKey) -> &mut REPLRoot {
        return &mut self.repl_root_nodes[key];
    }

    pub fn new_repl_root(&self, node: REPLRoot) -> REPLRootKey {
        return self.repl_root_nodes.insert(node);
    }

    pub fn get_file_root(&self, key: FileRootKey) -> &FileRoot {
        return &self.file_root_nodes[key];
    }

    pub fn get_file_root_mut(&self, key: FileRootKey) -> &mut FileRoot {
        return &mut self.file_root_nodes[key];
    }

    pub fn new_file_root(&self, node: FileRoot) -> FileRootKey {
        return self.file_root_nodes.insert(node);
    }

    pub fn get_file(&self, key: FileKey) -> &File {
        return &self.file_nodes[key];
    }

    pub fn get_file_mut(&self, key: FileKey) -> &mut File {
        return &mut self.file_nodes[key];
    }

    pub fn new_file(&self, node: File) -> FileKey {
        return self.file_nodes.insert(node);
    }

    pub fn get_variable_declaration(&self, key: VariableDeclarationKey) -> &VariableDeclaration {
        return &self.variable_declaration_nodes[key];
    }

    pub fn get_variable_declaration_mut(
        &self,
        key: VariableDeclarationKey,
    ) -> &mut VariableDeclaration {
        return &mut self.variable_declaration_nodes[key];
    }

    pub fn new_variable_declaration(&self, node: VariableDeclaration) -> VariableDeclarationKey {
        return self.variable_declaration_nodes.insert(node);
    }

    pub fn get_variable_assignment(&self, key: VariableAssignmentKey) -> &VariableAssignment {
        return &self.variable_assignment_nodes[key];
    }

    pub fn get_variable_assignment_mut(
        &self,
        key: VariableAssignmentKey,
    ) -> &mut VariableAssignment {
        return &mut self.variable_assignment_nodes[key];
    }

    pub fn new_variable_assignment(&self, node: VariableAssignment) -> VariableAssignmentKey {
        return self.variable_assignment_nodes.insert(node);
    }

    pub fn get_binary_expression(&self, key: BinaryExpressionKey) -> &BinaryExpression {
        return &self.binary_expression_nodes[key];
    }

    pub fn get_binary_expression_mut(&self, key: BinaryExpressionKey) -> &mut BinaryExpression {
        return &mut self.binary_expression_nodes[key];
    }

    pub fn new_binary_expression(&self, node: BinaryExpression) -> BinaryExpressionKey {
        return self.binary_expression_nodes.insert(node);
    }

    pub fn get_unary_expression(&self, key: UnaryExpressionKey) -> &UnaryExpression {
        return &self.unary_expression_nodes[key];
    }

    pub fn get_unary_expression_mut(&self, key: UnaryExpressionKey) -> &mut UnaryExpression {
        return &mut self.unary_expression_nodes[key];
    }

    pub fn new_unary_expression(&self, node: UnaryExpression) -> UnaryExpressionKey {
        return self.unary_expression_nodes.insert(node);
    }

    pub fn get_variable_expression(&self, key: VariableExpressionKey) -> &VariableExpression {
        return &self.variable_expression_nodes[key];
    }

    pub fn get_variable_expression_mut(
        &self,
        key: VariableExpressionKey,
    ) -> &mut VariableExpression {
        return &mut self.variable_expression_nodes[key];
    }

    pub fn new_variable_expression(&self, node: VariableExpression) -> VariableExpressionKey {
        return self.variable_expression_nodes.insert(node);
    }

    pub fn get_literal_expression(&self, key: LiteralExpressionKey) -> &LiteralExpression {
        return &self.literal_expression_nodes[key];
    }

    pub fn get_literal_expression_mut(&self, key: LiteralExpressionKey) -> &mut LiteralExpression {
        return &mut self.literal_expression_nodes[key];
    }

    pub fn new_literal_expression(&self, node: LiteralExpression) -> LiteralExpressionKey {
        return self.literal_expression_nodes.insert(node);
    }

    pub fn get_identifier(&self, key: IdentifierKey) -> &Identifier {
        return &self.identifier_nodes[key];
    }

    pub fn get_identifier_mut(&self, key: IdentifierKey) -> &mut Identifier {
        return &mut self.identifier_nodes[key];
    }

    pub fn new_identifier(&self, node: Identifier) -> IdentifierKey {
        return self.identifier_nodes.insert(node);
    }

    pub fn get_type_hint(&self, key: TypeHintKey) -> &TypeHint {
        return &self.type_hint_nodes[key];
    }

    pub fn get_type_hint_mut(&self, key: TypeHintKey) -> &mut TypeHint {
        return &mut self.type_hint_nodes[key];
    }

    pub fn new_type_hint(&self, node: TypeHint) -> TypeHintKey {
        return self.type_hint_nodes.insert(node);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ASTNodeKey {
    Root(RootKey),
    File(FileKey),
    REPLCommand(REPLCommandKey),
    Statement(StatementKey),
    Expression(ExpressionKey),
    Identifier(IdentifierKey),
    TypeHint(TypeHintKey),
    NotANode,
}

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    Root(Root),
    File(File),
    REPLCommand(REPLCommand),
    Statement(Statement),
    Expression(Expression),
    Identifier(Identifier),
    TypeHint(TypeHint),
}

impl ASTNode {
    pub fn to_string(&self, ast: &AST) -> String {
        match self {
            ASTNode::Root(node) => return node.to_string(ast),
            ASTNode::File(node) => node.to_string(ast),
            ASTNode::REPLCommand(node) => return node.to_string(ast),
            ASTNode::Statement(node) => return node.to_string(ast),
            ASTNode::Expression(node) => return node.to_string(ast),
            ASTNode::Identifier(node) => return node.to_string(),
            ASTNode::TypeHint(node) => return node.to_string(),
        }
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        match self {
            ASTNode::Root(node) => return node.to_string_with_level(level, ast),
            ASTNode::File(node) => return node.to_string_with_level(level, ast),
            ASTNode::REPLCommand(node) => return node.to_string_with_level(level, ast),
            ASTNode::Statement(node) => return node.to_string_with_level(level, ast),
            ASTNode::Expression(node) => return node.to_string_with_level(level, ast),
            ASTNode::Identifier(node) => return node.to_string_with_level(level),
            ASTNode::TypeHint(node) => return node.to_string_with_level(level),
        }
    }

    pub fn node_to_string(&self) -> String {
        match self {
            ASTNode::Root(node) => return node.node_to_string(),
            ASTNode::File(node) => return node.node_to_string(),
            ASTNode::REPLCommand(node) => return node.node_to_string(),
            ASTNode::Statement(node) => return node.node_to_string(),
            ASTNode::Expression(node) => return node.node_to_string(),
            ASTNode::Identifier(node) => return node.node_to_string(),
            ASTNode::TypeHint(node) => return node.node_to_string(),
        }
    }

    pub fn to_code(&self, ast: &AST) -> String {
        match self {
            ASTNode::Root(node) => return node.to_code(),
            ASTNode::File(node) => return node.to_code(ast),
            ASTNode::REPLCommand(node) => return node.to_code(),
            ASTNode::Statement(node) => return node.to_code(),
            ASTNode::Expression(node) => return node.to_code(),
            ASTNode::Identifier(node) => return node.to_code(),
            ASTNode::TypeHint(node) => return node.to_code(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum RootKey {
    REPL(REPLRootKey),
    File(FileRootKey),
}

#[derive(Clone, PartialEq)]
pub enum Root {
    REPL(REPLRoot),
    File(FileRoot),
}

new_key_type! { pub struct REPLRootKey; }
#[derive(Clone, PartialEq)]
pub struct REPLRoot {
    pub commands: Vec<REPLCommandKey>,
    pub new_commands_start_at: usize,
    pub ast_has_been_changed: bool,
}

impl REPLRoot {
    pub fn new() -> REPLRoot {
        REPLRoot {
            commands: vec![],
            new_commands_start_at: 0,
            ast_has_been_changed: false,
        }
    }

    pub fn add_command(&mut self, new_command: REPLCommandKey) {
        self.ast_has_been_changed = true;
        self.new_commands_start_at = self.commands.len();
        self.commands.push(new_command);
    }

    pub fn add_commands(&mut self, mut new_commands: Vec<REPLCommandKey>) {
        self.ast_has_been_changed = true;
        self.new_commands_start_at = self.commands.len();
        self.commands.append(&mut new_commands);
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        let mut format = format!("{}\n", self.node_to_string());

        for command in self.commands.clone() {
            match command {
                REPLCommandKey::Stmt(StatementKey::VarDecl(decl)) => format.push_str(
                    &ast.get_variable_declaration(decl)
                        .to_string_with_level(level + 1, ast),
                ),
                REPLCommandKey::Stmt(StatementKey::VarAssmt(assmt)) => format.push_str(
                    &ast.get_variable_assignment(assmt)
                        .to_string_with_level(level + 1, ast),
                ),
                REPLCommandKey::Expr(ExpressionKey::Binary(bin)) => format.push_str(
                    &ast.get_binary_expression(bin)
                        .to_string_with_level(level + 1, ast),
                ),
                REPLCommandKey::Expr(ExpressionKey::Unary(un)) => format.push_str(
                    &ast.get_unary_expression(un)
                        .to_string_with_level(level + 1, ast),
                ),
                REPLCommandKey::Expr(ExpressionKey::Variable(var)) => format.push_str(
                    &ast.get_variable_expression(var)
                        .to_string_with_level(level + 1),
                ),
                REPLCommandKey::Expr(ExpressionKey::Literal(lit)) => format.push_str(
                    &ast.get_literal_expression(lit)
                        .to_string_with_level(level + 1),
                ),
            }
        }

        return format;
    }

    pub fn node_to_string(&self) -> String {
        return String::from("{REPLRoot}");
    }

    pub fn to_code(&self, ast: &AST) -> String {
        let mut format = format!("{}", self.node_to_string());

        for command in self.commands.clone() {
            match command {
                REPLCommandKey::Stmt(StatementKey::VarDecl(decl)) => {
                    format.push_str(&ast.get_variable_declaration(decl).to_code(ast))
                }
                REPLCommandKey::Stmt(StatementKey::VarAssmt(assmt)) => {
                    format.push_str(&ast.get_variable_assignment(assmt).to_code(ast))
                }
                REPLCommandKey::Expr(ExpressionKey::Binary(bin)) => {
                    format.push_str(&ast.get_binary_expression(bin).to_code(ast))
                }
                REPLCommandKey::Expr(ExpressionKey::Unary(un)) => {
                    format.push_str(&ast.get_unary_expression(un).to_code(ast))
                }
                REPLCommandKey::Expr(ExpressionKey::Variable(var)) => {
                    format.push_str(&ast.get_variable_expression(var).to_code())
                }
                REPLCommandKey::Expr(ExpressionKey::Literal(lit)) => {
                    format.push_str(&ast.get_literal_expression(lit).to_code())
                }
            }
        }

        return format;
    }
}

new_key_type! { pub struct FileRootKey; }
#[derive(Clone, PartialEq)]
pub struct FileRoot {
    pub file: Option<FileKey>,
}

impl FileRoot {
    pub fn new() -> FileRoot {
        FileRoot { file: None }
    }

    pub fn set_file(&mut self, new_file: FileKey) {
        self.file = Some(new_file);
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        return format!(
            "{}\n{}",
            self.node_to_string(),
            match self.file {
                None => String::from(""),
                Some(file) => ast.get_file(file).to_string_with_level(level + 1, ast),
            }
        );
    }

    pub fn node_to_string(&self) -> String {
        return String::from("{FileRoot}");
    }

    pub fn to_code(&self, ast: &AST) -> String {
        return match self.file {
            None => String::from(""),
            Some(file) => ast.get_file(file).to_code(ast),
        };
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum REPLCommandKey {
    Stmt(StatementKey),
    Expr(ExpressionKey),
}

#[derive(Clone, PartialEq)]
pub enum REPLCommand {
    Stmt(Statement),
    Expr(Expression),
}

new_key_type! { pub struct FileKey; }
#[derive(Clone, PartialEq)]
pub struct File {
    pub statements: Vec<StatementKey>,
    pub expression: Option<ExpressionKey>,
}

impl File {
    pub fn new(statements: Vec<StatementKey>, expression: Option<ExpressionKey>) -> File {
        File {
            statements,
            expression,
        }
    }

    pub fn set_statements(&mut self, new_statements: Vec<StatementKey>) {
        self.statements = new_statements;
    }

    pub fn set_expression(&mut self, expression: ExpressionKey) {
        self.expression = Some(expression);
    }

    pub fn add_statement(&mut self, new_statement: StatementKey) {
        self.statements.push(new_statement);
    }

    pub fn add_statements(&mut self, mut new_statements: Vec<StatementKey>) {
        self.statements.append(&mut new_statements);
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        let mut format = format!("{}\n", self.node_to_string());

        for statement in self.statements.clone() {
            match statement {
                StatementKey::VarDecl(decl) => format.push_str(
                    &ast.get_variable_declaration(decl)
                        .to_string_with_level(level + 1, ast),
                ),
                StatementKey::VarAssmt(assmt) => format.push_str(
                    &ast.get_variable_assignment(assmt)
                        .to_string_with_level(level + 1, ast),
                ),
            }
        }

        match self.expression {
            Some(ExpressionKey::Binary(bin)) => format.push_str(
                &ast.get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
            ),
            Some(ExpressionKey::Unary(un)) => format.push_str(
                &ast.get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
            ),
            Some(ExpressionKey::Variable(var)) => format.push_str(
                &ast.get_variable_expression(var)
                    .to_string_with_level(level + 1),
            ),
            Some(ExpressionKey::Literal(lit)) => format.push_str(
                &ast.get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            ),
            None => {}
        }

        return format;
    }

    pub fn node_to_string(&self) -> String {
        return String::from("{File}");
    }

    pub fn to_code(&self, ast: &AST) -> String {
        let mut format = format!("{}", self.node_to_string());

        for statement in self.statements.clone() {
            match statement {
                StatementKey::VarDecl(decl) => {
                    format.push_str(&ast.get_variable_declaration(decl).to_code(ast))
                }
                StatementKey::VarAssmt(assmt) => {
                    format.push_str(&ast.get_variable_assignment(assmt).to_code(ast))
                }
            }
        }

        match self.expression {
            Some(ExpressionKey::Binary(bin)) => {
                format.push_str(&ast.get_binary_expression(bin).to_code(ast))
            }
            Some(ExpressionKey::Unary(un)) => {
                format.push_str(&ast.get_unary_expression(un).to_code(ast))
            }
            Some(ExpressionKey::Variable(var)) => {
                format.push_str(&ast.get_variable_expression(var).to_code())
            }
            Some(ExpressionKey::Literal(lit)) => {
                format.push_str(&ast.get_literal_expression(lit).to_code())
            }
            None => {}
        }

        return format;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatementKey {
    VarDecl(VariableDeclarationKey),
    VarAssmt(VariableAssignmentKey),
    NotANode,
}

#[derive(Clone, PartialEq)]
pub enum Statement {
    VarDecl(VariableDeclaration),
    VarAssmt(VariableAssignment),
}

impl Statement {
    pub fn to_string(&self, ast: &AST) -> String {
        match self {
            Statement::VarDecl(decl) => decl.to_string_with_level(0, ast),
            Statement::VarAssmt(assmt) => assmt.to_string_with_level(0, ast),
        }
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        match self {
            Statement::VarDecl(decl) => decl.to_string_with_level(level, ast),
            Statement::VarAssmt(assmt) => assmt.to_string_with_level(level, ast),
        }
    }

    pub fn node_to_string(&self) -> String {
        match self {
            Statement::VarDecl(decl) => decl.node_to_string(),
            Statement::VarAssmt(assmt) => assmt.node_to_string(),
        }
    }
}

new_key_type! { pub struct VariableDeclarationKey; }
#[derive(Clone, PartialEq)]
pub struct VariableDeclaration {
    pub mutable: bool,
    pub name: String,
    pub identifier: IdentifierKey,
    pub type_hint: Option<TypeHintKey>,
    pub expression: ExpressionKey,
    pub token: Token,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl VariableDeclaration {
    pub fn new(
        mutable: bool,
        name: String,
        identifier: IdentifierKey,
        type_hint: Option<TypeHintKey>,
        expression: ExpressionKey,
        token: Token,
    ) -> VariableDeclaration {
        VariableDeclaration {
            mutable,
            name,
            identifier,
            type_hint,
            expression,
            token,
            symbol: None,
        }
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        return format!(
            "{}{}\n{}{}{}",
            "\t".repeat(level),
            self.node_to_string(),
            ast.get_identifier(self.identifier)
                .to_string_with_level(level + 1),
            match self.type_hint {
                Some(type_hint) => format!(
                    " {}",
                    ast.get_type_hint(type_hint).to_string_with_level(level + 1)
                ),
                None => String::from(""),
            },
            match self.expression {
                ExpressionKey::Binary(bin) => ast
                    .get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Unary(un) => ast
                    .get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Variable(var) => ast
                    .get_variable_expression(var)
                    .to_string_with_level(level + 1),
                ExpressionKey::Literal(lit) => ast
                    .get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            }
        );
    }

    pub fn node_to_string(&self) -> String {
        return format!(
            "{{Declaration{}}}",
            if self.mutable { " (mut)" } else { "" }
        );
    }

    pub fn to_code(&self, ast: &AST) -> String {
        return format!(
            "let{} {}{} := {};",
            if self.mutable { " (mut)" } else { "" },
            ast.get_identifier(self.identifier).to_code(),
            match self.type_hint {
                Some(type_hint) => format!(" {}", ast.get_type_hint(type_hint).to_code()),
                None => String::from(""),
            },
            match self.expression {
                ExpressionKey::Binary(bin) => ast.get_binary_expression(bin).to_code(ast),
                ExpressionKey::Unary(un) => ast.get_unary_expression(un).to_code(ast),
                ExpressionKey::Variable(var) => ast.get_variable_expression(var).to_code(),
                ExpressionKey::Literal(lit) => ast.get_literal_expression(lit).to_code(),
            }
        );
    }
}

pub enum ExpressionOrAssignmentKey {
    Expression(ExpressionKey),
    Assignment(VariableAssignmentKey),
    NotANode,
}

new_key_type! { pub struct VariableAssignmentKey; }
#[derive(Clone, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub identifier: IdentifierKey,
    pub expression: ExpressionKey,
    pub token: Token,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl VariableAssignment {
    pub fn new(
        name: String,
        identifier: IdentifierKey,
        expression: ExpressionKey,
        token: Token,
    ) -> VariableAssignment {
        VariableAssignment {
            name,
            identifier,
            expression,
            token,
            symbol: None,
        }
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        return format!(
            "{}{}\n{}{}",
            "\t".repeat(level),
            self.node_to_string(),
            ast.get_identifier(self.identifier)
                .to_string_with_level(level + 1),
            match self.expression {
                ExpressionKey::Binary(bin) => ast
                    .get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Unary(un) => ast
                    .get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Variable(var) => ast
                    .get_variable_expression(var)
                    .to_string_with_level(level + 1),
                ExpressionKey::Literal(lit) => ast
                    .get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            }
        );
    }

    pub fn node_to_string(&self) -> String {
        return String::from("{Assignment}");
    }

    pub fn to_code(&self, ast: &AST) -> String {
        return format!(
            "{} := {};",
            ast.get_identifier(self.identifier).to_code(),
            match self.expression {
                ExpressionKey::Binary(bin) => ast.get_binary_expression(bin).to_code(ast),
                ExpressionKey::Unary(un) => ast.get_unary_expression(un).to_code(ast),
                ExpressionKey::Variable(var) => ast.get_variable_expression(var).to_code(),
                ExpressionKey::Literal(lit) => ast.get_literal_expression(lit).to_code(),
            }
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ExpressionKey {
    Binary(BinaryExpressionKey),
    Unary(UnaryExpressionKey),
    Variable(VariableExpressionKey),
    Literal(LiteralExpressionKey),
    NotANode,
}

#[derive(Clone, PartialEq)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Variable(VariableExpression),
    Literal(LiteralExpression),
}

impl Expression {
    pub fn to_string(&self, ast: &AST) -> String {
        match self {
            Expression::Binary(bin) => bin.to_string_with_level(0, ast),
            Expression::Unary(un) => un.to_string_with_level(0, ast),
            Expression::Variable(var) => var.to_string_with_level(0),
            Expression::Literal(lit) => lit.to_string_with_level(0),
        }
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        match self {
            Expression::Binary(bin) => bin.to_string_with_level(level, ast),
            Expression::Unary(un) => un.to_string_with_level(level, ast),
            Expression::Variable(var) => var.to_string_with_level(level),
            Expression::Literal(lit) => lit.to_string_with_level(level),
        }
    }

    pub fn node_to_string(&self) -> String {
        match self {
            Expression::Binary(bin) => bin.node_to_string(),
            Expression::Unary(un) => un.node_to_string(),
            Expression::Variable(var) => var.node_to_string(),
            Expression::Literal(lit) => lit.node_to_string(),
        }
    }
}

new_key_type! { pub struct BinaryExpressionKey; }
#[derive(Clone, PartialEq)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub token: Token,
    pub data_type: DataType,

    pub left: ExpressionKey,
    pub right: ExpressionKey,
}

impl BinaryExpression {
    pub fn new(
        operator: BinaryOperator,
        token: Token,
        left: ExpressionKey,
        right: ExpressionKey,
    ) -> BinaryExpression {
        BinaryExpression {
            operator,
            token,
            data_type: DataType::new(Type::UnTyped),
            left,
            right,
        }
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn set_left(&mut self, new_child: ExpressionKey) {
        self.left = new_child;
    }

    pub fn set_right(&mut self, new_child: ExpressionKey) {
        self.right = new_child;
    }

    pub fn get_type(&self) -> DataType {
        return self.data_type.clone();
    }

    pub fn set_type(&mut self, data_type: DataType) {
        self.data_type = data_type;
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        return format!(
            "{}{}\n{}{}",
            "\t".repeat(level),
            self.node_to_string(),
            match self.left {
                ExpressionKey::Binary(bin) => ast
                    .get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Unary(un) => ast
                    .get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Variable(var) => ast
                    .get_variable_expression(var)
                    .to_string_with_level(level + 1),
                ExpressionKey::Literal(lit) => ast
                    .get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            },
            match self.right {
                ExpressionKey::Binary(bin) => ast
                    .get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Unary(un) => ast
                    .get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Variable(var) => ast
                    .get_variable_expression(var)
                    .to_string_with_level(level + 1),
                ExpressionKey::Literal(lit) => ast
                    .get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            },
        );
    }

    pub fn node_to_string(&self) -> String {
        return format!(
            "{{{}{}}}",
            self.get_operator_str(),
            match self.data_type.data_type {
                Type::UnTyped => String::from(""),
                _ => format!(", type: '{}'", self.data_type.get_label()),
            }
        );
    }

    pub fn get_operator_str(&self) -> String {
        return match self.operator {
            BinaryOperator::Plus => String::from("+"),
            BinaryOperator::Minus => String::from("-"),
            BinaryOperator::Times => String::from("*"),
            BinaryOperator::Divide => String::from("/"),
            BinaryOperator::Modulus => String::from("%"),
            BinaryOperator::NotEqual => String::from("!="),
            BinaryOperator::Equal => String::from("=="),
            BinaryOperator::LessThan => String::from("<"),
            BinaryOperator::LessThanOrEqual => String::from("<="),
            BinaryOperator::GreaterThan => String::from(">"),
            BinaryOperator::GreaterThanOrEqual => String::from(">="),
            BinaryOperator::And => String::from("and"),
            BinaryOperator::Or => String::from("or"),
        };
    }

    pub fn to_code(&self, ast: &AST) -> String {
        return format!(
            "{} {} {};",
            match self.left {
                ExpressionKey::Binary(bin) => ast.get_binary_expression(bin).to_code(ast),
                ExpressionKey::Unary(un) => ast.get_unary_expression(un).to_code(ast),
                ExpressionKey::Variable(var) => ast.get_variable_expression(var).to_code(),
                ExpressionKey::Literal(lit) => ast.get_literal_expression(lit).to_code(),
            },
            self.get_operator_str(),
            match self.right {
                ExpressionKey::Binary(bin) => ast.get_binary_expression(bin).to_code(ast),
                ExpressionKey::Unary(un) => ast.get_unary_expression(un).to_code(ast),
                ExpressionKey::Variable(var) => ast.get_variable_expression(var).to_code(),
                ExpressionKey::Literal(lit) => ast.get_literal_expression(lit).to_code(),
            },
        );
    }
}

#[derive(Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulus,
    NotEqual,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

new_key_type! { pub struct UnaryExpressionKey; }
#[derive(Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub token: Token,
    pub data_type: DataType,

    pub operand: ExpressionKey,
}

impl UnaryExpression {
    pub fn new(operator: UnaryOperator, token: Token, operand: ExpressionKey) -> UnaryExpression {
        return UnaryExpression {
            operator,
            token,
            data_type: DataType::new(Type::UnTyped),
            operand,
        };
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn set_operand(&mut self, new_child: ExpressionKey) {
        self.operand = new_child;
    }

    pub fn get_type(&self) -> DataType {
        return self.data_type.clone();
    }

    pub fn set_type(&mut self, data_type: DataType) {
        self.data_type = data_type;
    }

    pub fn to_string(&self, ast: &AST) -> String {
        return self.to_string_with_level(0, ast);
    }

    pub fn to_string_with_level(&self, level: usize, ast: &AST) -> String {
        return format!(
            "{}{}\n{}",
            "\t".repeat(level),
            self.node_to_string(),
            match self.operand {
                ExpressionKey::Binary(bin) => ast
                    .get_binary_expression(bin)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Unary(un) => ast
                    .get_unary_expression(un)
                    .to_string_with_level(level + 1, ast),
                ExpressionKey::Variable(var) => ast
                    .get_variable_expression(var)
                    .to_string_with_level(level + 1),
                ExpressionKey::Literal(lit) => ast
                    .get_literal_expression(lit)
                    .to_string_with_level(level + 1),
            },
        );
    }

    pub fn node_to_string(&self) -> String {
        return format!(
            "{{{}{}{}}}",
            match self.operator {
                UnaryOperator::Minus => "unary ",
                _ => "",
            },
            self.get_operator_str(),
            match self.data_type.data_type {
                Type::UnTyped => String::from(""),
                _ => format!(", type: '{}'", self.data_type.get_label()),
            }
        );
    }

    pub fn get_operator_str(&self) -> String {
        return match self.operator {
            UnaryOperator::Minus => String::from("-"),
            UnaryOperator::Not => String::from("not"),
        };
    }

    pub fn to_code(&self, ast: &AST) -> String {
        return format!(
            "{}{};",
            self.get_operator_str(),
            match self.operand {
                ExpressionKey::Binary(bin) => ast.get_binary_expression(bin).to_code(ast),
                ExpressionKey::Unary(un) => ast.get_unary_expression(un).to_code(ast),
                ExpressionKey::Variable(var) => ast.get_variable_expression(var).to_code(),
                ExpressionKey::Literal(lit) => ast.get_literal_expression(lit).to_code(),
            },
        );
    }
}

#[derive(Clone, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Not,
}

new_key_type! { pub struct VariableExpressionKey; }
#[derive(Clone, PartialEq)]
pub struct VariableExpression {
    pub name: String,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
    pub token: Token,
}

impl VariableExpression {
    pub fn new(name: String, token: Token) -> VariableExpression {
        return VariableExpression {
            name,
            symbol: None,
            token,
        };
    }
    pub fn get_type(&self) -> DataType {
        match &self.symbol {
            Some(symbol) => {
                return symbol.borrow().get_type().clone();
            }
            None => return DataType::new(Type::UnTyped),
        };
    }

    pub fn set_type(&mut self, data_type: DataType) {
        match &self.symbol {
            Some(symbol) => {
                return symbol.borrow_mut().set_type(data_type);
            }
            None => panic!("Trying to set type of variable that has no symbol"),
        }
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self) -> String {
        return self.to_string_with_level(0);
    }

    pub fn to_string_with_level(&self, level: usize) -> String {
        return format!("{}{}\n", "\t".repeat(level), self.node_to_string());
    }

    pub fn node_to_string(&self) -> String {
        return format!(
            "{{Variable: {}{}}}",
            self.name,
            match self.get_type().data_type {
                Type::UnTyped => String::from(""),
                _type => format!(", type: '{}'", _type.get_label()),
            }
        );
    }

    pub fn to_code(&self) -> String {
        return self.name.clone();
    }
}

new_key_type! { pub struct LiteralExpressionKey; }
#[derive(Clone, PartialEq)]
pub struct LiteralExpression {
    pub value: Literal,
    pub token: Token,
    pub data_type: DataType,
}

impl LiteralExpression {
    pub fn new(value: Literal, token: Token) -> LiteralExpression {
        return LiteralExpression {
            value,
            token,
            data_type: DataType::new(Type::UnTyped),
        };
    }

    pub fn get_type(&self) -> DataType {
        return self.data_type.clone();
    }

    pub fn set_type(&mut self, data_type: DataType) {
        self.data_type = data_type;
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self) -> String {
        return self.to_string_with_level(0);
    }

    pub fn to_string_with_level(&self, level: usize) -> String {
        return format!("{}{}\n", "\t".repeat(level), self.node_to_string());
    }

    pub fn node_to_string(&self) -> String {
        return format!(
            "{{{}{}}}",
            match &self.value {
                Literal::Int(intval) => format!("int: {}", intval),
                Literal::Float(floatval) => format!("float: {}", floatval),
                Literal::True(_) => String::from("true"),
                Literal::False(_) => String::from("false"),
                Literal::String(stringval) => format!("string: \"{}\"", stringval.clone()),
            },
            match self.data_type.data_type {
                Type::UnTyped => String::from(""),
                _ => format!(", type: '{}'", self.data_type.get_label()),
            }
        );
    }

    pub fn to_code(&self) -> String {
        return match &self.value {
            Literal::Int(intval) => format!("int: {}", intval),
            Literal::Float(floatval) => format!("float: {}", floatval),
            Literal::True(_) => String::from("true"),
            Literal::False(_) => String::from("false"),
            Literal::String(stringval) => format!("string: \"{}\"", stringval.clone()),
        };
    }
}

#[derive(Clone, PartialEq)]
pub enum Literal {
    Int(i128),
    Float(f64),
    True(bool),
    False(bool),
    String(String),
}

new_key_type! { pub struct IdentifierKey; }
#[derive(Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub token: Token,
}

impl Identifier {
    pub fn new(name: String, token: Token) -> Identifier {
        return Identifier { name, token };
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self) -> String {
        return self.to_string_with_level(0);
    }

    pub fn to_string_with_level(&self, level: usize) -> String {
        return format!("{}{}\n", "\t".repeat(level), self.node_to_string());
    }

    pub fn node_to_string(&self) -> String {
        return format!("{{ID: {}}}", self.name);
    }

    pub fn to_code(&self) -> String {
        return self.name.clone();
    }
}

new_key_type! { pub struct TypeHintKey; }
#[derive(Clone, PartialEq)]
pub struct TypeHint {
    data_type: DataType,
    token: Token,
}

impl TypeHint {
    pub fn new(data_type: DataType, token: Token) -> TypeHint {
        return TypeHint { data_type, token };
    }

    pub fn get_type(&self) -> DataType {
        return self.data_type.clone();
    }

    pub fn get_token(&self) -> &Token {
        return &self.token;
    }

    pub fn to_string(&self) -> String {
        return self.to_string_with_level(0);
    }

    pub fn to_string_with_level(&self, level: usize) -> String {
        return format!("{}{}\n", "\t".repeat(level), self.node_to_string());
    }

    pub fn node_to_string(&self) -> String {
        return format!("{{Type: {}}}", self.data_type.get_label());
    }

    pub fn to_code(&self) -> String {
        return String::from(self.data_type.get_label());
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
