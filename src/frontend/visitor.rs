use super::ast;

pub trait AstVisitor {
    type ProgramOutput;
    type ItemOutput;
    type FunctionOutput;
    type BlockOutput;
    type StmtOutput;
    type ExprOutput;

    fn visit_program(&mut self, node: &ast::Program) -> Self::ProgramOutput;
    fn visit_item(&mut self, node: &ast::Item) -> Self::ItemOutput;
    fn visit_function(&mut self, node: &ast::Function) -> Self::FunctionOutput;
    fn visit_block(&mut self, node: &ast::Block) -> Self::BlockOutput;
    fn visit_stmt(&mut self, node: &ast::Stmt) -> Self::StmtOutput;
    fn visit_expr(&mut self, node: &ast::Expr) -> Self::ExprOutput;
}

pub trait AstNode<V: AstVisitor> {
    type Output;
    fn accept(&self, visitor: &mut V) -> Self::Output;
}

impl<V: AstVisitor> AstNode<V> for ast::Program {
    type Output = V::ProgramOutput;

    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_program(self)
    }
}
impl<V: AstVisitor> AstNode<V> for ast::Item {
    type Output = V::ItemOutput;

    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_item(self)
    }
}
impl<V: AstVisitor> AstNode<V> for ast::Function {
    type Output = V::FunctionOutput;
    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_function(self)
    }
}
impl<V: AstVisitor> AstNode<V> for ast::Block {
    type Output = V::BlockOutput;

    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_block(self)
    }
}
impl<V: AstVisitor> AstNode<V> for ast::Stmt {
    type Output = V::StmtOutput;

    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_stmt(self)
    }
}
impl<V: AstVisitor> AstNode<V> for ast::Expr {
    type Output = V::ExprOutput;
    fn accept(&self, visitor: &mut V) -> Self::Output {
        visitor.visit_expr(self)
    }
}


