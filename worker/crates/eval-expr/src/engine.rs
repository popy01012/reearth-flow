use std::sync::{Arc, RwLock};

use rhai::{Engine as ScriptEngine, Scope as RhaiScope};

use super::module::env::{env_module, scope_module};
use crate::{error::Error, scope::Scope, ShareLock, Value, Vars};

#[derive(Debug, Default, Clone)]
pub struct Engine {
    pub(crate) script_engine: Arc<ScriptEngine>,
    pub(crate) scope: ShareLock<RhaiScope<'static>>,
    pub(crate) vars: ShareLock<Vars>,
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub fn new() -> Self {
        let mut script_engine = ScriptEngine::new();
        script_engine.set_allow_looping(false);
        script_engine.set_allow_anonymous_fn(false);
        script_engine.set_allow_shadowing(false);
        let scope = rhai::Scope::new();
        let module = rhai::exported_module!(env_module);
        script_engine.register_global_module(module.into());

        let module = rhai::exported_module!(scope_module);
        script_engine.register_global_module(module.into());

        let engine = Self {
            script_engine: Arc::new(script_engine),
            scope: Arc::new(RwLock::new(scope)),
            vars: Arc::new(RwLock::new(Vars::new())),
        };
        engine.init();
        engine
    }

    pub fn init(&self) {
        self.scope.write().unwrap().set_or_push("env", self.clone());
    }

    pub fn new_scope(&self) -> Scope {
        Scope::new(self)
    }

    pub fn vars(&self) -> Vars {
        self.vars.read().unwrap().clone()
    }

    pub fn set_scope_var<T: Send + Sync + Clone + 'static>(&self, name: &str, v: &T) {
        self.scope.write().unwrap().set_or_push(name, v.clone());
    }

    pub fn append(&self, vars: &Vars) {
        let env = &mut self.vars.write().unwrap();
        for (name, v) in vars {
            env.entry(name.to_string())
                .and_modify(|i| *i = v.clone())
                .or_insert(v.clone());
        }
    }

    pub fn eval<T: rhai::Variant + Clone>(&self, expr: &str) -> anyhow::Result<T> {
        let scr = Arc::clone(&self.script_engine);
        let mut scope = self
            .scope
            .write()
            .map_err(|_| Error::InternalRuntime("lock".to_string()))?;
        match scr.eval_with_scope::<T>(&mut scope, expr) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(Error::InternalRuntime(format!("{}", err)).into()),
        }
    }

    pub fn eval_ast<T: rhai::Variant + Clone>(&self, ast: &rhai::AST) -> anyhow::Result<T> {
        let scr = Arc::clone(&self.script_engine);
        let mut scope = self
            .scope
            .write()
            .map_err(|_| Error::InternalRuntime("lock".to_string()))?;
        match scr.eval_ast_with_scope::<T>(&mut scope, ast) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(Error::InternalRuntime(format!("{}", err)).into()),
        }
    }

    pub fn eval_scope<T: rhai::Variant + Clone>(
        &self,
        expr: &str,
        scope: &Scope,
    ) -> anyhow::Result<T> {
        let scr = Arc::clone(&self.script_engine);
        let mut scope = scope.scope.write().unwrap();

        match scr.eval_with_scope::<T>(&mut scope, expr) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(Error::InternalRuntime(format!("{}", err)).into()),
        }
    }

    pub fn eval_scope_ast<T: rhai::Variant + Clone>(
        &self,
        ast: &rhai::AST,
        scope: &Scope,
    ) -> anyhow::Result<T> {
        let scr = Arc::clone(&self.script_engine);
        let mut scope = scope.scope.write().unwrap();

        match scr.eval_ast_with_scope::<T>(&mut scope, ast) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(Error::InternalRuntime(format!("{}", err)).into()),
        }
    }

    pub fn compile(&self, expr: &str) -> anyhow::Result<rhai::AST> {
        let scr = Arc::clone(&self.script_engine);
        scr.compile(expr)
            .map_err(|err| Error::InternalRuntime(format!("{}", err)).into())
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let vars = self.vars.read().unwrap();
        vars.get(name).cloned()
    }

    pub fn set(&self, name: &str, value: Value) {
        let mut vars = self.vars.write().unwrap();
        vars.entry(name.to_string())
            .and_modify(|i| *i = value.clone())
            .or_insert(value);
    }

    pub fn remove(&self, name: &str) {
        let mut vars = self.vars.write().unwrap();
        vars.remove(name);
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_eval() {
        let engine = Engine::new();
        let script = r#"
        let v = 5;
        v
        "#;

        let result = engine.eval::<i64>(script);
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_eval_error() {
        let engine = Engine::new();

        let script = r#"
        let v = 5
        v
        "#;

        let script_result = engine.eval::<i64>(script);
        let reuslt = match script_result {
            Ok(..) => false,
            Err(_) => true,
        };
        assert!(reuslt);
    }

    #[test]
    fn test_get() {
        let engine = Engine::new();
        let vars = Vars::from_iter([("a".to_string(), 10.into()), ("b".to_string(), "b".into())]);
        engine.append(&vars);

        let script = r#"
        let a = env.get("a");
        a
        "#;
        let result = engine.eval::<i64>(script);
        print!("hogehoge {:?}", result);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_scope_eval() {
        let engine = Engine::new();
        let scope = engine.new_scope();
        let script = r#"
        let v = 5;
        v
        "#;

        let result = scope.eval::<i64>(script);
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn env_room_get() {
        let engine = Engine::new();
        let scope = engine.new_scope();

        let vars = Vars::from_iter([("a".to_string(), 10.into()), ("b".to_string(), "b".into())]);
        scope.append(&vars);

        let script = r#"
        let a = env.get("a");
        a
        "#;
        let result = scope.eval::<i64>(script);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_scope_share_global_vars() {
        let engine = Engine::new();

        engine.set("abc", serde_json::json!(1.5));
        let scope = engine.new_scope();
        let script = r#"
        let v = 5;
        let v2 = env.get("abc");
        v2
        "#;

        let result = scope.eval::<f64>(script);
        assert_eq!(result.unwrap(), 1.5);
    }
}