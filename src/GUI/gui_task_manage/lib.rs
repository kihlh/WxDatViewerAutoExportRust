use rusqlite::Connection;
use crate::{global_var_util, handle_dat};

pub(crate) fn remove_export_path(path_string:String) {
    let conn: Connection = Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
    handle_dat::initialize_table(&conn);

    match conn.execute(
        "DELETE FROM export_dir_path WHERE path = ?1",
        [path_string.clone()],
    ) {
        Ok(updated) => {
            global_var_util::update_export_task_item_list();
        }
        Err(err) => {

        }
    }

    let _ = conn.close();
}

pub(crate) fn remove_export_id(id:i32)->bool {
    let mut result = false; 
   
    let conn: Connection = Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
    handle_dat::initialize_table(&conn);

    match conn.execute(
        "DELETE FROM export_dir_path WHERE id = ?1",
        [id.clone()],
    ) {
        Ok(updated) => {
            global_var_util::update_export_task_item_list();
            println!("成功{}",updated);
            result=true;
        }
        Err(err) => {
            eprintln!("失败{}",err);
            result= false;
        }
    }

    let _ = conn.close();
    
    result
}
