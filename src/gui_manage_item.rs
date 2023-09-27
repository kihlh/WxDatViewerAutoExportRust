#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    unused_mut,
    unused_must_use,
    unused_assignments,
    non_snake_case,
    non_camel_case_types
)]

use crate::{
    global_var, handle_dat, libWxIkunPlus,
    util::{str_eq_str, Sleep},
};
use fltk::{
    app::{self, handle},
    button::{self, Button},
    draw::font,
    enums::{self, Color, Cursor, Event, Font, FrameType, LabelType},
    frame::Frame,
    group::{self, Flex, Group},
    image,
    input::{InputType, IntInput},
    prelude::*,
    text::TextDisplay,
    tree,
    window::{self, DoubleWindow, Window},
};
use fltk_table::{SmartTable, TableOpts};
use rusqlite::Connection;
use std::{
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{CloseWindow, SetActiveWindow};

// 按钮基本都是以坐标判断的
struct PointExistHasmap {
    // 关闭按钮
    quit: bool,
    // 下个
    next: bool,
    // 上个
    backup: bool,
    // 卡片 1
    card_01: bool,
    // 卡片 2
    card_02: bool,
    // 卡片 3
    card_03: bool,

    // 卡片 1
    rm_card_01: bool,
    // 卡片 2
    rm_card_02: bool,
    // 卡片 3
    rm_card_03: bool,

    // 所有按钮
    existCursor: bool,
}

// 判断鼠标坐标是否在此元素内
fn getFormPointSpace(x: i32, y: i32) -> PointExistHasmap {
    // 输入: x,y,w,y
    macro_rules! check_point_in_space {
        ($xx:expr, $yy:expr, $width:expr, $height:expr) => {
            x > $xx && x < $xx + $width && y > $yy && y < $yy + $height
        };
    }

    let mut point_exist_hasmap = PointExistHasmap {
        quit: false,
        next: false,
        backup: false,
        card_01: false,
        card_02: false,
        card_03: false,
        rm_card_01: false,
        rm_card_02: false,
        rm_card_03: false,
        existCursor: false,
    };

    point_exist_hasmap.quit = check_point_in_space!(273, 17, 40, 40);
    point_exist_hasmap.backup = check_point_in_space!(41, 492, 121, 44);
    point_exist_hasmap.next = check_point_in_space!(171, 492, 121, 44);
    point_exist_hasmap.card_01 = check_point_in_space!(20, 90, 286, 110);
    point_exist_hasmap.card_02 = check_point_in_space!(20, 216, 286, 110);
    point_exist_hasmap.card_03 = check_point_in_space!(20, 345, 286, 110);

    // 移除
    point_exist_hasmap.rm_card_01 = check_point_in_space!(230, 164, 65, 22);
    point_exist_hasmap.rm_card_02 = check_point_in_space!(230, 291, 65, 22);
    point_exist_hasmap.rm_card_03 = check_point_in_space!(230, 419, 65, 22);

    let mut win_coords_cursor_list = vec![
        point_exist_hasmap.quit,
        point_exist_hasmap.backup,
        point_exist_hasmap.next,
        point_exist_hasmap.rm_card_01,
        point_exist_hasmap.rm_card_02,
        point_exist_hasmap.rm_card_03,
    ];

    let mut existCursor = false;
    for value in win_coords_cursor_list.iter() {
        // 关闭按钮
        if *(value) {
            existCursor = true;
        }
    }

    point_exist_hasmap.existCursor = existCursor;

    return point_exist_hasmap;
}

// 设置背景为图片（主视图）
fn setInterfaceBackgroundImage(appMainWin: &mut window::DoubleWindow) -> Frame {
    let background_image = image::PngImage::from_data(include_bytes!("./assets/manage_itme.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(326, 0).center_of(appMainWin);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));
    return frame;
}

// 剪裁路径 字数不超过XXX
pub fn get_form_slice_path(path: String, max_size: usize) -> Result<String, String> {
    let mut new_path: String = String::new();
    let mut size = 0;

    for ch in path.chars() {
        new_path.push(ch);
        size = size + 1;
        if (size > max_size) {
            break;
        }
        // println!("{}", ch);
    }

    // let mut size = path.len();

    // if size > max_size {
    // size = max_size;
    // return Err("Input string exceeds maximum size".to_string());
    // }

    // let new_path: String = path[..size].to_string();
    Ok(new_path)
}

// 卡片会回传为这个参数 用来控制
struct CardItme {
    background: Frame,
    path: Frame,
    ouput: Frame,
    name: Frame,
    status_err: Frame,
    status_ok: Frame,
    main: DoubleWindow,
    nameStr: String,
    pathStr: String,
    ouputStr: String,
    remove: bool,
}

// 任务卡片 （不会动态创建 而是引用同一个）
fn create_card(x: i32, y: i32, name: String, path: String, ouput: String) -> CardItme {
    let mut card_win = window::Window::new(x, y, 289, 113, None);
    card_win.set_color(Color::from_rgb(24, 24, 24));

    // 背景
    let background_image = image::PngImage::from_data(include_bytes!("./assets/card.png"))
        .expect("set main icon error");
    let mut frame = Frame::default().with_size(289, 0).center_of(&card_win);
    frame.set_frame(FrameType::EngravedBox);
    frame.set_image(Some(background_image));

    // 文本-> path
    let mut frame_info_path = Frame::new(17, 18, 200, 18, None); //.with_size(200, 18).left_of(&flex,1);
                                                                 // let slice_path =path.get(0..35).unwrap();

    frame_info_path.set_label(&get_form_slice_path(path.clone(), 20).unwrap_or(path.clone()));
    frame_info_path.set_label_size(12);
    frame_info_path.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_path.resize(32, 18, 200, 18);

    // 文本-> ouput
    let mut frame_info_ouput = Frame::new(17, 18, 200, 18, None); //::default().with_size(200, 18).left_of(&flex,1);
    frame_info_ouput.set_label(&get_form_slice_path(ouput.clone(), 20).unwrap_or(ouput.clone()));
    frame_info_ouput.set_label_size(12);
    frame_info_ouput.set_label_color(Color::from_rgb(186, 186, 186));
    frame_info_ouput.resize(32, 44, 200, 18);

    // 文本-> name
    let mut frame_info_name = Frame::default().with_size(200, 22);
    frame_info_name.set_label(&get_form_slice_path(name.clone(), 7).unwrap_or(name.clone()));
    frame_info_name.set_label_size(15);
    frame_info_name.set_label_color(Color::from_rgb(255, 255, 255));
    frame_info_name.resize(32, 80, 200, 22);

    let card_ok = image::PngImage::from_data(include_bytes!("./assets/card_ok.png"))
        .expect("set main icon error");

    let card_error = image::PngImage::from_data(include_bytes!("./assets/card_error.png"))
        .expect("set main icon error");

    let mut card_status_ok = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_ok.set_frame(FrameType::EngravedBox);
    card_status_ok.set_image(Some(card_ok));
    card_status_ok.set_pos(244, 11);

    let mut card_status_error = Frame::default().with_size(26, 26).center_of(&card_win);
    card_status_error.set_frame(FrameType::EngravedBox);
    card_status_error.set_image(Some(card_error));
    card_status_error.set_pos(244, 11);
    card_status_error.hide();

    card_win.end();

    let move_frame_info_path = frame_info_path.clone();
    let mut move_card_status_error = card_status_error.clone();
    let mut move_card_status_ok = card_status_ok.clone();

    return CardItme {
        main: card_win,
        background: frame,
        path: frame_info_path,
        ouput: frame_info_ouput,
        name: frame_info_name,
        status_err: card_status_error,
        status_ok: card_status_ok,
        ouputStr: ouput,
        nameStr: name,
        pathStr: path,
        remove: false,
    };
}

#[macro_export]
macro_rules! console_log {
    ($message:expr) => {
        println!("{}", $message);
        global_var::insert_vec_str("console_log", &[$message]);
        // handle_dat::push_console_message($message);
    };
}

// 主窗口
pub fn ManageItmeMain() {
    if (global_var::get_bool("gui::open::manage_item")) {
        if let Some(mut wins) =
            app::widget_from_id("gui::DoubleWindow::manage_item::main") as Option<DoubleWindow>
        {
            wins.show();
            wins.set_visible_focus();
        }

        return ;
    }

    global_var::set_bool("gui::open::manage_item", true);

    let mut win: DoubleWindow = DoubleWindow::new(0, 0, 326, 554, "管理分组");
    win.set_color(Color::from_rgb(24, 24, 24));
    win.set_border(false);
    win.set_id ("gui::DoubleWindow::manage_item::main");

    let mut export_dir_path_list: Vec<global_var::ExportDirItme> =
        global_var::get_export_dir_itme_list();

    // 偏移量
    let mut offset = 0;

    fltk::app::set_scrollbar_size(3);
    fltk::app::set_selection_color(24, 24, 24);
    setInterfaceBackgroundImage(&mut win);

    let mut default_itme_text = "没有数据...";

    macro_rules! create_card_itme {
        ($card_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
                default_itme_text.to_owned(),
            )
        };

        ($card_id:expr,$itme_id:expr) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                export_dir_path_list[$itme_id].name.clone().to_owned(),
                export_dir_path_list[$itme_id].path.clone().to_owned(),
                export_dir_path_list[$itme_id].ouput.clone().to_owned(),
            )
        };

        ($card_id:expr,$name:expr,$path:expr,$ouput:expr ) => {
            create_card(
                18,
                (if ($card_id == 1) {
                    88
                } else if ($card_id == 2) {
                    215
                } else {
                    342
                }),
                $name.to_owned(),
                $path.to_owned(),
                $ouput.to_owned(),
            )
        };
    }

    let mut card_01 = if export_dir_path_list.len() >= 1 {
        create_card_itme!(1, 0)
    } else {
        create_card_itme!(1)
    };
    let mut card_02 = if export_dir_path_list.len() >= 2 {
        create_card_itme!(2, 1)
    } else {
        create_card_itme!(2)
    };
    let mut card_03 = if export_dir_path_list.len() >= 3 {
        create_card_itme!(3, 2)
    } else {
        create_card_itme!(3)
    };

    if export_dir_path_list.len() < 2 {
        card_02.main.hide();
    } else {
        // offset = 1;
    }

    if export_dir_path_list.len() < 3 {
        card_03.main.hide();
    } else {
        // offset = 2;
    }

    // card_01
    macro_rules! push_card_itme {
        ($card_itme:expr,$name:expr,$path:expr,$ouput:expr) => {
            $card_itme
                .path
                .set_label(&get_form_slice_path($path.clone(), 20).unwrap_or($path.clone()));
            $card_itme.path.set_label_size(12);
            $card_itme
                .path
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.path.resize(32, 18, 200, 18);
            $card_itme.pathStr = $path;

            $card_itme
                .ouput
                .set_label(&get_form_slice_path($ouput.clone(), 20).unwrap_or($ouput.clone()));
            $card_itme.ouput.set_label_size(12);
            $card_itme
                .ouput
                .set_label_color(Color::from_rgb(186, 186, 186));
            $card_itme.ouput.resize(32, 44, 200, 18);
            $card_itme.ouputStr = $ouput;

            $card_itme
                .name
                .set_label(&get_form_slice_path($name.clone(), 7).unwrap_or($name.clone()));
            $card_itme.name.set_label_size(15);
            $card_itme
                .name
                .set_label_color(Color::from_rgb(255, 255, 255));
            $card_itme.name.resize(32, 80, 200, 22);
            $card_itme.nameStr = $name;

            $card_itme.status_ok.show();
            $card_itme.status_err.hide();
            $card_itme.remove = false;
        };

        ($card_id:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    export_dir_path_list[$itme_id].name.clone().to_owned(),
                    export_dir_path_list[$itme_id].path.clone().to_owned(),
                    export_dir_path_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };

        ($card_id:expr,$itme_list:expr,$itme_id:expr) => {
            if $card_id == 1 {
                push_card_itme!(
                    card_01,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 2 {
                push_card_itme!(
                    card_02,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
            if $card_id == 3 {
                push_card_itme!(
                    card_03,
                    $itme_list[$itme_id].name.clone().to_owned(),
                    $itme_list[$itme_id].path.clone().to_owned(),
                    $itme_list[$itme_id].ouput.clone().to_owned()
                );
            }
        };
    }

    win.clone().center_screen();
    win.end();
    win.show();
    let mut num_items_to_take = 3;

    win.handle({
        let mut x = 0;
        let mut y = 0;
        let mut point_exist_hasmap = getFormPointSpace(x, y);
        let mut has_show = false;
        // let mut move_
        move |win, ev| match ev {
            enums::Event::Show => {
                win.set_visible_focus();
                libWxIkunPlus::setwinVisible(win.raw_handle() as i128 , true);
                println!("hwnd -> :  {}",win.raw_handle() as i128);
                true
            }

            enums::Event::Push => {
                // 关闭按钮
                if (point_exist_hasmap.quit) {
                    let mut hwnd = win.raw_handle() as i128;
                    win.clear();
                    global_var::set_bool("gui::open::manage_item", false);
                    libWxIkunPlus::setwinVisible(hwnd, false);                  
                    libWxIkunPlus::closeWindow(hwnd, true);
                    
                    println!("closeWindow hwnd -> :  {}",hwnd);
                }

                macro_rules! handle_point_card {
                    ($rm_card_id:expr, $card_itme:expr) => {
                        if ($rm_card_id && !$card_itme.remove) {
                          
                      
                            let mut path_string = $card_itme.pathStr.clone();
                            let conn: Connection =
                                Connection::open("ikun_user_data.db").expect("无法 创建/打开 数据库");
                            handle_dat::initialize_table(&conn);
                            match conn.execute(
                                "DELETE FROM export_dir_path WHERE path = ?1",
                                [path_string.clone()],
                            ) {
                                Ok(updated) => {
                                    $card_itme.status_err.show();
                                    $card_itme.status_ok.hide();
                                    // println!(
                                    //     "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    console_log!(format!(
                                        "[移除] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                    export_dir_path_list.clear();
                                    for cat in global_var::update_export_dir_itme_list() {
                                        export_dir_path_list.push(cat);
                                    }

                                }
                                Err(err) => {
                                    // $card_itme.status_err.show();
                                    // $card_itme.status_ok.hide();
                                    // println!(
                                        // "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        // $card_itme.nameStr,
                                        // $card_itme.path.label()
                                    // );
                                    $card_itme.remove = true;
                                    // handle_dat::push_console_message(format!(
                                    //     "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                    //     $card_itme.nameStr,
                                    //     $card_itme.path.label()
                                    // ));
                                    console_log!(format!(
                                        "[移除失败] 移除的文件夹是: 名称：{}  路径：{}",
                                        $card_itme.nameStr,
                                        $card_itme.path.label()
                                    ));

                                }
                            }
            
                            let _ = conn.close();

                            
                        }
                    };
                }
                
                handle_point_card!(point_exist_hasmap.rm_card_01, card_01);
                handle_point_card!(point_exist_hasmap.rm_card_02, card_02);
                handle_point_card!(point_exist_hasmap.rm_card_03, card_03);
            
                if (point_exist_hasmap.next){
                    let mut start_index = 0;
                  
                    let mut end_index = offset+3;

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }
                    start_index = offset;
                    offset= end_index ;

                    if(start_index==end_index){
                        start_index= end_index-3;
                        if(start_index>9999999){
                            start_index=0;
                        }
                    }
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                        if(indexof>export_dir_path_list.len()-1){
                            break;
                        }

                        push_card_itme!(index+1,indexof);

                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        
                        index = index+1;
                       
                    }

                    println!("next-> {} -> {}",start_index,end_index);

                }
                if (point_exist_hasmap.backup){
                    // let start_index = if offset >= 3 { offset - 3 } else { 0 };
                    // let end_index = if offset + 3 < get_item.len() { offset + 3 } else { get_item.len() };
                
                    let mut start_index = 0;
            
                    
                    let mut end_index = offset-3;
                    
                    // 这里有个usize溢出问题 负数会巨大
                    if(end_index<999999){
                        end_index = 3;
                    }
                    
                    // end 不低于三个 除非没有那么多
                    if(end_index<3){
                        end_index=if export_dir_path_list.len()<=3 {export_dir_path_list.len()} else {3};
                    }

                    // 偏移量不能超出数组
                    if(end_index >= export_dir_path_list.len()){
                        end_index = export_dir_path_list.len();
                    }

                    if(end_index-start_index>3){
                        start_index= start_index-3;
                    }

                    if(start_index<999999){
                        start_index=0;
                    }
                    
                    let mut index = 0;
                    loop{
                        let mut indexof = start_index+index;
                            if(indexof>export_dir_path_list.len()-1){
                            break;
                        }
                        push_card_itme!(index+1,indexof);
                        if(index>=3){
                            card_03.main.clone().show();
                            break;
                        }
                        index = index+1;
                    }
                    
                    offset= end_index ;

                    println!("backup-> {} -> {}",start_index,end_index);

                }

                true
            }

            enums::Event::Move => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                point_exist_hasmap = getFormPointSpace(x, y);
                // -处理鼠标图标的逻辑

                if (point_exist_hasmap.existCursor) {
                    win.clone().set_cursor(Cursor::Hand);
                } else {
                    win.clone().set_cursor(Cursor::Default);
                }

                true
            }

            // enums::Event:
            enums::Event::Drag => {
                if (y < 69) {
                    win.clone()
                        .set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }

                true
            }
            _ => false,
        }
    });

    libWxIkunPlus::setWinIcon(win.raw_handle() as i128);
    // libWxIkunPlus::setWinTop(win.raw_handle() as i128, true);
}
