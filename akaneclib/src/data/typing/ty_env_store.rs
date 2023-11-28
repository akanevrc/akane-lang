// use std::{
//     cell::RefCell,
//     rc::Rc,
// };
// use anyhow::Result;
// use crate::data::*;

// #[derive(Debug)]
// pub struct TyEnvStore {
//     tvars: Vec<TVarKey>,
//     ty_envs: Vec<Rc<RefCell<TyEnv>>>,
// }

// impl TyEnvStore {
//     pub fn new(tvars: Vec<TVarKey>) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Self {
//             tvars,
//             ty_envs: Vec::new(),
//         }))
//     }

//     pub fn new_ty_env(&mut self) -> Rc<RefCell<TyEnv>> {
//         let ty_env = TyEnv::new(&self.tvars);
//         self.ty_envs.push(ty_env.clone());
//         ty_env
//     }

//     pub fn is_generic(&self) -> bool {
//         self.tvars.len() != 0
//     }

//     pub fn concrete(&mut self, ctx: &SemantizerContext) -> Result<()> {
//         let ty_envs = self.ty_envs.clone();
//         self.ty_envs.clear();
//         for ty_env in ty_envs.iter() {
//             let concretes = ty_env.borrow().concrete(ctx)?;
//             self.ty_envs.extend(concretes);
//         }
//         self.distinct();
//         Ok(())
//     }

//     fn distinct(&mut self) {
//         let mut ty_envs = Vec::new();
//         for i in 0 .. self.ty_envs.len() {
//             let mut unique = true;
//             let ty_env_i = self.ty_envs[i].borrow();
//             for j in i + 1 .. self.ty_envs.len() {
//                 if *ty_env_i == *self.ty_envs[j].borrow() {
//                     unique = false;
//                     break;
//                 }
//             }
//             if unique {
//                 ty_envs.push(self.ty_envs[i].clone());
//             }
//         }
//         self.ty_envs = ty_envs;
//     }

//     pub fn iter(&self) -> impl Iterator<Item = &Rc<RefCell<TyEnv>>> {
//         self.ty_envs.iter()
//     }
// }
