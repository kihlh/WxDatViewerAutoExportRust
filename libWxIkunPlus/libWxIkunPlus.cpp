#include "pch.h"
#include "./main.hpp"
#include <Psapi.h>
#include "./tray.hpp"
#include "./registr.hpp"

#define HMC_CHECK_CATCH catch (char *err){};


namespace Mutex
{
    map<string, HANDLE> AllMutexHandles;
    /**
     * @brief 创建互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool create(string MutexName)
    {
        bool has_mut_exist = false;

        HANDLE hMutex = CreateMutexA(NULL, FALSE, MutexName.c_str());

        AllMutexHandles.insert(pair<string, HANDLE>(MutexName, hMutex));

        if (hMutex == NULL)
        {
            has_mut_exist = true;
        }
        // 检查是否已经存在同名的互斥体
        if (GetLastError() == ERROR_ALREADY_EXISTS)
        {
            has_mut_exist = true;
            CloseHandle(hMutex);
        }

        return !has_mut_exist;
    }

    /**
     * @brief 判断是否有这个互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool has(string MutexName)
    {
        bool has_mut_exist = true;

        HANDLE hMutex;

        hMutex = OpenMutexA(MUTEX_ALL_ACCESS, FALSE, MutexName.c_str());
        if (NULL == hMutex)
        {
            has_mut_exist = false;
        }

        CloseHandle(hMutex);
        return has_mut_exist;
    }

    /**
     * @brief 删除通过此方法创建的互斥体
     *
     * @param MutexName
     * @return true
     * @return false
     */
    bool remove(string MutexName)
    {

        auto it = AllMutexHandles.find(MutexName);
        if (it == AllMutexHandles.end())
        {
            return false;
        }
        while (it != AllMutexHandles.end())
        {
            CloseHandle(it->second);
            it++;
        }

        if (!has(MutexName))
        {
            AllMutexHandles.erase(MutexName);
            return true;
        }
        return false;
    }

    /**
     * @brief 获取当前已经创建的互斥体内容
     *
     * @return vector<string>
     */
    vector<string> list()
    {
        vector<string> list;
        map<string, HANDLE>::iterator it = AllMutexHandles.begin();

        while (it != AllMutexHandles.end())
        {
            list.push_back(it->first);
            it++;
        }
        return list;
    }
}

// 获取进程可执行文件路径
string getProcessidFilePath(int ProcessID)
{
    string Run_lpFilename = "";
    HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, ProcessID);
    char lpFilename[1024];
    if (hProcess == nullptr)
    {
        CloseHandle(hProcess);
        return Run_lpFilename;
    }
    GetModuleFileNameExA(hProcess, NULL, (LPSTR)lpFilename, 1024);
    CloseHandle(hProcess);
    return string(lpFilename);
}


/**
 * @brief 设置窗口的顶设状态
 *
 * @param hwnd
 * @return true
 * @return false
 */
bool setWindowTop(HWND hwnd, bool isWindowTop)
{
    bool result = false;
    try
    {
        RECT rect;
        if (!::GetWindowRect(hwnd, &rect))
            return result;

        if (::GetWindowLong(hwnd, GWL_EXSTYLE) & WS_EX_TOPMOST)
        {
            result = ::SetWindowPos(hwnd, HWND_NOTOPMOST, rect.left, rect.top, abs(rect.right - rect.left), abs(rect.bottom - rect.top), SWP_SHOWWINDOW);
        }
        else
            result = ::SetWindowPos(hwnd, HWND_TOPMOST, rect.left, rect.top, abs(rect.right - rect.left), abs(rect.bottom - rect.top), SWP_SHOWWINDOW);
    }
    HMC_CHECK_CATCH;
    return result;
}


/**
 * @brief 设置窗口图标为指定的icon文件
 *
 * @param hwnd
 * @param iconStr
 * @param index
 * @param titleIcon
 * @param Icon
 * @return true
 * @return false
 */
