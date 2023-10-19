#ifndef defined(HMC_IMPORT_TRAY_H)
#define HMC_IMPORT_TRAY_H

#include <iostream>
#include <string>
#include <windows.h>
#include <thread>
#include <unordered_map>
#include <vector>
#include <iostream>
#include <psapi.h>
#include <functional>

using namespace std;

namespace hmc_tray

{

#define WM_SYSICON (WM_USER + 1)
    static NOTIFYICONDATAA nid;
#define HMC_CHECK_CATCH catch (char *err){};
#define HMC_EMIT_ENVINFO(assert_IF)                            \
    for (auto &&onEvent : ON_EVENT)                            \
    {                                                          \
        if (onEvent.first == assert_IF)                        \
        {                                                      \
            for (size_t i = 0; i < onEvent.second.size(); i++) \
            {                                                  \
                onEvent.second[i]();                           \
            }                                                  \
        }                                                      \
    }                                                          \
                                                               \
    for (auto &&onEvent : ONCE_EVENT)                          \
    {                                                          \
        if (onEvent.first == assert_IF)                        \
        {                                                      \
            for (size_t i = 0; i < onEvent.second.size(); i++) \
            {                                                  \
                onEvent.second[i]();                           \
            }                                                  \
            ONCE_EVENT.clear();                                \
        }                                                      \
    };

#define HMC_EMIT_ENVINFO2(assert_IF, assert_IF02)                       \
    for (auto &&onEvent : ON_EVENT)                                     \
    {                                                                   \
        if (onEvent.first == assert_IF || onEvent.first == assert_IF02) \
        {                                                               \
            for (size_t i = 0; i < onEvent.second.size(); i++)          \
            {                                                           \
                onEvent.second[i]();                                    \
            }                                                           \
        }                                                               \
    }                                                                   \
                                                                        \
    for (auto &&onEvent : ONCE_EVENT)                                   \
    {                                                                   \
        if (onEvent.first == assert_IF || onEvent.first == assert_IF02) \
        {                                                               \
            for (size_t i = 0; i < onEvent.second.size(); i++)          \
            {                                                           \
                onEvent.second[i]();                                    \
            }                                                           \
            ONCE_EVENT.clear();                                         \
        }                                                               \
    };

    HWND PuppetTrayWindowHwnd;

    namespace chMenuType
    {
        // 按钮类型
        typedef enum
        {
            check = MF_CHECKED,
            separator = MF_SEPARATOR,
            // radio = MFT_RADIOCHECK,
            menu = 60409,
        } chMenuType;
    }

    string __hmc_trayInfo = "";
    string __hmc_title = "hmc-puppet-tray-window";
    string __hmc_className = "hmc-puppet-tray-window-class";
    bool ___Start_hmc_Tray = false;
    long long __tray_next_id = 0;
    // 按钮
    struct chMenuItem
    {
        // 显示名称
        string name;
        // js的文本id
        string id;
        // 自动分配的id 留 0
        int messageID;
        // 按钮类型
        chMenuType::chMenuType type;
        // 禁用
        bool disable;
        // 子按钮列表
        vector<int> menuList;
        // 主按钮 如果是子按钮 不会被显示出来  只有主按钮中包含此键的id才会被包含到子按钮里显示
        bool rootMenu;
        // 是否显示此按钮
        bool show;
        // 是否选定
        bool select;
        chMenuItem() : messageID(0)
        {
            messageID = -1;
            select = false;
            menuList = {};
            disable = false;
            rootMenu = true;
            show = true;
            name = "unknown";
            id = "unknown" + to_string(messageID);
            type = chMenuType::menu;
        }
    };
    std::vector<chMenuItem> __MenuList;
    std::thread *__tray_worker;
    int __openPuppetTrayWindow();
    namespace _HMC__EVENT
    {
        unordered_map<string, vector<function<void()>>> ON_EVENT;
        unordered_map<string, vector<function<void()>>> ONCE_EVENT;

