use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use crate::domain::domain::{SysRes, SysRole, SysRoleRes, SysUserRole};
use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::service::CONTEXT;
use std::collections::HashMap;
use crate::util::string::IsEmpty;
use crate::domain::vo::SysRoleVO;

///角色服务
pub struct SysRoleService {}

impl SysRoleService {
    ///角色分页
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let wrapper = CONTEXT.rbatis.new_wrapper()
            .do_if(!arg.name.is_empty(),|w|w.like("name",&arg.name))
            .is_null("parent_id")
            .order_by(false, &["create_date"]);
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysRole>(
                "",
                &wrapper,
                &PageRequest::new(arg.page_no.unwrap_or(0), arg.page_size.unwrap_or(10)),
            )
            .await?;
        let all_role = self.finds_all_map().await?;
        let mut datas = vec![];
        for x in data.records {
            let mut vo = SysRoleVO::from(x);
            self.loop_find_childs(&mut vo, &all_role);
            datas.push(vo);
        }
        let new_page = Page {
            records: datas,
            total: data.total,
            pages: data.pages,
            page_no: data.page_no,
            page_size: data.page_size,
            search_count: data.search_count,
        };
        Ok(new_page)
    }


    /// 查找role数组
    pub async fn finds_all(&self) -> Result<Vec<SysRole>> {
        //TODO 查找的全部数据缓存于Redis，同时 remove，edit方法调用时刷新redis缓存
        CONTEXT.rbatis.fetch_list("").await
    }

    pub async fn finds_all_map(&self)->Result<HashMap<String,SysRole>>{
        let all = self.finds_all().await?;
        let mut result = HashMap::new();
        for x in all {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        return Ok(result);
    }

    ///角色添加
    pub async fn add(&self, arg: &RoleAddDTO) -> Result<(u64, String)> {
        let role = SysRole {
            id: Some(
                rbatis::plugin::snowflake::async_snowflake_id()
                    .await
                    .to_string(),
            ),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: Some(0),
            create_date: Some(NaiveDateTime::now()),
        };
        Ok((
            CONTEXT.rbatis.save("", &role).await?.rows_affected,
            role.id.clone().unwrap(),
        ))
    }

    ///角色修改
    pub async fn edit(&self, arg: &RoleEditDTO) -> Result<u64> {
        let mut role = SysRole {
            id: arg.id.clone(),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: None,
            create_date: None,
        };
        CONTEXT.rbatis.update_by_id("", &mut role).await
    }

    ///角色删除
    pub async fn remove(&self, id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_id::<SysRole>("", &id.to_string())
            .await
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().r#in("id", ids))
            .await
    }

    pub async fn find(&self, id: &String) -> Result<SysRole> {
        CONTEXT.rbatis.fetch_by_id("", id).await
    }

    pub async fn find_role_res(&self, ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().r#in("role_id", ids))
            .await
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &HashMap<String, SysRes>,
    ) -> Result<Vec<String>> {
        let user_roles: Vec<SysUserRole> = CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().eq("user_id", user_id))
            .await?;
        let role_res = self
            .find_role_res(&field_vec!(&user_roles, role_id))
            .await?;
        let res = CONTEXT
            .sys_res_service
            .finds_layer(&field_vec!(&role_res, res_id), &all_res)
            .await?;
        let permissons = field_vec!(&res, permission);
        return Ok(permissons);
    }

    ///死循环找出父-子 关联关系数组
    pub fn loop_find_childs(&self, arg: &mut SysRoleVO, all: &HashMap<String, SysRole>) {
        let mut childs: Option<Vec<SysRoleVO>> = None;
        for (key, x) in all {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
                let mut item = SysRoleVO::from(x.clone());
                self.loop_find_childs(&mut item, all);
                match &mut childs {
                    Some(childs) => {
                        childs.push(item);
                    }
                    None => {
                        let mut vec = vec![];
                        vec.push(item);
                        childs = Some(vec);
                    }
                }
            }
        }
        if childs.is_some() {
            arg.childs = childs;
        }
    }
}
