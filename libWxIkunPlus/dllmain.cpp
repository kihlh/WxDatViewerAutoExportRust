#include "pch.h"
#include "./main.hpp"

using namespace std;


#ifdef IMPORT_DLL
#else
#define IMPORT_DLL extern "C" _declspec(dllimport) 
#endif
#define HMC_CHECK_CATCH catch (char *err){};



BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved)
{
   
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
    case DLL_PROCESS_DETACH:
        break;
    }
    return TRUE;
}