        /**
         * @brief 处理事件
         *
         * @param lParam
         */
        void emit(LPARAM lParam)
        {
            switch (lParam)
            {
                // 鼠标右键按下
            case WM_RBUTTONDOWN:
            {

                HMC_EMIT_ENVINFO("rightButtonDown");
                break;
            }
                // 鼠标右键松开
            case WM_RBUTTONUP:
            {
                HMC_EMIT_ENVINFO("rightButtonUp");
                break;
            }
                // 鼠标左键按下
            case WM_LBUTTONDOWN:
            {
                HMC_EMIT_ENVINFO("leftButtonDown");

                break;
            }
                // 鼠标左键松开
            case WM_LBUTTONUP:
            {
                HMC_EMIT_ENVINFO2("click", "leftButtonUp");

                break;
            }
                // 鼠标左键双击
            case WM_LBUTTONDBLCLK:
            {
                HMC_EMIT_ENVINFO2("dblclick", "leftButtonDoubleClick");
                break;
            }
                // 鼠标浮动
            case WM_MOUSEMOVE:
            {
                HMC_EMIT_ENVINFO2("move", "mouseMove");
                break;
            }
            // 中键点击
            case WM_MBUTTONDOWN:
            {
                HMC_EMIT_ENVINFO("middleClick");
                break;
            }
            }
        }

        /**
         * @brief 处理按钮响应
         *
         * @param keyID
         */
        void emit(string keyID, chMenuItem &menuItem)
        {
            bool isOnOK = false;
            for (auto &&event : ON_EVENT)
            {

                if (event.first == keyID)
                {
                    for (size_t i = 0; i < event.second.size(); i++)
                    {
                        event.second[i]();
                    }
                    isOnOK = true;
                }
            }

            bool isOnCeOK = false;
            for (auto &&event : ONCE_EVENT)
            {
                if (event.first == keyID)
                {
                    for (size_t i = 0; i < event.second.size(); i++)
                    {
                        event.second[i]();
                    }
                    isOnCeOK = true;
                    event.second.clear();
                }
            }

            // cout << "click->" << keyID << "   key->" << menuItem.messageID << "   name->" << menuItem.name << endl;
        }

        // 点击了按钮
        void clickButtonItem(WORD IDKey)
        {
            string id = string();
            for (size_t i = 0; i < __MenuList.size(); i++)
            {
                auto Menu = __MenuList[i];
                if (Menu.messageID == IDKey)
                {
                    id.clear();
                    id.append(Menu.id);
                    if (Menu.type == chMenuType::check)
                    {
                        __MenuList[i].select = !Menu.select;
                    }
                    emit(id, __MenuList[i]);
                }
            }
        }

    }

    /**
     * @brief 监听
     *
     * @param eventName
     * @param fnc
     */
    void on(string eventName, std::function<void()> fnc)
    {

        vector<function<void()>> funList = {};

        if (_HMC__EVENT::ON_EVENT.find(eventName) == _HMC__EVENT::ON_EVENT.end())
        {
            _HMC__EVENT::ON_EVENT.insert(std::make_pair(eventName, funList));
        }

        for (auto &&EVENT : _HMC__EVENT::ON_EVENT)
        {
            if (EVENT.first == eventName)
            {
                EVENT.second.push_back(fnc);
            }
        }
    }

    /**
     * @brief 单次监听
     *
     * @param eventName
     * @param fnc
     */
    void once(string eventName, std::function<void()> fnc)
    {
        vector<function<void()>> funList = {};

        if (_HMC__EVENT::ONCE_EVENT.find(eventName) == _HMC__EVENT::ONCE_EVENT.end())
        {
            _HMC__EVENT::ONCE_EVENT.insert(std::make_pair(eventName, funList));
        }

        for (auto &&EVENT : _HMC__EVENT::ONCE_EVENT)
        {
            if (EVENT.first == eventName)
            {
                EVENT.second.push_back(fnc);
            }
        }
    }

    BOOL setTrayIcon(HICON hNewIcon, int index = 0)
    {
        try
        {
            nid.hIcon = hNewIcon;
            return Shell_NotifyIconA(NIM_MODIFY, &nid);
        }
        HMC_CHECK_CATCH;

        return false;
    }

    BOOL setTrayIcon(string Icons, int index = 0)
    {
        try
        {
            HICON hNewIcon = NULL; // 声明一个HICON句柄
            hNewIcon = ExtractIconA(GetModuleHandleA(NULL), (LPCSTR)Icons.c_str(), (UINT)index);
            if (hNewIcon != NULL)
            {
                nid.hIcon = hNewIcon;
                return Shell_NotifyIconA(NIM_MODIFY, &nid);
            }
        }
        HMC_CHECK_CATCH;

        return false;
    }

