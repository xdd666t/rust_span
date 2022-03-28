//允许未使用的变量
#![allow(unused_variables)]
//允许未使用的代码
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

///配置模块
pub mod config;
///服务
pub mod service;
///数据实体
pub mod data;
///api接口模块
pub mod web;