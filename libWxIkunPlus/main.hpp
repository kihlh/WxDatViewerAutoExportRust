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
IMPORT_DLL bool  _createMutex(string MutexName);
IMPORT_DLL bool  _removeMutex(string MutexName);
IMPORT_DLL bool  _hasMutex(string MutexName);
IMPORT_DLL bool  _setStartup();
IMPORT_DLL void  _openSelectFolder();
IMPORT_DLL bool  _setWindowsTop(long hwnds, bool visible);
IMPORT_DLL bool _setCloseWindow(long hwnds, bool closeRoot);