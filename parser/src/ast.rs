#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Assign { name: String, value: Box<AstNode> },
    MultiAssign { names: Vec<String>, values: Vec<AstNode> },
    AttributeAssign { object: Box<AstNode>, attr: String, value: Box<AstNode> },
    IndexAssign { object: Box<AstNode>, index: Box<AstNode>, value: Box<AstNode> },
    Call { func: Box<AstNode>, args: Vec<AstNode> },
    /// 类实例化
    Instance { class: String, args: Vec<AstNode> },
    /// 参数展开 *args
    StarredArg(Box<AstNode>),
    /// 关键字参数展开 **kwargs
    DoubleStarredArg(Box<AstNode>),
    If { cond: Box<AstNode>, body: Vec<AstNode>, orelse: Vec<AstNode> },
    For { var: String, iter: Box<AstNode>, body: Vec<AstNode> },
    Return(Option<Box<AstNode>>),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Def { name: String, params: Vec<Parameter>, body: Vec<AstNode> },
    DecoratedDef { decorators: Vec<AstNode>, name: String, params: Vec<Parameter>, body: Vec<AstNode> },
    DecoratedClass { decorators: Vec<AstNode>, name: String, bases: Vec<String>, body: Vec<AstNode> },
    Bool(bool),
    BinaryOp { left: Box<AstNode>, op: String, right: Box<AstNode> },
    UnaryOp { op: String, expr: Box<AstNode> },
    Lambda { params: Vec<Parameter>, body: Box<AstNode> },
    Paren(Box<AstNode>),
    None,
    Break,
    Continue,
    List(Vec<AstNode>),
    Class { name: String, bases: Vec<String>, body: Vec<AstNode> },
    Dict(Vec<(AstNode, AstNode)>),
    Set(Vec<AstNode>),
    Tuple(Vec<AstNode>),
    Range { start: Box<AstNode>, end: Box<AstNode>, step: Option<Box<AstNode>> },
    Index { value: Box<AstNode>, index: Box<AstNode> },
    Slice { value: Box<AstNode>, start: Option<Box<AstNode>>, end: Option<Box<AstNode>>, step: Option<Box<AstNode>> },
    Attribute { value: Box<AstNode>, attr: String },
    Import { module: String, alias: Option<String> },
    ImportFrom { module: String, names: Vec<(String, Option<String>)> },
    Pass,
}

#[derive(Debug)]
pub enum Parameter {
    Normal(String),
    Args(String),     // *参数 -> *args
    Kwargs(String),   // **键值对参数 -> **kwargs
}
