use std::sync::Arc;
use value::Value;
use value::_Tuple;
use core::module;
use core::interpreter::FunctionContext;
use core::interpreter::ThreadContext;
use std::sync::Weak;


pub struct Function {
    pub modu: Weak<module::Module>,
    pub name: String,
    pub expr: Vec<Value>,
    pub env: Option<Arc<FunctionContext>>,
}

pub struct Native {
    pub name: String,
    pub fp: fn(&Arc<ThreadContext>, _Tuple) -> Value,
}

pub trait Call {
    fn call_function(&self, ic: &Arc<ThreadContext>, args: _Tuple) -> Value;
}

