#include <string>
#include <windows.h>
#include <Windows.h>
#include <vector>
#include <iostream>
#include <fstream>
#include <chrono>
#include <time.h>
#include <objbase.h>
#include <ShlObj.h>
#include <Psapi.h>
#include <Shellapi.h>

#include <map>
#include <thread>

#pragma comment(lib, "dwmapi.lib")
#pragma comment(lib, "psapi.lib")
using namespace std;


#ifdef IMPORT_DLLa
#else
#define IMPORT_DLL extern "C" _declspec(dllimport) 
#endif




// dllmain.cpp
IMPORT_DLL void  _setWinIcon(long hwnds);
IMPORT_DLL bool  _setShowWindows(long hwnds, bool visible);
IMPORT_DLL void  _set_tray();
IMPORT_DLL bool  _createMutex(const char* MutexName);
IMPORT_DLL bool  _removeMutex(const char* MutexName);
IMPORT_DLL bool  _hasMutex(const char* MutexName);
IMPORT_DLL bool  _setStartup();
IMPORT_DLL void  _openSelectFolder();
IMPORT_DLL bool  _setWindowsTop(long hwnds, bool visible);
IMPORT_DLL bool _setCloseWindow(long hwnds, bool closeRoot);
IMPORT_DLL const char* _openSelectFolder2();
IMPORT_DLL void _Error(const char* title, const char* info);
IMPORT_DLL void _Stop(const char* title, const char* info);
IMPORT_DLL bool _Confirm(const char* title, const char* info);
IMPORT_DLL bool _Alert(const char* title, const char* info);
IMPORT_DLL const char* _getRegistrValue(long hKey, const char* _subKey, const char* _key);
IMPORT_DLL bool _hasWeChat();
IMPORT_DLL void _setTaskbarWin(long hwnds);
IMPORT_DLL long _findWindow(const char* className, const char* title);
IMPORT_DLL long _findWindowW(const char* className, const char* title);
IMPORT_DLL long _findWindowU8(const char* className, const char* title);
IMPORT_DLL bool _hasStartup();
IMPORT_DLL bool _has_auto_sync();
IMPORT_DLL void _set_auto_sync(bool value);
IMPORT_DLL bool _has_sync_token();
IMPORT_DLL bool _hasStartupGlobalVar();
IMPORT_DLL long _getFocusWindow();
IMPORT_DLL long _getFocusTopWindow();
IMPORT_DLL bool _setMinWindows(long hwnds);
IMPORT_DLL const char* _findAllWindow(const char* className, const char* title);
IMPORT_DLL void _setWinIconMain(long hwnds);
IMPORT_DLL bool _isWindow(long hwnds);
IMPORT_DLL void _setWindowShake(long hwnds