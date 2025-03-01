use std::{collections::HashMap};
use crate::{parser::Expr, tokens::{Token, TokenType}};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LocalVar {
    position: HashMap<String, usize>,
    value: Vec<f32>
}

pub struct Environment<'a> {
    name: &'a str,
    import_names: Vec<String>,
    var_hash: HashMap<String, f32>,
    vec_hash: HashMap<String, Vec<f32>>,
    matrix_hash: HashMap<String, Vec<Vec<f32>>>,
    fn_store: HashMap<String, Rc<RefCell<Vec<Rc<RefCell<Expr>>>>>>,
    fn_local: HashMap<String, LocalVar>,
    
}

impl  Environment<'_> {
    pub fn new(name: &str) -> Environment {
       let mut new = Environment {name:name,import_names: Vec::new(), var_hash: HashMap::new(), vec_hash: HashMap::new(),matrix_hash: HashMap::new(), fn_store: HashMap::new(), fn_local: HashMap::new()};
        new.var_hash.insert("e".to_string(), 2.718281);
        new.var_hash.insert("pi".to_string(), 3.141592);
        new
    }
    pub fn check_import<'a>(&'a mut  self, file_name: &String) -> bool {
        if self.import_names.contains(file_name) {
            false
        } else {
            self.import_names.push(file_name.clone());
            true
        }
    }

    pub fn add_matrix(&mut self, mat_name: String, mat_value: Vec<Vec<f32>>) {
        if self.matrix_hash.contains_key(&mat_name){
             self.matrix_hash.remove(&mat_name);
        }
        self.matrix_hash.insert(mat_name, mat_value);
    }

    pub fn use_matrix<'a>(&'a self, mat_name: &String) -> Vec<Vec<f32>> {
        self.matrix_hash.get(mat_name).unwrap().clone()
    }

    pub fn find_var_value<'a>(&'a self, var_name: &String,fn_name: &String) -> &'a f32 {
        if fn_name != &"_".to_string() {
            
            if self.fn_local.get(fn_name).unwrap().position.contains_key(var_name) {
                
                let index = self.fn_local.get(fn_name).unwrap().position.get(var_name).unwrap();
                &self.fn_local.get(fn_name).unwrap().value[*index]
            } else {
                self.var_hash.get(var_name).unwrap()
            }
        } else {
            self.var_hash.get(var_name).unwrap()
        }
        
    }
    
    pub fn add_var(&mut self, var_name: String, var_value: f32) {
        if self.var_hash.contains_key(&var_name){
           let old = self.var_hash.get_mut(&var_name).unwrap();
           *old = var_value;
        }
        else {
            self.var_hash.insert(var_name, var_value);
        }
    }

    pub fn find_vec_value<'a>(&'a self, vec_name: &String, index: usize) -> &'a f32 {
        &self.vec_hash.get(vec_name).unwrap()[index]
    }
    pub fn push_vec_value<'a>(&'a mut self, vec_name: &String, index: usize, value: f32) -> &'a f32 {
        self.vec_hash.get_mut(vec_name).unwrap()[index] = value;
        &self.vec_hash.get(vec_name).unwrap()[index]
    }
    pub fn pop_vec_value<'a>(&'a mut self, vec_name: &String, index: usize) ->  f32 {
        self.vec_hash.get_mut(vec_name).unwrap().remove(index)
        
    }
    pub fn snap_vec_value<'a>(&'a mut self, vec_name: &String, value: f32) ->  f32 {
        self.vec_hash.get_mut(vec_name).unwrap().push(value);
        value
    }
    pub fn len_vec<'a>(&'a mut self, vec_name: &String) ->  i32 {
        self.vec_hash.get_mut(vec_name).unwrap().len() as i32
        
    }

    pub fn add_vec(&mut self, vec_name: String, vec_value: Vec<f32>) {
        if self.vec_hash.contains_key(&vec_name){
            let old = self.vec_hash.get_mut(&vec_name).unwrap();
            *old = vec_value;

        } else {
            self.vec_hash.insert(vec_name, vec_value);
        }
    }

    pub fn add_fn(&mut self, fn_name: String, inner_exp: Vec<Rc<RefCell<Expr>>>, params: Vec<String>) {
        if self.fn_local.contains_key(&fn_name) {
           self.fn_store.remove(&fn_name);
           //self.fn_store.remove(&fn_name);
        }
        self.fn_store.insert(fn_name.clone(), Rc::new(RefCell::new(inner_exp)));
        self.fn_local.insert(fn_name.clone(), LocalVar {position: HashMap::new(), value: Vec::new()});
        let mut index:usize = 0;
        for i in params.iter() {
            let local = self.fn_local.get_mut(&fn_name).unwrap();
            local.position.insert(i.to_string(), index);
            local.value.push(0.0);
            index += 1;
        }
    }
    pub fn find_local_var<'a>(&'a self, fn_name: &String, local_var: &String) -> &'a f32 {
        let varlocal = self.fn_local.get(fn_name).unwrap();
        let index = varlocal.position.get(local_var).unwrap();
        &varlocal.value[*index]
    }
    pub fn find_fn_expr(&mut self, fn_name: &String) -> Vec<Rc<RefCell<Expr>>> {
        self.fn_store.get(fn_name).unwrap().borrow().clone()
    }
    pub fn setup_local(&mut self, fn_name: String,params: Vec<f32>) {
        let hash = self.fn_local.get_mut(&fn_name).unwrap();
        let scope_keys: Vec<String> = hash.position.keys().cloned().collect();
        for name in scope_keys.iter(){
            hash.value[*hash.position.get(name).unwrap()] = params[*hash.position.get(name).unwrap()];
        };
    }
    pub fn fn_exist(&mut self, fn_name: &String) -> bool {
        self.fn_local.contains_key(fn_name)
    }
    pub fn remove(&mut self,var_name: &String) {
        self.var_hash.remove(var_name);
    }
}