bool setWindowIcon(HWND hwnd, string iconStr, int index, bool titleIcon = true, bool Icon = true)
{
    bool result = false;
    try
    {

        HICON hIcon;
        hIcon = (HICON)ExtractIconA(NULL, iconStr.c_str(), index);
        HINSTANCE hIn = NULL;
        hIn = ::LoadLibraryA("user32.dll");
        if (hIn)
        {
            LRESULT(WINAPI * SendMessageA)
            (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
            SendMessageA = (LRESULT(WINAPI *)(HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam))GetProcAddress(hIn, "SendMessageA");
            if (SendMessageA)
            {
                if (titleIcon)
                    SendMessageA(hwnd, WM_SETICON, ICON_SMALL, (LPARAM)hIcon);
                if (Icon)
                    SendMessageA(hwnd, WM_SETICON, ICON_BIG, (LPARAM)hIcon);
            }
        }
    }
    HMC_CHECK_CATCH;
    return result;
}

HWND winmian = NULL;

void _setWinIcon(long hwnds)
{

    if (IsWindow(winmian))
    {
        return;
    }
    winmian = (HWND)hwnds;
    //setWindowTop(winmian,true);
    string execPath = getProcessidFilePath(_getpid());
    setWindowIcon(winmian, execPath, 0);
}

void  _set_tray()
{
    string execPath = getProcessidFilePath(_getpid());
    setWindowIcon(winmian, execPath, 0);
    hmc_tray::start();
    hmc_tray::setTrayIcon(execPath, 0);

    //hmc_tray::addMenuItem(hmc_tray::Menu::check("自动同步", "btn::auto_sync", true));
    //hmc_tray::addMenuItem(hmc_tray::Menu::separator("btn::separator::01"));

    hmc_tray::addMenuItem(hmc_tray::Menu::menu("退出程序", "btn::quit_app"));

    hmc_tray::on("click", []()
                 {
            //_putenv("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed=true");
            _putenv_s("K9V7OKIIMR1E1_theInitializationWindowIsDisplayed", "true");
                     if (IsWindowVisible(winmian))
                     {
                         ShowWindow(winmian, 0);
                     }
                     else
                     {
                         ShowWindow(winmian, SW_RESTORE);
                         SetFocus(winmian);
                         SetActiveWindow(winmian);
                         SetForegroundWindow(winmian);
                     }
                 });
                 
    hmc_tray::on("btn::auto_sync", []() {
       
       if (hmc_tray::getMenuItme("btn::auto_sync").select) {
           _putenv_s("ikun_user_auto_disable_sync", "true");

       }
       else {
           _putenv_s("ikun_user_auto_disable_sync", "false");
       }
        
        });
   
    hmc_tray::once("btn::quit_app", []()
                   {
        hmc_tray::close();

        exit(0);
        
        });
}

bool  _setCloseWindow(long hwnds,bool force)
{
    if (force) {
        CloseHandle((HWND)hwnds);
        DestroyWindow((HWND)hwnds);

    }
    return CloseWindow((HWND)hwnds);
}


bool  _setShowWindows(long hwnds, bool visible)
{
    HWND hwnd = (HWND)hwnds;
    ShowWindow(hwnd, visible ? SW_RESTORE : 0);
    if (visible) {
        SetActiveWindow(hwnd);
        SetForegroundWindow(hwnd);
    }

    return true;
}

bool  _setWindowsTop(long hwnds, bool visible)
{
    return setWindowTop((HWND)hwnds, visible);
}

bool  _createMutex(string MutexName) {
    return Mutex::create(MutexName);

}

bool  _hasMutex(string MutexName) {
    return Mutex::has(MutexName);

}

bool  _removeMutex(string MutexName) {
    return Mutex::remove(MutexName);

}
bool _setStartup() {
    string path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    string key = "IkunWxExportDat";
    string execPath = "\"";
    execPath+= getProcessidFilePath(_getpid());
    execPath.append("\" -startup");
   
    if (hmc_registr::hasRegistrKey(HKEY_LOCAL_MACHINE, path,key)) {
        return hmc_registr::removeRegistrValue(HKEY_LOCAL_MACHINE, path, key)?false:true;
    }
    else {
        return hmc_registr::setRegistrValue(HKEY_LOCAL_MACHINE, path, key, execPath) ? true : false;
    }

    return false;
}

bool _hasStartup() {
    string path = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
    string key = "IkunWxExportDat";
    return hmc_registr::hasRegistrKey(HKEY_LOCAL_MACHINE, path, key);
}

void _openSelectFolder() {
    //setWindowTop(winmian, false);

    CoInitialize(NULL);

    BROWSEINFOA browseInfo = { 0 };
    char folderPath[MAX_PATH];

    browseInfo.hwndOwner = NULL;
    browseInfo.pidlRoot = NULL;
    browseInfo.pszDisplayName = folderPath;
    browseInfo.lpszTitle = "选择文件夹";
    browseInfo.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;

    LPITEMIDLIST pidl = SHBrowseForFolderA(&browseInfo);
    _putenv_s("IKUN@SelectedFolderPath", "\0");

    if (pidl != NULL) {
        SHGetPathFromIDListA(pidl, folderPath);
        std::cout << "Selected folder path: " << folderPath << std::endl;
        _putenv_s("IKUN@SelectedFolderPath", folderPath);
        CoTaskMemFree(pidl);
    }
    else {
        std::cout << "Folder selection canceled." << std::endl;
    }

    // 释放 COM 库
    CoUninitialize();
    //setWindowTop(winmian, true);

    //return string(folderPath);
}