    /**
     * @brief 修改tray的图标（从当前可执行文件获取）
     *
     * @return BOOL
     */
    BOOL setTrayIcon(int index = 0)
    {
        try
        {
            // 获取进程可执行文件路径
            CHAR lpFilename[MAX_PATH];
            HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ::GetCurrentProcessId());

            if (hProcess == nullptr)
            {
                return false;
            }

            if (GetModuleFileNameExA(hProcess, NULL, (LPSTR)lpFilename, MAX_PATH) != ERROR_SUCCESS)
            {
                return false;
            }

            // 取出图标
            HICON hNewIcon = NULL; // 声明一个HICON句柄
            hNewIcon = ExtractIconA(GetModuleHandle(NULL), (LPSTR)lpFilename, index);
            if (!hNewIcon)
                return false;
            nid.hIcon = hNewIcon;
            return Shell_NotifyIconA(NIM_MODIFY, &nid);
        }
        HMC_CHECK_CATCH;
        return false;
    }

    /**
     * @brief 启动
     *
     */
    bool start()
    {
        if (___Start_hmc_Tray)
            return false;
        __tray_worker = new std::thread(__openPuppetTrayWindow);
    }

    // 结束托盘
    bool close()
    {
        Shell_NotifyIconA(NIM_DELETE, &nid);
        CloseWindow(PuppetTrayWindowHwnd);

        try
        {
            PostThreadMessage(GetThreadId(__tray_worker->native_handle()), WM_QUIT, NULL, NULL);
            __tray_worker->join();
            delete __tray_worker;
            __tray_worker = nullptr;
        }

        HMC_CHECK_CATCH;
        try
        {
            ___Start_hmc_Tray = false;
            PuppetTrayWindowHwnd = NULL;
            __MenuList.clear();

            // 清空监听
            for (auto &&EVENT : _HMC__EVENT::ON_EVENT)
                EVENT.second.clear();
            for (auto &&EVENT : _HMC__EVENT::ONCE_EVENT)
                EVENT.second.clear();
            _HMC__EVENT::ON_EVENT.clear();
            _HMC__EVENT::ONCE_EVENT.clear();
            Shell_NotifyIconA(NIM_DELETE, &nid); // 然后立即删除该托盘图标
            ZeroMemory(&nid, sizeof(NOTIFYICONDATAA));
        }
        HMC_CHECK_CATCH;

        return false;
    }

    /**
     * @brief 添加按钮
     *
     * @param menuItem
     * @return true
     * @return false
     */
    bool addMenuItem(chMenuItem menuItem)
    {
        for (auto &&Menu : __MenuList)
        {
            if (Menu.id == menuItem.id)
                return false;
        }

        int idkey = (__tray_next_id++);

        menuItem.messageID = idkey;

        __MenuList.push_back(menuItem);

        return true;
    }

    // 修改托盘显示的消息内容
    BOOL setTrayInfo(string trayInfo)
    {
        string subtxt = string();
        try
        {
            if (trayInfo.length() > 255)
            {
                subtxt.append(trayInfo.substr(0, 255));
            }
            else
            {
                subtxt.append(trayInfo.substr(0, trayInfo.length()));
            }

            strncpy_s(nid.szTip, subtxt.c_str(), subtxt.size());
            if (Shell_NotifyIconA(NIM_MODIFY, &nid))
            {
                __hmc_trayInfo.clear();
                __hmc_trayInfo.append(subtxt);
                return true;
            };
        }
        HMC_CHECK_CATCH;
        return false;

        // wcscpy_s(nid.szTip, pszNewInfo);
    }

    LRESULT CALLBACK ___openPuppetTrayWindow_WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam)
    {
        switch (message)
        {
        case WM_CREATE:
            nid.cbSize = sizeof(NOTIFYICONDATA);
            nid.hWnd = hWnd;
            nid.uID = 0;
            nid.uVersion = NOTIFYICON_VERSION;
            nid.uCallbackMessage = WM_SYSICON;
            if (!nid.hIcon)
            {
                nid.hIcon = LoadIcon(NULL, IDI_APPLICATION);
            }
            strncpy_s(nid.szTip, "", sizeof(nid.szTip));
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            Shell_NotifyIconA(NIM_ADD, &nid);
            break;

        case WM_DESTROY:
            Shell_NotifyIconA(NIM_DELETE, &nid);
            PostQuitMessage(0);
            break;

        case WM_SYSICON:
        {
            _HMC__EVENT::emit(lParam);
            // 如果是右键显示右键菜单
            if (lParam == WM_RBUTTONUP)
            {
                POINT curPoint;
                GetCursorPos(&curPoint);
                SetForegroundWindow(hWnd);
                HMENU hMenu = CreatePopupMenu();
                shared_ptr<void> close_hMenu(nullptr, [&](void *)
                                             {if (hMenu != nullptr) {try{DestroyMenu(hMenu);}HMC_CHECK_CATCH;} });

                if (hMenu)
                {
                    for (auto &&Menu : __MenuList)
                    {

                        // 根按钮  且为按钮按钮中无子分类 分割线和选项将忽略子按钮检测
                        if (Menu.rootMenu && Menu.menuList.size() == 0)
                        {
                            if (!Menu.show)
                                continue;
                            // 按钮
                            if (Menu.type == hmc_tray::chMenuType::menu)
                            {
                                InsertMenuA(hMenu, -1, MF_BYPOSITION, Menu.messageID, Menu.name.c_str());
                            }
                            // 分割线
                            else if (Menu.type == hmc_tray::chMenuType::separator)
                            {
                                InsertMenuA(hMenu, -1, MF_BYPOSITION | MF_SEPARATOR, Menu.messageID, Menu.name.c_str());
                            }
                            // 选项
                            else if (Menu.type == hmc_tray::chMenuType::check)
                            {
                                if (Menu.select)
                                {
                                    InsertMenuA(hMenu, -1, MF_BYCOMMAND | MF_CHECKED, Menu.messageID, Menu.name.c_str());
                                }
                                else
                                {
                                    InsertMenuA(hMenu, -1, MF_BYCOMMAND | MF_UNCHECKED, Menu.messageID, Menu.name.c_str());
                                }
                            }
                            // 选项
                            // else if (Menu.type == hmc_tray::chMenuType::radio)
                            // {
                            //     if (Menu.select)
                            //     {
                            //         InsertMenuA(hMenu, -1, MF_BYCOMMAND | MFT_RADIOCHECK | MF_CHECKED, Menu.messageID, Menu.name.c_str());
                            //     }
                            //     else
                            //     {
                            //         InsertMenuA(hMenu, -1, MF_BYCOMMAND | MFT_RADIOCHECK | MF_UNCHECKED, Menu.messageID, Menu.name.c_str());
                            //     }
                            // }

                            // 禁用
                            if (Menu.disable)
                            {
                                EnableMenuItem(hMenu, Menu.messageID, MF_BYCOMMAND | MF_DISABLED | MF_GRAYED);
                            }
                        }
                        // 有子目录的按钮
                        else if (Menu.show && Menu.rootMenu && Menu.menuList.size() != 0)
                        {
                            InsertMenuA(hMenu, -1, MF_BYPOSITION, Menu.messageID, Menu.name.c_str());
                            HMENU hSubMenu = CreatePopupMenu();
                            shared_ptr<void> close_hSubMenu(nullptr, [&](void *)
                                                            {if (hSubMenu != nullptr) {try{DestroyMenu(hSubMenu);}HMC_CHECK_CATCH;} });
                            if (!Menu.show)
                                continue;

                            for (auto &&IDmessageID : Menu.menuList)
                            {
                                // 创建子菜单
                                if (hSubMenu)
                                {
                                    for (auto &&_Menu : __MenuList)
                                    {
                                        if (_Menu.messageID != IDmessageID)
                                            continue;

                                        // 按钮
                                        if (_Menu.type == hmc_tray::chMenuType::menu)
                                        {
                                            AppendMenuA(hSubMenu, MF_STRING, _Menu.messageID, _Menu.name.c_str());

                                            // InsertMenuA(hMenu, -1, MF_BYPOSITION, _Menu.messageID, _Menu.name.c_str());
                                        }
                                        // 分割线
                                        else if (_Menu.type == hmc_tray::chMenuType::separator)
                                        {
                                            AppendMenuA(hSubMenu, MF_SEPARATOR, _Menu.messageID, _Menu.name.c_str());
                                            // InsertMenuA(hMenu, -1, MF_BYPOSITION | MF_SEPARATOR, _Menu.messageID, _Menu.name.c_str());
                                        }
                                        // 选项
                                        else if (_Menu.type == hmc_tray::chMenuType::check)
                                        {
                                            if (_Menu.select)
                                            {
                                                AppendMenuA(hSubMenu, MF_CHECKED, _Menu.messageID, _Menu.name.c_str());
                                            }
                                            else
                                            {
                                                AppendMenuA(hSubMenu, MF_UNCHECKED, _Menu.messageID, _Menu.name.c_str());
                                            }
                                        }

                                        // 禁用
                                        if (_Menu.disable)
                                        {
                                            EnableMenuItem(hSubMenu, _Menu.messageID, MF_DISABLED | MF_GRAYED);
                                        }
                                    }

                                    // 把子菜单加到主菜单项
                                    ModifyMenuA(hMenu, Menu.messageID, MF_POPUP, (UINT_PTR)hSubMenu, Menu.name.c_str());
                                }
                            }

                            // 这让菜单显示为需要的样子，例如处理具有复选菜单项的菜单
                            SetMenuDefaultItem(hMenu, Menu.messageID, FALSE);
                        }
                    }
                    TrackPopupMenu(hMenu, TPM_BOTTOMALIGN, curPoint.x, curPoint.y, 0, hWnd, NULL);
                    DestroyMenu(hMenu);
                }
            }
        }
        break;

        case WM_COMMAND:
            _HMC__EVENT::clickButtonItem(LOWORD(wParam));
        default:
            return DefWindowProc(hWnd, message, wParam, lParam);
        }

        return 0;
    }

    int __openPuppetTrayWindow()
    {
        WNDCLASSEXA wc;
        MSG Msg;
        HINSTANCE hInstance = (HINSTANCE)GetModuleHandle(NULL);

        wc.cbSize = sizeof(WNDCLASSEXA);
        wc.style = 0;
        wc.lpfnWndProc = ___openPuppetTrayWindow_WndProc;
        wc.cbClsExtra = 0;
        wc.cbWndExtra = 0;
        wc.hInstance = hInstance;
        wc.hIcon = LoadIcon(NULL, IDI_APPLICATION);
        wc.hCursor = LoadCursor(NULL, IDC_ARROW);
        wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
        wc.lpszMenuName = NULL;
        wc.lpszClassName = __hmc_className.c_str();
        wc.hIconSm = LoadIcon(NULL, IDI_APPLICATION);

        RegisterClassExA(&wc);

        PuppetTrayWindowHwnd = CreateWindowExA(WS_EX_APPWINDOW, wc.lpszClassName, __hmc_title.c_str(), WS_OVERLAPPEDWINDOW,
                                               CW_USEDEFAULT, CW_USEDEFAULT, 1, 1, NULL, NULL, hInstance, NULL);

        ShowWindow(PuppetTrayWindowHwnd, SW_HIDE);
        // ShowWindow(hWnd, SW_SHOW);

        while (GetMessageA(&Msg, NULL, 0, 0) > 0)
        {
            TranslateMessage(&Msg);
            DispatchMessage(&Msg);
        }

        return Msg.wParam;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param MenuId
     * @param RootMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(string RootMenuId, string SubMenuId)
    {
        for (auto &&menu : __MenuList)
        {
            if (menu.id == RootMenuId || menu.id == RootMenuId)
            {
                menu.rootMenu = true;
                for (auto &&menu2 : __MenuList)
                {
                    if (menu2.id == SubMenuId || menu2.id == SubMenuId)
                    {
                        menu2.rootMenu = false;
                        menu.menuList.push_back(menu2.messageID);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, string SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId.id, SubMenuId);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, chMenuItem SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId.id, SubMenuId.id);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmetoSubMenu(string RootMenuId, chMenuItem SubMenuId)
    {
        setMenuItmetoSubMenu(RootMenuId, SubMenuId.id);
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    template <typename... Args>
    bool setMenuItmetoSubMenu(string RootMenuId, Args... SubMenuIdArgs)
    {
        string temp[] = {SubMenuIdArgs...};
        for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
        {
            setMenuItmetoSubMenu(RootMenuId, temp[i]);
        }
        return true;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    template <typename... Args>
    bool setMenuItmetoSubMenu(chMenuItem RootMenuId, Args... SubMenuIdArgs)
    {
        string temp[] = {SubMenuIdArgs...};
        for (size_t i = 0; i < sizeof(temp) / sizeof(temp[0]); i++)
        {
            setMenuItmetoSubMenu(RootMenuId.id, temp[i]);
        }
        return true;
    }

    /**
     * @brief 设置指定按钮可见性
     *
     * @param MenuId
     * @param Visible
     * @return true
     * @return false
     */
    bool setMenuItmeVisible(string MenuId, bool Visible)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].show = Visible ? true : false;
                return true;
            }
        }
        return false;
    }

    // 设置按钮为禁用
    bool setMenuItmeEnable(string MenuId, bool Enable = true)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].disable = Enable;
                return true;
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的子按钮
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmeName(string MenuId, string Name)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {

                __MenuList[i].name.clear();
                __MenuList[i].name.append(Name);
                return true;
            }
        }
        return false;
    }

    /**
     * @brief 设置指定的按钮为XXX的选择
     *
     * @param RootMenuId
     * @param SubMenuId
     * @return true
     * @return false
     */
    bool setMenuItmeSelect(string MenuId, bool Select)
    {
        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {
                __MenuList[i].select = Select;
                return true;
            }
        }
        return false;
    }

    /**
     * @brief Get the Menu Itme object
     *
     * @param MenuId
     * @return chMenuItem
     */
    chMenuItem getMenuItme(string MenuId)
    {

        for (size_t i = 0; i < __MenuList.size(); i++)
        {
            auto Menu = __MenuList[i];
            if (Menu.id == MenuId)
            {
                return __MenuList[i];
            }
        }

        return chMenuItem();
    }

    namespace Menu
    {
        /**
         * @brief 创建一个按钮
         *
         * @param name
         * @param id
         * @param disable
         * @return chMenuItem
         */
        chMenuItem menu(string name, string id, bool disable = false)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = disable;
            menuItem.id = id;
            menuItem.name = name;
            menuItem.rootMenu = true;
            menuItem.menuList = {};
            menuItem.show = true;
            menuItem.select = false;
            menuItem.type = hmc_tray::chMenuType::menu;
            return menuItem;
        }

        /**
         * @brief 创建一个带有选项的按钮
         *
         * @param name
         * @param id
         * @param disable
         * @return chMenuItem
         */
        chMenuItem check(string name, string id, bool select = false)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = false;
            menuItem.id = id;
            menuItem.name = name;
            menuItem.rootMenu = true;
            menuItem.menuList = {};
            menuItem.show = true;
            menuItem.select = select;
            menuItem.type = hmc_tray::chMenuType::check;
            return menuItem;
        }

        /**
         * @brief 创建一个分割线按钮
         *
         * @param id
         * @param root
         * @return chMenuItem
         */
        chMenuItem separator(string id, bool root = true)
        {
            hmc_tray::chMenuItem menuItem;
            menuItem.disable = false;
            menuItem.id = id;
            menuItem.name = "";
            menuItem.show = true;
            menuItem.rootMenu = root;
            menuItem.menuList = {};
            menuItem.select = false;
            menuItem.type = hmc_tray::chMenuType::separator;
            return menuItem;
        }

        // /**
        //  * @brief 创建一个带有选项的按钮
        //  *
        //  * @param name
        //  * @param id
        //  * @param disable
        //  * @return chMenuItem
        //  */
        // chMenuItem radio(string name, string id, bool select = false)
        // {
        //     hmc_tray::chMenuItem menuItem;
        //     menuItem.disable = false;
        //     menuItem.id = id;
        //     menuItem.name = name;
        //     menuItem.rootMenu = true;
        //     menuItem.menuList = {};
        //     menuItem.show = true;
        //     menuItem.select = select;
        //     menuItem.type = hmc_tray::chMenuType::radio;
        //     return menuItem;
        // }

    };

}
#endif
