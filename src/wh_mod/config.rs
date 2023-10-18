

// 判断当前是否处于开发者模式
pub fn is_developer() -> bool {

    // std::env::var("_debug").is_ok()
    !false
}

// 编译版本是 52破解专版
pub fn is_build_52pojie() -> bool {
    false
}

// 是否对显示的数据进行消敏
pub fn is_show_mask() -> bool {
    is_show_dome()||true
}

// 是否在选择对象后自动显示最近十张照片
pub fn is_click_open_preview() -> bool {
    false
}

// 演示模式
pub fn is_show_dome() -> bool {
    false
}

// 显示国际货币捐赠
pub fn is_show_token_donate() -> bool {
    (!is_build_52pojie()&&false)||is_developer()
}

